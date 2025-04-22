//! # Commands Module
//!
//! This module contains all the Tauri commands that are exposed to the frontend.
//! These commands serve as the API between the frontend and backend, allowing
//! the frontend to interact with the application's core functionality.
//!
//! ## Command Categories
//!
//! - **Video Processing**: Commands for video information retrieval and conversion
//! - **Task Management**: Commands for managing processing tasks
//! - **State Management**: Commands for accessing and updating application state
//! - **File Management**: Commands for managing the file list
//!
//! Each command is annotated with `#[tauri::command]` and can be invoked from
//! the frontend using Tauri's invoke mechanism.

mod task_commands;

// Re-export task commands
pub use task_commands::*;


use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State, Emitter};

// Preset management has been moved to frontend
use crate::services::video_processor::{VideoInfo, VideoProcessor};
use crate::state::StateManager;
use crate::utils::error::{ErrorCode, ErrorInfo};
use crate::utils::error_handler;
use crate::{
    handle_command, handle_command_with_event, handle_string_as_error_info,

};

/// Basic greeting command for testing the Tauri command system
///
/// This command is a simple example that demonstrates how to create and use
/// Tauri commands. It takes a name parameter and returns a greeting message.
///
/// # Parameters
/// * `name` - The name to include in the greeting
///
/// # Returns
/// A greeting string that includes the provided name
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Video processing commands section

/// Retrieves detailed information about a video file
///
/// This command extracts metadata from a video file including format, duration,
/// dimensions, bitrate, codec, and framerate. It creates a temporary VideoProcessor
/// instance since this is a read-only operation.
///
/// # Parameters
/// * `path` - The file path to the video to analyze
///
/// # Returns
/// * `Result<VideoInfo, ErrorInfo>` - Video metadata or an error
#[tauri::command]
pub fn get_video_info(path: String, app_handle: AppHandle) -> Result<VideoInfo, ErrorInfo> {
    // Create a new processor for this operation
    // This is a read-only operation so we don't need to use the shared state
    let processor = VideoProcessor::new();
    handle_command_with_event!(processor.get_video_info(&path), &app_handle)
}

// Legacy commands are removed as they are replaced by the new task system

// Preset management commands have been moved to frontend

// Legacy state management wrapper commands have been removed as they are replaced by the new task system

// State access commands
/// Get application information including GPU and FFmpeg version
///
/// This command returns information about the application, including GPU availability,
/// GPU list, selected GPU, and FFmpeg version. It replaces the old app_state system.
///
/// # Returns
/// * `Result<AppInfo, ErrorInfo>` - Application information or an error
#[tauri::command]
pub fn get_app_info(app_handle: AppHandle) -> Result<AppInfo, ErrorInfo> {
    // Get FFmpeg version
    let ffmpeg_version = Some("FFmpeg 7.1.0".to_string()); // Replace with actual function

    // Check GPU availability
    let gpu_list = match crate::utils::gpu_detector::check_gpu_availability() {
        Ok(list) => list,
        Err(e) => return Err(ErrorInfo {
            code: ErrorCode::UnknownError,
            message: format!("Failed to detect GPU: {}", e),
            details: Some("Error detecting GPU capabilities".to_string()),
        }),
    };

    // Get selected GPU index from preferences
    let selected_gpu_index = match app_handle.state::<StateManager>().inner().preferences.state.lock().use_gpu {
        true => {
            // Find first available GPU
            if let Some((i, _)) = gpu_list.gpus.iter().enumerate().find(|(_, g)| g.is_available) {
                i as i32
            } else {
                -1 // No available GPU, use CPU
            }
        },
        false => -1, // Use CPU
    };

    // Create AppInfo
    let app_info = AppInfo {
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        ffmpeg_version,
        gpu_available: gpu_list.gpus.iter().any(|g| g.is_available),
        gpus: gpu_list.gpus,
        selected_gpu_index,
    };

    Ok(app_info)
}

/// Set selected GPU
///
/// This command sets the selected GPU index and updates the preferences.
///
/// # Parameters
/// * `gpu_index` - The index of the GPU to select (-1 for CPU, 0+ for GPU)
/// * `app_handle` - Tauri AppHandle for accessing application resources
///
/// # Returns
/// * `Result<(), ErrorInfo>` - Success or an error
#[tauri::command]
pub fn set_gpu(gpu_index: i32, app_handle: AppHandle) -> Result<(), ErrorInfo> {
    // Get GPU list
    let gpu_list = match crate::utils::gpu_detector::check_gpu_availability() {
        Ok(list) => list,
        Err(e) => return Err(ErrorInfo {
            code: ErrorCode::UnknownError,
            message: format!("Failed to detect GPU: {}", e),
            details: Some("Error detecting GPU capabilities".to_string()),
        }),
    };

    // Validate GPU index
    if gpu_index != -1 && (gpu_index < 0 || gpu_index as usize >= gpu_list.gpus.len()) {
        return Err(ErrorInfo {
            code: ErrorCode::InvalidArgument,
            message: format!("Invalid GPU index: {}", gpu_index),
            details: Some("GPU index out of range".to_string()),
        });
    }

    // We no longer update preferences in backend
    // Instead, we just emit the app-info-changed event

    // Emit app-info-changed event
    let _ = app_handle.emit("app-info-changed", get_app_info(app_handle.clone())?);

    Ok(())
}

/// Application information including GPU and FFmpeg version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub app_version: String,
    pub ffmpeg_version: Option<String>,
    pub gpu_available: bool,
    pub gpus: Vec<crate::utils::gpu_detector::GpuInfo>,
    pub selected_gpu_index: i32, // -1 for CPU, 0+ for GPU
}

// DEPRECATED: Legacy app state command - will be removed in future versions
// Use get_app_info instead
#[tauri::command]
pub fn get_app_state(
    state_manager: State<'_, StateManager>,
) -> Result<crate::state::app_state::AppState, ErrorInfo> {
    handle_string_as_error_info!(crate::state::get_app_state(state_manager))
}

/// Emit conversion-state-changed event
///
/// This command emits the conversion-state-changed event with the provided conversion state.
/// It is used for backward compatibility with the old conversion state system.
///
/// # Parameters
/// * `conversion_state` - The conversion state to emit
/// * `app_handle` - Tauri AppHandle for accessing application resources
///
/// # Returns
/// * `Result<(), ErrorInfo>` - Success or an error
#[tauri::command]
pub fn emit_conversion_state_changed(
    conversion_state: crate::state::conversion_state::ConversionState,
    app_handle: AppHandle,
) -> Result<(), ErrorInfo> {
    // Emit conversion-state-changed event
    let _ = app_handle.emit("conversion-state-changed", conversion_state);
    Ok(())
}

// DEPRECATED: Legacy conversion state command - will be removed in future versions
// Frontend should use its own state management
#[tauri::command]
pub fn get_conversion_state(
    state_manager: State<'_, StateManager>,
) -> Result<crate::state::conversion_state::ConversionState, ErrorInfo> {
    handle_string_as_error_info!(crate::state::get_conversion_state(state_manager))
}

/// Emit preferences-changed event
///
/// This command emits the preferences-changed event with the provided preferences.
/// It is used for backward compatibility with the old preferences system.
///
/// # Parameters
/// * `preferences` - The preferences to emit
/// * `app_handle` - Tauri AppHandle for accessing application resources
///
/// # Returns
/// * `Result<(), ErrorInfo>` - Success or an error
#[tauri::command]
pub fn emit_preferences_changed(
    preferences: crate::state::preferences_state::UserPreferencesState,
    app_handle: AppHandle,
) -> Result<(), ErrorInfo> {
    // Emit preferences-changed event
    let _ = app_handle.emit("preferences-changed", preferences);
    Ok(())
}

// DEPRECATED: Legacy preferences command - will be removed in future versions
// Frontend should use its own state management
#[tauri::command]
pub fn get_preferences(
    state_manager: State<'_, StateManager>,
) -> Result<crate::state::preferences_state::UserPreferencesState, ErrorInfo> {
    handle_string_as_error_info!(crate::state::get_preferences(state_manager))
}

// DEPRECATED: Legacy update_preferences command - will be removed in future versions
// Frontend should use its own state management
#[tauri::command]
pub fn update_preferences(
    preferences: crate::state::preferences_state::UserPreferencesState,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> Result<(), ErrorInfo> {
    handle_string_as_error_info!(crate::state::update_preferences(
        preferences,
        state_manager,
        app_handle
    ))
}

#[tauri::command]
pub fn get_global_state(
    state_manager: State<'_, StateManager>,
) -> Result<crate::state::GlobalState, ErrorInfo> {
    handle_string_as_error_info!(crate::state::get_global_state(state_manager))
}

// DEPRECATED: Legacy file management commands - will be removed in future versions
// Frontend should use its own state management
#[tauri::command]
pub fn add_file_to_list(
    id: String,
    name: String,
    path: String,
    size: u64,
    file_type: String,
    duration: Option<f64>,
    resolution: Option<(u32, u32)>,
    thumbnail: Option<String>,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> Result<(), ErrorInfo> {
    // Convert String -> Uuid and String -> PathBuf
    let uuid = match crate::state::conversion_state::get_file_id_from_string(&id) {
        Ok(uuid) => uuid,
        Err(e) => return Err(error_handler::to_error_info(e)),
    };

    let file_info = crate::state::FileInfo {
        id: uuid,
        name,
        path: std::path::PathBuf::from(path),
        size,
        file_type,
        duration,
        resolution,
        thumbnail,
    };

    handle_command!(crate::state::add_file_to_list(
        file_info,
        state_manager,
        app_handle
    ))
}

// DEPRECATED: Legacy remove_file_from_list command - will be removed in future versions
// Frontend should use its own state management
#[tauri::command]
pub fn remove_file_from_list(
    file_id: String,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> Result<(), ErrorInfo> {
    handle_string_as_error_info!(crate::state::remove_file_from_list(
        file_id,
        state_manager,
        app_handle
    ))
}

// DEPRECATED: Legacy select_file command - will be removed in future versions
// Frontend should use its own state management
#[tauri::command]
pub fn select_file(
    file_id: Option<String>,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> Result<(), ErrorInfo> {
    handle_string_as_error_info!(crate::state::select_file(
        file_id,
        state_manager,
        app_handle
    ))
}

// DEPRECATED: Legacy clear_file_list command - will be removed in future versions
// Frontend should use its own state management
#[tauri::command]
pub fn clear_file_list(
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> Result<(), ErrorInfo> {
    handle_string_as_error_info!(crate::state::clear_file_list(state_manager, app_handle))
}

// DEPRECATED: Legacy save_preferences_to_file command - will be removed in future versions
// Frontend should use its own state management
#[tauri::command]
pub fn save_preferences_to_file(app_handle: AppHandle) -> Result<(), ErrorInfo> {
    handle_string_as_error_info!(crate::state::save_preferences_to_file(app_handle))
}

// DEPRECATED: Legacy load_preferences_from_file command - will be removed in future versions
// Frontend should use its own state management
#[tauri::command]
pub fn load_preferences_from_file(app_handle: AppHandle) -> Result<(), ErrorInfo> {
    handle_string_as_error_info!(crate::state::load_preferences_from_file(app_handle))
}

#[tauri::command]
pub fn set_selected_gpu(
    gpu_index: i32,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> Result<(), ErrorInfo> {
    handle_string_as_error_info!(crate::state::set_selected_gpu(
        gpu_index,
        state_manager,
        app_handle
    ))
}

/// Get the path to the current log file
///
/// This command returns the path to the current log file, which is stored
/// in the application data directory.
///
/// # Parameters
/// * `app_handle` - Tauri AppHandle for accessing application resources
///
/// # Returns
/// * `Result<String, ErrorInfo>` - Path to the log file as a string if successful, or an error
#[tauri::command]
pub fn get_current_log_file_path(app_handle: AppHandle) -> Result<String, ErrorInfo> {
    match crate::utils::logger::get_current_log_file_path(&app_handle) {
        Ok(path) => Ok(path.to_string_lossy().to_string()),
        Err(e) => Err(ErrorInfo {
            code: ErrorCode::FileReadError,
            message: format!("Failed to get log file path: {}", e),
            details: Some("Error accessing log file".to_string()),
        }),
    }
}

/// Open the current log file in the default text editor
///
/// # Parameters
/// * `app_handle` - Tauri AppHandle for accessing application resources
///
/// # Returns
/// * `Result<bool, ErrorInfo>` - True if successful, or an error
#[tauri::command]
pub fn open_log_file(app_handle: AppHandle) -> Result<bool, ErrorInfo> {
    match crate::utils::logger::open_log_file(&app_handle) {
        Ok(result) => Ok(result),
        Err(e) => Err(ErrorInfo {
            code: ErrorCode::FileReadError,
            message: format!("Failed to open log file: {}", e),
            details: Some("Error opening log file".to_string()),
        }),
    }
}

/// Open the log directory in the file explorer
///
/// # Parameters
/// * `app_handle` - Tauri AppHandle for accessing application resources
///
/// # Returns
/// * `Result<bool, ErrorInfo>` - True if successful, or an error
#[tauri::command]
pub fn open_log_directory(app_handle: AppHandle) -> Result<bool, ErrorInfo> {
    match crate::utils::logger::open_log_directory(&app_handle) {
        Ok(result) => Ok(result),
        Err(e) => Err(ErrorInfo {
            code: ErrorCode::FileReadError,
            message: format!("Failed to open log directory: {}", e),
            details: Some("Error opening log directory".to_string()),
        }),
    }
}

// Legacy processor state and related functions are removed as they are replaced by the new task system
