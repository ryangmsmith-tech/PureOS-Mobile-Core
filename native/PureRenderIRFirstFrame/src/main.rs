use anyhow::{bail, Context, Result};
use bytemuck::{Pod, Zeroable};
use purerenderir_first_frame::PureRenderIr;
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
struct FrameReceipt {
    receipt_version: String,
    ir_version: String,
    object_id: String,
    object_name: String,
    width: u32,
    height: u32,
    triangle_count: usize,
    non_background_pixels: usize,
    geometry_sha256: String,
    frame_sha256: String,
    adapter: AdapterReport,
    render_execution: String,
    native_vulkan_offscreen_frame_produced: bool,
    hardware_gpu_frame_produced: bool,
    window_presented: bool,
    validation: String,
}

fn main() -> Result<()> {
    pollster::block_on(run())
}

async fn run() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let ir_path = args
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("assets/gold_ocean_city_crystal.pureir.json"));
    let output_dir = args
        .get(2)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("dist"));

    fs::create_dir_all(&output_dir)
        .with_context(|| format!("create output directory {}", output_dir.display()))?;

    let ir_bytes = fs::read(&ir_path)
        .with_context(|| format!("read PureRenderIR asset {}", ir_path.display()))?;
    let ir: PureRenderIr = serde_json::from_slice(&ir_bytes).context("parse PureRenderIR JSON")?;
    ir.validate().context("validate PureRenderIR geometry")?;

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
        let normal_options = wgpu::RequestAdapterOptions {
            force_fallback_adapter: false,
            ..fallback_options
        };
        instance
            .request_adapter(&normal_options)
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
                label: Some("PureRenderIR First Frame Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .context("request Vulkan device")?;

    let gpu_vertices: Vec<GpuVertex> = ir
        .vertices
        .iter()
        .map(|vertex| GpuVertex {
            position: vertex.position,
            color: vertex.color,
        })
        .collect();

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("PureRenderIR Vertex Buffer"),
        contents: bytemuck::cast_slice(&gpu_vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("PureRenderIR Index Buffer"),
        contents: bytemuck::cast_slice(&ir.indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("PureRenderIR Gold Ocean City Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/gold_ocean_city.wgsl").into()),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("PureRenderIR Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let texture_format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("PureRenderIR Offscreen Pipeline"),
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
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    let extent = wgpu::Extent3d {
        width: ir.width,
        height: ir.height,
        depth_or_array_layers: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("PureRenderIR Offscreen Texture"),
        size: extent,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: texture_format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let unpadded_bytes_per_row = ir.width * BYTES_PER_PIXEL;
    let padded_bytes_per_row = align_up(unpadded_bytes_per_row, COPY_BYTES_PER_ROW_ALIGNMENT);
    let output_buffer_size = u64::from(padded_bytes_per_row) * u64::from(ir.height);
    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("PureRenderIR Readback Buffer"),
        size: output_buffer_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("PureRenderIR First Frame Encoder"),
    });
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("PureRenderIR First Frame Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: ir.clear_color[0],
                        g: ir.clear_color[1],
                        b: ir.clear_color[2],
                        a: ir.clear_color[3],
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        render_pass.set_pipeline(&render_pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..ir.indices.len() as u32, 0, 0..1);
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
                rows_per_image: Some(ir.height),
            },
        },
        extent,
    );
    queue.submit(Some(encoder.finish()));

    let buffer_slice = output_buffer.slice(..);
    let (sender, receiver) = std::sync::mpsc::channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        let _ = sender.send(result);
    });
    device.poll(wgpu::Maintain::Wait);
    receiver
        .recv()
        .context("wait for Vulkan readback mapping")?
        .context("map Vulkan readback buffer")?;

    let mapped = buffer_slice.get_mapped_range();
    let mut rgba = vec![0_u8; (ir.width * ir.height * BYTES_PER_PIXEL) as usize];
    for row in 0..ir.height as usize {
        let source_start = row * padded_bytes_per_row as usize;
        let source_end = source_start + unpadded_bytes_per_row as usize;
        let destination_start = row * unpadded_bytes_per_row as usize;
        let destination_end = destination_start + unpadded_bytes_per_row as usize;
        rgba[destination_start..destination_end]
            .copy_from_slice(&mapped[source_start..source_end]);
    }
    drop(mapped);
    output_buffer.unmap();

    let background = rgba
        .get(0..4)
        .context("frame did not contain a complete pixel")?
        .to_vec();
    let non_background_pixels = rgba
        .chunks_exact(4)
        .filter(|pixel| pixel_difference(pixel, &background) > 18)
        .count();
    let minimum_pixels = ((u64::from(ir.width) * u64::from(ir.height)) / 40) as usize;
    if non_background_pixels < minimum_pixels {
        bail!(
            "frame validation failed: only {non_background_pixels} non-background pixels, expected at least {minimum_pixels}"
        );
    }

    let frame_path = output_dir.join("gold_ocean_city_first_vulkan_frame.png");
    image::save_buffer_with_format(
        &frame_path,
        &rgba,
        ir.width,
        ir.height,
        image::ColorType::Rgba8,
        image::ImageFormat::Png,
    )
    .with_context(|| format!("save frame {}", frame_path.display()))?;

    let frame_bytes = fs::read(&frame_path).context("read generated PNG for receipt")?;
    let geometry_sha256 = sha256_hex(&ir_bytes);
    let frame_sha256 = sha256_hex(&frame_bytes);
    let triangle_count = ir.indices.len() / 3;

    let receipt = FrameReceipt {
        receipt_version: "1.0".to_owned(),
        ir_version: ir.ir_version.clone(),
        object_id: ir.object_id.clone(),
        object_name: ir.object_name.clone(),
        width: ir.width,
        height: ir.height,
        triangle_count,
        non_background_pixels,
        geometry_sha256,
        frame_sha256,
        hardware_gpu_frame_produced: adapter_report.hardware_accelerated,
        adapter: adapter_report,
        render_execution: "wgpu_vulkan_offscreen".to_owned(),
        native_vulkan_offscreen_frame_produced: true,
        window_presented: false,
        validation: "passed".to_owned(),
    };

    write_json(&output_dir.join("frame_receipt.json"), &receipt)?;
    write_json(&output_dir.join("adapter_report.json"), &receipt.adapter)?;

    println!("PureRenderIR first Vulkan frame: PASSED");
    println!("Object: {} ({})", ir.object_name, ir.object_id);
    println!("Frame: {}", frame_path.display());
    println!("Triangles: {triangle_count}");
    println!("Non-background pixels: {non_background_pixels}");
    println!("Adapter: {} ({})", receipt.adapter.name, receipt.adapter.device_type);
    println!("Hardware accelerated: {}", receipt.hardware_gpu_frame_produced);
    println!("Frame SHA-256: {}", receipt.frame_sha256);

    Ok(())
}

fn align_up(value: u32, alignment: u32) -> u32 {
    ((value + alignment - 1) / alignment) * alignment
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
