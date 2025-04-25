use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::utils::gpu_detector;

/// GPU information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub vendor: String,
    pub is_available: bool,
    pub supported_codecs: Vec<String>,
}

/// Application information including GPU and FFmpeg version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub app_version: String,
    pub ffmpeg_version: Option<String>,
    pub gpu_available: bool,
    pub gpus: Vec<GpuInfo>,
    pub selected_gpu_index: i32, // -1 for CPU, 0+ for GPU
}

/// Get application information
pub fn get_app_info() -> Result<AppInfo, String> {
    // Get FFmpeg version
    let ffmpeg_version = Some("FFmpeg 7.1.0".to_string()); // Replace with actual function
    
    // Check GPU availability
    let gpu_list = match gpu_detector::check_gpu_availability() {
        Ok(list) => list,
        Err(e) => return Err(format!("Failed to detect GPU: {}", e)),
    };
    
    // Default to first available GPU if any, otherwise use CPU
    let selected_gpu_index = if let Some((i, _)) = gpu_list.gpus.iter().enumerate().find(|(_, g)| g.is_available) {
        i as i32
    } else {
        -1 // No available GPU, use CPU
    };
    
    // Convert GPU info
    let gpus: Vec<GpuInfo> = gpu_list.gpus
        .into_iter()
        .map(|gpu| GpuInfo {
            name: gpu.name,
            vendor: gpu.vendor,
            is_available: gpu.is_available,
            supported_codecs: gpu.supported_codecs,
        })
        .collect();
    
    // Create AppInfo
    let app_info = AppInfo {
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        ffmpeg_version,
        gpu_available: gpus.iter().any(|g| g.is_available),
        gpus,
        selected_gpu_index,
    };
    
    Ok(app_info)
}

/// Set selected GPU index
pub fn set_gpu(gpu_index: i32, app_handle: &AppHandle) -> Result<(), String> {
    // Get GPU list
    let gpu_list = match gpu_detector::check_gpu_availability() {
        Ok(list) => list,
        Err(e) => return Err(format!("Failed to detect GPU: {}", e)),
    };
    
    // Validate GPU index
    if gpu_index != -1 && (gpu_index < 0 || gpu_index as usize >= gpu_list.gpus.len()) {
        return Err(format!("Invalid GPU index: {}", gpu_index));
    }
    
    // Get updated app info with new GPU index
    let mut app_info = match get_app_info() {
        Ok(info) => info,
        Err(e) => return Err(format!("Failed to get app info: {}", e)),
    };
    
    // Update selected GPU index
    app_info.selected_gpu_index = gpu_index;
    
    // Emit app-info-changed event
    let _ = app_handle.emit("app-info-changed", app_info);
    
    Ok(())
}
