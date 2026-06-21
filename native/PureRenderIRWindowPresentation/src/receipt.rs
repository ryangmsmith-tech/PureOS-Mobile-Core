use crate::scene::Camera;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AdapterReport {
    pub backend: String,
    pub name: String,
    pub vendor: u32,
    pub device: u32,
    pub device_type: String,
    pub driver: String,
    pub software_adapter: bool,
    pub hardware_accelerated: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ObjectReceipt {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub triangle_count: usize,
    pub index_start: u32,
    pub index_end: u32,
}

#[derive(Debug, Serialize)]
pub struct WindowPresentationReceipt {
    pub receipt_version: String,
    pub runtime_version: String,
    pub scene_version: String,
    pub scene_id: String,
    pub scene_name: String,
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub surface_format: String,
    pub present_mode: String,
    pub surface_frames_presented: u32,
    pub scripted_camera_smoke_test_executed: bool,
    pub interactive_keyboard_camera_controls_compiled: bool,
    pub camera_start: Camera,
    pub camera_end: Camera,
    pub camera_positions: Vec<[f32; 2]>,
    pub object_count: usize,
    pub draw_call_count_per_frame: usize,
    pub triangle_count_per_frame: usize,
    pub scene_ir_sha256: String,
    pub capture_sha256: String,
    pub capture_non_background_pixels: usize,
    pub objects: Vec<ObjectReceipt>,
    pub adapter: AdapterReport,
    pub native_window_created: bool,
    pub vulkan_surface_configured: bool,
    pub surface_frame_presentation_produced: bool,
    pub x11_virtual_display_used: bool,
    pub physical_monitor_presented: bool,
    pub hardware_gpu_frame_produced: bool,
    pub android_window_presented: bool,
    pub validation: String,
}
