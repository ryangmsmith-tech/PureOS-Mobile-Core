use crate::receipt::{AdapterReport, ObjectReceipt};
use crate::scene::{Camera, SceneIr};
use anyhow::{bail, Context, Result};
use bytemuck::{Pod, Zeroable};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::window::Window;

const SHADER: &str = include_str!("../shaders/window_scene.wgsl");

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub(crate) struct GpuVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl GpuVertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4];

    fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WorldVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

pub(crate) struct RuntimeState {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: PhysicalSize<u32>,
    pub pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub world_vertices: Vec<WorldVertex>,
    pub object_receipts: Vec<ObjectReceipt>,
    pub camera: Camera,
    pub camera_start: Camera,
    pub clear_color: [f64; 4],
    pub adapter_report: AdapterReport,
    pub surface_frames_presented: u32,
    pub camera_positions: Vec<[f32; 2]>,
    pub output_dir: PathBuf,
    pub scene: SceneIr,
    pub scene_ir_sha256: String,
    pub automation: bool,
    pub last_frame_time: Instant,
}

impl RuntimeState {
    pub async fn new(
        window: Arc<Window>,
        output_dir: PathBuf,
        automation: bool,
        scene_ir_sha256: String,
    ) -> Result<Self> {
        let scene = SceneIr::embedded()?;
        std::fs::create_dir_all(&output_dir)
            .with_context(|| format!("create output directory {}", output_dir.display()))?;

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            ..Default::default()
        });
        let surface = instance
            .create_surface(window.clone())
            .context("create Vulkan window surface")?;

        let fallback_options = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: Some(&surface),
            force_fallback_adapter: true,
        };
        let adapter = if let Some(adapter) = instance.request_adapter(&fallback_options).await {
            adapter
        } else {
            instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    force_fallback_adapter: false,
                    ..fallback_options
                })
                .await
                .context("no Vulkan adapter supports the window surface")?
        };

        let info = adapter.get_info();
        let software_adapter = matches!(info.device_type, wgpu::DeviceType::Cpu);
        let hardware_accelerated = matches!(
            info.device_type,
            wgpu::DeviceType::DiscreteGpu
                | wgpu::DeviceType::IntegratedGpu
                | wgpu::DeviceType::VirtualGpu
        );
        let adapter_report = AdapterReport {
            backend: format!("{:?}", info.backend),
            name: info.name,
            vendor: info.vendor,
            device: info.device,
            device_type: format!("{:?}", info.device_type),
            driver: info.driver,
            software_adapter,
            hardware_accelerated,
        };

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("PureRenderIR Window Presentation Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_defaults(),
                },
                None,
            )
            .await
            .context("request Vulkan presentation device")?;

        let capabilities = surface.get_capabilities(&adapter);
        let surface_format = [
            wgpu::TextureFormat::Bgra8UnormSrgb,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            wgpu::TextureFormat::Bgra8Unorm,
            wgpu::TextureFormat::Rgba8Unorm,
        ]
        .into_iter()
        .find(|candidate| capabilities.formats.contains(candidate))
        .or_else(|| capabilities.formats.first().copied())
        .context("surface reported no formats")?;
        ensure_four_byte_surface_format(surface_format)?;

        let present_mode = if capabilities.present_modes.contains(&wgpu::PresentMode::Fifo) {
            wgpu::PresentMode::Fifo
        } else {
            *capabilities
                .present_modes
                .first()
                .context("surface reported no presentation modes")?
        };
        let alpha_mode = *capabilities
            .alpha_modes
            .first()
            .context("surface reported no alpha modes")?;

        let size = window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode,
            alpha_mode,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let mut world_vertices = Vec::new();
        let mut indices = Vec::new();
        let mut object_receipts = Vec::new();
        for object in &scene.objects {
            let vertex_base = u16::try_from(world_vertices.len()).context("vertex count exceeds u16")?;
            let index_start = u32::try_from(indices.len()).context("index start exceeds u32")?;
            world_vertices.extend(object.vertices.iter().map(|vertex| WorldVertex {
                position: vertex.position,
                color: vertex.color,
            }));
            for index in &object.indices {
                indices.push(vertex_base.checked_add(*index).context("scene index overflow")?);
            }
            let index_end = u32::try_from(indices.len()).context("index end exceeds u32")?;
            object_receipts.push(ObjectReceipt {
                id: object.id.clone(),
                name: object.name.clone(),
                kind: object.kind.clone(),
                triangle_count: object.indices.len() / 3,
                index_start,
                index_end,
            });
        }

        let camera_start = scene.camera;
        let initial_vertices = project_vertices(&world_vertices, camera_start, config.width, config.height);
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("PureRenderIR Window Vertex Buffer"),
            contents: bytemuck::cast_slice(&initial_vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("PureRenderIR Window Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("PureRenderIR Window Scene Shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("PureRenderIR Window Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("PureRenderIR Window Presentation Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[GpuVertex::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
            pipeline,
            vertex_buffer,
            index_buffer,
            world_vertices,
            object_receipts,
            camera: camera_start,
            camera_start,
            clear_color: scene.clear_color,
            adapter_report,
            surface_frames_presented: 0,
            camera_positions: vec![camera_start.center],
            output_dir,
            scene,
            scene_ir_sha256,
            automation,
            last_frame_time: Instant::now(),
        })
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return;
        }
        self.size = size;
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
        self.update_vertices();
    }

    pub fn move_camera(&mut self, dx: f32, dy: f32) {
        self.camera.center[0] += dx;
        self.camera.center[1] += dy;
        self.camera_positions.push(self.camera.center);
        self.update_vertices();
    }

    pub fn zoom_camera(&mut self, scale: f32) {
        self.camera.half_height = (self.camera.half_height * scale).clamp(3.0, 18.0);
        self.camera_positions.push(self.camera.center);
        self.update_vertices();
    }

    pub fn scripted_camera_step(&mut self) {
        let target = match self.surface_frames_presented {
            0..=2 => [-1.8, -0.4],
            3..=5 => [1.4, 0.6],
            6..=8 => [2.8, -0.8],
            _ => [0.6, 0.2],
        };
        self.camera.center[0] += (target[0] - self.camera.center[0]) * 0.32;
        self.camera.center[1] += (target[1] - self.camera.center[1]) * 0.32;
        self.camera.half_height = 8.6;
        self.camera_positions.push(self.camera.center);
        self.update_vertices();
    }

    pub fn render_surface(&mut self) -> Result<()> {
        let frame = self
            .surface
            .get_current_texture()
            .context("acquire Vulkan surface frame")?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("PureRenderIR Surface Frame Encoder"),
            });
        self.encode_scene_pass(&mut encoder, &view, "PureRenderIR Surface Render Pass");
        self.queue.submit(Some(encoder.finish()));
        frame.present();
        self.surface_frames_presented += 1;
        self.last_frame_time = Instant::now();
        Ok(())
    }

    pub fn encode_scene_pass(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        label: &'static str,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(label),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: self.clear_color[0],
                        g: self.clear_color[1],
                        b: self.clear_color[2],
                        a: self.clear_color[3],
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        for object in &self.object_receipts {
            pass.draw_indexed(object.index_start..object.index_end, 0, 0..1);
        }
    }

    fn update_vertices(&mut self) {
        let projected = project_vertices(
            &self.world_vertices,
            self.camera,
            self.config.width,
            self.config.height,
        );
        self.queue
            .write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&projected));
    }
}

fn project_vertices(
    vertices: &[WorldVertex],
    camera: Camera,
    width: u32,
    height: u32,
) -> Vec<GpuVertex> {
    let aspect = width.max(1) as f32 / height.max(1) as f32;
    vertices
        .iter()
        .map(|vertex| GpuVertex {
            position: [
                (vertex.position[0] - camera.center[0]) / (camera.half_height * aspect),
                (vertex.position[1] - camera.center[1]) / camera.half_height,
            ],
            color: vertex.color,
        })
        .collect()
}

fn ensure_four_byte_surface_format(format: wgpu::TextureFormat) -> Result<()> {
    match format {
        wgpu::TextureFormat::Bgra8Unorm
        | wgpu::TextureFormat::Bgra8UnormSrgb
        | wgpu::TextureFormat::Rgba8Unorm
        | wgpu::TextureFormat::Rgba8UnormSrgb => Ok(()),
        _ => bail!("unsupported surface format for capture: {format:?}"),
    }
}
