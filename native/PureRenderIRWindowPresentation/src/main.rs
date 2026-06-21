mod capture;
mod receipt;
mod renderer;
mod scene;

use anyhow::{Context, Result};
use renderer::RuntimeState;
use scene::SCENE_JSON;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowBuilder;

const AUTOMATION_FRAMES: u32 = 12;

fn main() -> Result<()> {
    let output_dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("dist"));
    let automation = std::env::var("PUREOS_AUTOMATION")
        .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    let event_loop = EventLoop::new().context("create desktop event loop")?;
    let window_title = "Pure OS — Gold Ocean City Native Window v0.11";
    let window = Arc::new(
        WindowBuilder::new()
            .with_title(window_title)
            .with_inner_size(PhysicalSize::new(960_u32, 540_u32))
            .with_min_inner_size(PhysicalSize::new(640_u32, 360_u32))
            .build(&event_loop)
            .context("create native desktop window")?,
    );
    let scene_ir_sha256 = format!("{:x}", Sha256::digest(SCENE_JSON.as_bytes()));
    let mut state = pollster::block_on(RuntimeState::new(
        window.clone(),
        output_dir,
        automation,
        scene_ir_sha256,
    ))?;

    event_loop.set_control_flow(ControlFlow::Poll);
    window.request_redraw();

    event_loop
        .run(move |event, target| match event {
            Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => target.exit(),
                WindowEvent::Resized(size) => state.resize(size),
                WindowEvent::KeyboardInput { event, .. }
                    if event.state == ElementState::Pressed && !event.repeat =>
                {
                    match event.physical_key {
                        PhysicalKey::Code(KeyCode::Escape) => target.exit(),
                        PhysicalKey::Code(KeyCode::KeyW)
                        | PhysicalKey::Code(KeyCode::ArrowUp) => state.move_camera(0.0, 0.45),
                        PhysicalKey::Code(KeyCode::KeyS)
                        | PhysicalKey::Code(KeyCode::ArrowDown) => {
                            state.move_camera(0.0, -0.45)
                        }
                        PhysicalKey::Code(KeyCode::KeyA)
                        | PhysicalKey::Code(KeyCode::ArrowLeft) => {
                            state.move_camera(-0.45, 0.0)
                        }
                        PhysicalKey::Code(KeyCode::KeyD)
                        | PhysicalKey::Code(KeyCode::ArrowRight) => {
                            state.move_camera(0.45, 0.0)
                        }
                        PhysicalKey::Code(KeyCode::KeyQ) => state.zoom_camera(1.08),
                        PhysicalKey::Code(KeyCode::KeyE) => state.zoom_camera(0.92),
                        _ => {}
                    }
                }
                WindowEvent::RedrawRequested => {
                    if state.automation {
                        state.scripted_camera_step();
                    }
                    match state.render_surface() {
                        Ok(()) => {
                            if state.automation
                                && state.surface_frames_presented >= AUTOMATION_FRAMES
                            {
                                match state.capture_and_write_receipt(window_title) {
                                    Ok(()) => target.exit(),
                                    Err(error) => {
                                        eprintln!("window capture failed: {error:#}");
                                        target.exit();
                                    }
                                }
                            } else {
                                window.request_redraw();
                            }
                        }
                        Err(error) => {
                            eprintln!("surface render failed: {error:#}");
                            state.surface.configure(&state.device, &state.config);
                            window.request_redraw();
                        }
                    }
                }
                _ => {}
            },
            Event::AboutToWait => {
                if !state.automation && state.last_frame_time.elapsed().as_millis() >= 16 {
                    window.request_redraw();
                }
            }
            _ => {}
        })
        .context("run desktop event loop")?;

    Ok(())
}
