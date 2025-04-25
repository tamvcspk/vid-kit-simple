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


use tauri::{AppHandle, Emitter};

// Preset management has been moved to frontend
use crate::services::video_processor::{VideoInfo, VideoProcessor};
use crate::utils::error::{ErrorCode, ErrorInfo};
use crate::handle_command_with_event;

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
pub fn get_app_info(_app_handle: AppHandle) -> Result<crate::utils::app_info::AppInfo, ErrorInfo> {
    match crate::utils::app_info::get_app_info() {
        Ok(app_info) => Ok(app_info),
        Err(e) => Err(ErrorInfo {
            code: ErrorCode::UnknownError,
            message: format!("Failed to get app info: {}", e),
            details: Some("Error getting application information".to_string()),
        }),
    }
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
    match crate::utils::app_info::set_gpu(gpu_index, &app_handle) {
        Ok(_) => Ok(()),
        Err(e) => Err(ErrorInfo {
            code: ErrorCode::UnknownError,
            message: format!("Failed to set GPU: {}", e),
            details: Some("Error setting GPU".to_string()),
        }),
    }
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
    preferences: serde_json::Value,
    app_handle: AppHandle,
) -> Result<(), ErrorInfo> {
    // Emit preferences-changed event
    let _ = app_handle.emit("preferences-changed", preferences);
    Ok(())
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
    conversion_state: serde_json::Value,
    app_handle: AppHandle,
) -> Result<(), ErrorInfo> {
    // Emit conversion-state-changed event
    let _ = app_handle.emit("conversion-state-changed", conversion_state);
    Ok(())
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
