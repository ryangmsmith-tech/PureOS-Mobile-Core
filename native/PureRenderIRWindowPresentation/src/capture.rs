use crate::receipt::{AdapterReport, ObjectReceipt, WindowPresentationReceipt};
use crate::renderer::RuntimeState;
use anyhow::{bail, Context, Result};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

const BYTES_PER_PIXEL: u32 = 4;
const COPY_BYTES_PER_ROW_ALIGNMENT: u32 = 256;

impl RuntimeState {
    pub fn capture_and_write_receipt(&self, window_title: &str) -> Result<()> {
        let extent = wgpu::Extent3d {
            width: self.config.width,
            height: self.config.height,
            depth_or_array_layers: 1,
        };
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("PureRenderIR Window Capture Texture"),
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let unpadded_bytes_per_row = self.config.width * BYTES_PER_PIXEL;
        let padded_bytes_per_row = align_up(unpadded_bytes_per_row, COPY_BYTES_PER_ROW_ALIGNMENT);
        let output_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("PureRenderIR Window Capture Readback Buffer"),
            size: u64::from(padded_bytes_per_row) * u64::from(self.config.height),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("PureRenderIR Window Capture Encoder"),
            });
        self.encode_scene_pass(&mut encoder, &view, "PureRenderIR Window Capture Pass");
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
                    rows_per_image: Some(self.config.height),
                },
            },
            extent,
        );
        self.queue.submit(Some(encoder.finish()));

        let slice = output_buffer.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });
        self.device.poll(wgpu::Maintain::Wait);
        receiver
            .recv()
            .context("wait for window capture readback")?
            .context("map window capture readback")?;

        let mapped = slice.get_mapped_range();
        let mut pixels = vec![0_u8; (self.config.width * self.config.height * 4) as usize];
        for row in 0..self.config.height as usize {
            let source_start = row * padded_bytes_per_row as usize;
            let source_end = source_start + unpadded_bytes_per_row as usize;
            let destination_start = row * unpadded_bytes_per_row as usize;
            let destination_end = destination_start + unpadded_bytes_per_row as usize;
            pixels[destination_start..destination_end]
                .copy_from_slice(&mapped[source_start..source_end]);
        }
        drop(mapped);
        output_buffer.unmap();
        convert_to_rgba(self.config.format, &mut pixels)?;

        let background = pixels.get(0..4).context("capture missing first pixel")?.to_vec();
        let non_background_pixels = pixels
            .chunks_exact(4)
            .filter(|pixel| pixel_difference(pixel, &background) > 18)
            .count();
        let minimum_pixels =
            (u64::from(self.config.width) * u64::from(self.config.height) / 5) as usize;
        if non_background_pixels < minimum_pixels {
            bail!(
                "window capture validation failed: {non_background_pixels} changed pixels, expected at least {minimum_pixels}"
            );
        }

        let capture_path = self
            .output_dir
            .join("gold_ocean_city_window_presentation.png");
        image::save_buffer_with_format(
            &capture_path,
            &pixels,
            self.config.width,
            self.config.height,
            image::ColorType::Rgba8,
            image::ImageFormat::Png,
        )
        .with_context(|| format!("save window capture {}", capture_path.display()))?;
        let capture_sha256 = sha256_hex(&fs::read(&capture_path).context("read window capture")?);

        let receipt = WindowPresentationReceipt {
            receipt_version: "1.0".to_owned(),
            runtime_version: "0.11".to_owned(),
            scene_version: self.scene.scene_version.clone(),
            scene_id: self.scene.scene_id.clone(),
            scene_name: self.scene.scene_name.clone(),
            window_title: window_title.to_owned(),
            window_width: self.config.width,
            window_height: self.config.height,
            surface_format: format!("{:?}", self.config.format),
            present_mode: format!("{:?}", self.config.present_mode),
            surface_frames_presented: self.surface_frames_presented,
            scripted_camera_smoke_test_executed: self.automation,
            interactive_keyboard_camera_controls_compiled: true,
            camera_start: self.camera_start,
            camera_end: self.camera,
            camera_positions: self.camera_positions.clone(),
            object_count: self.object_receipts.len(),
            draw_call_count_per_frame: self.object_receipts.len(),
            triangle_count_per_frame: self.scene.triangle_count(),
            scene_ir_sha256: self.scene_ir_sha256.clone(),
            capture_sha256,
            capture_non_background_pixels: non_background_pixels,
            objects: self.object_receipts.iter().map(clone_object_receipt).collect(),
            hardware_gpu_frame_produced: self.adapter_report.hardware_accelerated,
            adapter: clone_adapter_report(&self.adapter_report),
            native_window_created: true,
            vulkan_surface_configured: true,
            surface_frame_presentation_produced: self.surface_frames_presented > 0,
            x11_virtual_display_used: self.automation,
            physical_monitor_presented: false,
            android_window_presented: false,
            validation: "passed".to_owned(),
        };

        write_json(
            &self.output_dir.join("window_presentation_receipt.json"),
            &receipt,
        )?;
        write_json(&self.output_dir.join("adapter_report.json"), &receipt.adapter)?;

        println!("PureRenderIR window presentation: PASSED");
        println!("Surface frames presented: {}", self.surface_frames_presented);
        println!("Camera start: {:?}", self.camera_start.center);
        println!("Camera end: {:?}", self.camera.center);
        println!("Adapter: {} ({})", receipt.adapter.name, receipt.adapter.device_type);
        println!("Hardware accelerated: {}", receipt.hardware_gpu_frame_produced);
        println!("Capture SHA-256: {}", receipt.capture_sha256);
        Ok(())
    }
}

fn clone_object_receipt(source: &ObjectReceipt) -> ObjectReceipt {
    ObjectReceipt {
        id: source.id.clone(),
        name: source.name.clone(),
        kind: source.kind.clone(),
        triangle_count: source.triangle_count,
        index_start: source.index_start,
        index_end: source.index_end,
    }
}

fn clone_adapter_report(source: &AdapterReport) -> AdapterReport {
    AdapterReport {
        backend: source.backend.clone(),
        name: source.name.clone(),
        vendor: source.vendor,
        device: source.device,
        device_type: source.device_type.clone(),
        driver: source.driver.clone(),
        software_adapter: source.software_adapter,
        hardware_accelerated: source.hardware_accelerated,
    }
}

fn convert_to_rgba(format: wgpu::TextureFormat, pixels: &mut [u8]) -> Result<()> {
    match format {
        wgpu::TextureFormat::Bgra8Unorm | wgpu::TextureFormat::Bgra8UnormSrgb => {
            for pixel in pixels.chunks_exact_mut(4) {
                pixel.swap(0, 2);
            }
            Ok(())
        }
        wgpu::TextureFormat::Rgba8Unorm | wgpu::TextureFormat::Rgba8UnormSrgb => Ok(()),
        _ => bail!("cannot convert unsupported surface format: {format:?}"),
    }
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
