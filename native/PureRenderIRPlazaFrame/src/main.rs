use anyhow::{bail, Context, Result};
use bytemuck::{Pod, Zeroable};
use purerenderir_plaza_frame::{Camera, SceneIr};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use wgpu::util::DeviceExt;

const BYTES_PER_PIXEL: u32 = 4;
const COPY_BYTES_PER_ROW_ALIGNMENT: u32 = 256;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct GpuVertex {
    position: [f32; 2],
    color: [f32; 4],
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

#[derive(Debug, Serialize)]
struct AdapterReport {
    backend: String,
    name: String,
    vendor: u32,
    device: u32,
    device_type: String,
    driver: String,
    software_adapter: bool,
    hardware_accelerated: bool,
}

#[derive(Debug, Serialize)]
struct ObjectReceipt {
    id: String,
    name: String,
    kind: String,
    triangle_count: usize,
    index_start: u32,
    index_end: u32,
}

#[derive(Debug, Serialize)]
struct SceneFrameReceipt {
    receipt_version: String,
    scene_version: String,
    scene_id: String,
    scene_name: String,
    width: u32,
    height: u32,
    camera: Camera,
    object_count: usize,
    draw_call_count: usize,
    triangle_count: usize,
    non_background_pixels: usize,
    scene_ir_sha256: String,
    frame_sha256: String,
    objects: Vec<ObjectReceipt>,
    adapter: AdapterReport,
    render_execution: String,
    native_vulkan_offscreen_scene_frame_produced: bool,
    hardware_gpu_frame_produced: bool,
    window_presented: bool,
    interactive_camera_enabled: bool,
    validation: String,
}

fn main() -> Result<()> {
    pollster::block_on(run())
}

async fn run() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let scene_path = args
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("assets/gold_ocean_city_plaza.pureir.scene.json"));
    let output_dir = args
        .get(2)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("dist"));

    fs::create_dir_all(&output_dir)
        .with_context(|| format!("create output directory {}", output_dir.display()))?;

    let scene_bytes = fs::read(&scene_path)
        .with_context(|| format!("read scene IR {}", scene_path.display()))?;
    let scene: SceneIr = serde_json::from_slice(&scene_bytes).context("parse scene IR JSON")?;
    scene.validate().context("validate scene IR")?;

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        ..Default::default()
    });
    let fallback_options = wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
        compatible_surface: None,
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
            .context("no Vulkan adapter was available")?
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
                label: Some("PureRenderIR Plaza Frame Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .context("request Vulkan device")?;

    let mut vertices = Vec::<GpuVertex>::new();
    let mut indices = Vec::<u16>::new();
    let mut object_receipts = Vec::<ObjectReceipt>::new();

    for object in &scene.objects {
        let vertex_base = u16::try_from(vertices.len()).context("scene vertex count exceeds u16")?;
        let index_start = u32::try_from(indices.len()).context("index start exceeds u32")?;

        vertices.extend(object.vertices.iter().map(|vertex| GpuVertex {
            position: scene.project_position(vertex.position),
            color: vertex.color,
        }));
        for index in &object.indices {
            indices.push(
                vertex_base
                    .checked_add(*index)
                    .context("scene index overflow")?,
            );
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

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("PureRenderIR Plaza Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("PureRenderIR Plaza Index Buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("PureRenderIR Plaza Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/plaza.wgsl").into()),
    });
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("PureRenderIR Plaza Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });
    let texture_format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("PureRenderIR Plaza Pipeline"),
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
                format: texture_format,
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

    let extent = wgpu::Extent3d {
        width: scene.width,
        height: scene.height,
        depth_or_array_layers: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("PureRenderIR Plaza Offscreen Texture"),
        size: extent,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: texture_format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let unpadded_bytes_per_row = scene.width * BYTES_PER_PIXEL;
    let padded_bytes_per_row = align_up(unpadded_bytes_per_row, COPY_BYTES_PER_ROW_ALIGNMENT);
    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("PureRenderIR Plaza Readback Buffer"),
        size: u64::from(padded_bytes_per_row) * u64::from(scene.height),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("PureRenderIR Plaza Frame Encoder"),
    });
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("PureRenderIR Plaza Frame Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: scene.clear_color[0],
                        g: scene.clear_color[1],
                        b: scene.clear_color[2],
                        a: scene.clear_color[3],
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        for object in &object_receipts {
            pass.draw_indexed(object.index_start..object.index_end, 0, 0..1);
        }
    }

    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::ImageCopyBuffer {
            buffer: &output_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(padded_bytes_per_row),
                rows_per_image: Some(scene.height),
            },
        },
        extent,
    );
    queue.submit(Some(encoder.finish()));

    let slice = output_buffer.slice(..);
    let (sender, receiver) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |result| {
        let _ = sender.send(result);
    });
    device.poll(wgpu::Maintain::Wait);
    receiver
        .recv()
        .context("wait for Vulkan readback")?
        .context("map Vulkan readback buffer")?;

    let mapped = slice.get_mapped_range();
    let mut rgba = vec![0_u8; (scene.width * scene.height * BYTES_PER_PIXEL) as usize];
    for row in 0..scene.height as usize {
        let source_start = row * padded_bytes_per_row as usize;
        let source_end = source_start + unpadded_bytes_per_row as usize;
        let destination_start = row * unpadded_bytes_per_row as usize;
        let destination_end = destination_start + unpadded_bytes_per_row as usize;
        rgba[destination_start..destination_end]
            .copy_from_slice(&mapped[source_start..source_end]);
    }
    drop(mapped);
    output_buffer.unmap();

    let background = rgba.get(0..4).context("frame missing first pixel")?.to_vec();
    let non_background_pixels = rgba
        .chunks_exact(4)
        .filter(|pixel| pixel_difference(pixel, &background) > 18)
        .count();
    let minimum_pixels = (u64::from(scene.width) * u64::from(scene.height) / 5) as usize;
    if non_background_pixels < minimum_pixels {
        bail!(
            "scene validation failed: {non_background_pixels} changed pixels, expected at least {minimum_pixels}"
        );
    }

    let frame_path = output_dir.join("gold_ocean_city_plaza_vulkan_frame.png");
    image::save_buffer_with_format(
        &frame_path,
        &rgba,
        scene.width,
        scene.height,
        image::ColorType::Rgba8,
        image::ImageFormat::Png,
    )
    .with_context(|| format!("save frame {}", frame_path.display()))?;

    let frame_bytes = fs::read(&frame_path).context("read generated PNG")?;
    let frame_sha256 = sha256_hex(&frame_bytes);
    let scene_ir_sha256 = sha256_hex(&scene_bytes);
    let object_count = object_receipts.len();
    let triangle_count = scene.triangle_count();

    let receipt = SceneFrameReceipt {
        receipt_version: "1.0".to_owned(),
        scene_version: scene.scene_version.clone(),
        scene_id: scene.scene_id.clone(),
        scene_name: scene.scene_name.clone(),
        width: scene.width,
        height: scene.height,
        camera: scene.camera,
        object_count,
        draw_call_count: object_count,
        triangle_count,
        non_background_pixels,
        scene_ir_sha256,
        frame_sha256,
        objects: object_receipts,
        hardware_gpu_frame_produced: adapter_report.hardware_accelerated,
        adapter: adapter_report,
        render_execution: "wgpu_vulkan_offscreen_multi_object_scene".to_owned(),
        native_vulkan_offscreen_scene_frame_produced: true,
        window_presented: false,
        interactive_camera_enabled: false,
        validation: "passed".to_owned(),
    };

    write_json(&output_dir.join("scene_frame_receipt.json"), &receipt)?;
    write_json(&output_dir.join("adapter_report.json"), &receipt.adapter)?;

    println!("PureRenderIR Gold Ocean City plaza frame: PASSED");
    println!("Scene: {} ({})", scene.scene_name, scene.scene_id);
    println!("Objects / draw calls: {object_count}");
    println!("Triangles: {triangle_count}");
    println!("Non-background pixels: {non_background_pixels}");
    println!("Adapter: {} ({})", receipt.adapter.name, receipt.adapter.device_type);
    println!("Hardware accelerated: {}", receipt.hardware_gpu_frame_produced);
    println!("Frame SHA-256: {}", receipt.frame_sha256);
    Ok(())
}

fn align_up(value: u32, alignment: u32) -> u32 {
    value.div_ceil(alignment) * alignment
}

fn pixel_difference(pixel: &[u8], background: &[u8]) -> u32 {
    pixel
        .iter()
        .zip(background.iter())
        .take(3)
        .map(|(left, right)| u32::from(left.abs_diff(*right)))
        .sum()
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(value).context("serialize JSON evidence")?;
    fs::write(path, bytes).with_context(|| format!("write {}", path.display()))
}
