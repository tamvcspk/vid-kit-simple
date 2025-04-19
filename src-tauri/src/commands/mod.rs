//! # Commands Module
//!
//! This module contains all the Tauri commands that are exposed to the frontend.
//! These commands serve as the API between the frontend and backend, allowing
//! the frontend to interact with the application's core functionality.
//!
//! ## Command Categories
//!
//! - **Video Processing**: Commands for video information retrieval and conversion
//! - **Preset Management**: Commands for managing conversion presets
//! - **State Management**: Commands for accessing and updating application state
//! - **File Management**: Commands for managing the file list
//!
//! Each command is annotated with `#[tauri::command]` and can be invoked from
//! the frontend using Tauri's invoke mechanism.

use tauri::{AppHandle, State, Manager};
use std::sync::Mutex;
use std::collections::HashMap;

use crate::services::video_processor::{ProcessingOptions, VideoProcessor, VideoInfo};
use crate::services::preset_manager::{ConversionPreset};
use crate::state::StateManager;
use crate::utils::error::{AppError, ErrorCode, ErrorInfo};
use crate::{handle_command, handle_string_as_error_info};
use crate::utils::error_handler;

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
pub fn get_video_info(path: String) -> Result<VideoInfo, ErrorInfo> {
    // Create a new processor for this operation
    // This is a read-only operation so we don't need to use the shared state
    let processor = VideoProcessor::new();
    handle_command!(processor.get_video_info(&path))
}

/// Creates a new video processing task with the specified parameters
///
/// This command creates a new task for video conversion based on the provided
/// input file, output file, and conversion settings. It registers the task with
/// both the VideoProcessor and the state management system.
///
/// # Parameters
/// * `input_file` - Path to the input video file
/// * `output_file` - Path where the converted video will be saved
/// * `settings` - Map of conversion settings (format, resolution, bitrate, etc.)
/// * `app_handle` - Tauri AppHandle for accessing application resources
/// * `processor_state` - State containing the VideoProcessor instance
///
/// # Returns
/// * `Result<String, ErrorInfo>` - Task ID if successful, or an error
#[tauri::command]
pub fn create_processing_task(
    input_file: String,
    output_file: String,
    settings: HashMap<String, String>,
    app_handle: tauri::AppHandle,
    processor_state: State<'_, ProcessorState>
) -> Result<String, ErrorInfo> {
    // Create options from settings
    let options = match make_processing_options(&settings) {
        Ok(opts) => opts,
        Err(e) => {
            return Err(ErrorInfo {
                code: ErrorCode::InvalidArgument,
                message: format!("Invalid settings: {}", e),
                details: Some(format!("Error parsing conversion settings for file: {}", input_file))
            });
        }
    };

    // Use video_processor from processor_state
    // We need to clone it to get a mutable reference
    let mut video_processor = processor_state.video_processor.clone();

    // Set AppHandle
    video_processor.set_app_handle(app_handle.clone());

    // Store copies of input and output file paths for error reporting
    let input_file_copy = input_file.clone();
    let output_file_copy = output_file.clone();

    // Create task and handle errors
    let task_id_result = video_processor.create_task(input_file, output_file, options);

    // Handle errors
    let task_id = match task_id_result {
        Ok(id) => id,
        Err(e) => {
            return Err(ErrorInfo {
                code: ErrorCode::VideoProcessingFailed,
                message: format!("Failed to create processing task: {}", e),
                details: Some(format!("Error creating processing task for input file: {} and output file: {}", input_file_copy, output_file_copy))
            });
        }
    };

    // Register task with state manager
    if let Err(e) = crate::state::add_conversion_task(task_id.clone(), app_handle.clone()) {
        log::warn!("Failed to register task with state manager: {}", e);
    }

    Ok(task_id)
}

#[tauri::command]
pub fn run_processing_task(
    task_id: String,
    processor_state: State<'_, ProcessorState>,
    app_handle: AppHandle
) -> Result<(), ErrorInfo> {
    // Use video_processor from processor_state
    // We need to clone it to get a mutable reference
    let mut video_processor = processor_state.video_processor.clone();

    // Set AppHandle
    video_processor.set_app_handle(app_handle.clone());

    // Update task status in state manager
    let task_uuid = match crate::state::conversion_state::get_task_id_from_string(&task_id) {
        Ok(uuid) => uuid,
        Err(e) => {
            return Err(ErrorInfo {
                code: ErrorCode::TaskNotFound,
                message: format!("Invalid task ID: {}", e),
                details: Some(format!("Could not find or parse task ID: {}", task_id))
            });
        }
    };

    if let Err(e) = crate::state::conversion_state::start_task(task_uuid, app_handle.clone()) {
        log::warn!("Failed to update task status in state manager: {}", e);
    }

    // Call run_task and handle errors
    let result = video_processor.run_task(&task_id);

    // Handle errors
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            // Mark task as failed in state manager
            if let Err(state_err) = crate::state::mark_task_failed(task_id.clone(), app_handle.clone()) {
                log::warn!("Failed to mark task as failed in state manager: {}", state_err);
            }

            Err(ErrorInfo {
                code: ErrorCode::VideoProcessingFailed,
                message: format!("Failed to run processing task: {}", e),
                details: Some(format!("Error executing video processing task: {}", task_id))
            })
        }
    }
}

// Preset management commands
#[tauri::command]
pub fn list_presets(app_handle: AppHandle) -> Result<Vec<ConversionPreset>, ErrorInfo> {
    let manager = match crate::services::preset_manager::PresetManager::from_app_handle(&app_handle) {
        Ok(manager) => manager,
        Err(e) => {
            return Err(error_handler::to_error_info(AppError::preset_error(
                format!("Failed to create preset manager: {}", e),
                ErrorCode::PresetValidationError,
                Some("Error initializing preset manager to list presets".to_string())
            )));
        }
    };

    handle_command!(manager.list_presets())
}

#[tauri::command]
pub fn get_preset(id: String, app_handle: AppHandle) -> Result<ConversionPreset, ErrorInfo> {
    let manager = match crate::services::preset_manager::PresetManager::from_app_handle(&app_handle) {
        Ok(manager) => manager,
        Err(e) => return Err(error_handler::string_to_error_info(
            format!("Failed to create preset manager: {} (when getting preset {})", e, id)
        )),
    };

    handle_string_as_error_info!(manager.get_preset(&id))
}

#[tauri::command]
pub fn save_preset(preset: ConversionPreset, app_handle: AppHandle) -> Result<(), ErrorInfo> {
    let manager = match crate::services::preset_manager::PresetManager::from_app_handle(&app_handle) {
        Ok(manager) => manager,
        Err(e) => return Err(error_handler::string_to_error_info(
            format!("Failed to create preset manager: {} (when saving preset {})", e, preset.name)
        )),
    };

    handle_string_as_error_info!(manager.save_preset(&preset))
}

#[tauri::command]
pub fn delete_preset(id: String, app_handle: AppHandle) -> Result<(), ErrorInfo> {
    let manager = match crate::services::preset_manager::PresetManager::from_app_handle(&app_handle) {
        Ok(manager) => manager,
        Err(e) => return Err(error_handler::string_to_error_info(
            format!("Failed to create preset manager: {} (when deleting preset {})", e, id)
        )),
    };

    handle_string_as_error_info!(manager.delete_preset(&id))
}

#[tauri::command]
pub fn create_default_presets(app_handle: AppHandle) -> Result<(), ErrorInfo> {
    let manager = match crate::services::preset_manager::PresetManager::from_app_handle(&app_handle) {
        Ok(manager) => manager,
        Err(e) => return Err(error_handler::string_to_error_info(
            format!("Failed to create preset manager: {} (when creating default presets)", e)
        )),
    };

    handle_string_as_error_info!(manager.create_default_presets())
}

// State management wrapper commands
#[tauri::command]
pub fn update_conversion_progress_wrapper(
    task_id: String,
    progress: f32,
    app_handle: AppHandle,
) -> Result<(), ErrorInfo> {
    handle_string_as_error_info!(crate::state::update_conversion_progress(task_id, progress, app_handle))
}

#[tauri::command]
pub fn add_conversion_task_wrapper(
    task_id: String,
    app_handle: AppHandle,
) -> Result<(), ErrorInfo> {
    handle_string_as_error_info!(crate::state::add_conversion_task(task_id, app_handle))
}

#[tauri::command]
pub fn mark_task_failed_wrapper(
    task_id: String,
    app_handle: AppHandle,
) -> Result<(), ErrorInfo> {
    handle_string_as_error_info!(crate::state::mark_task_failed(task_id, app_handle))
}

// State access commands
#[tauri::command]
pub fn get_app_state(state_manager: State<'_, StateManager>) -> Result<crate::state::app_state::AppState, ErrorInfo> {
    handle_string_as_error_info!(crate::state::get_app_state(state_manager))
}

#[tauri::command]
pub fn get_conversion_state(state_manager: State<'_, StateManager>) -> Result<crate::state::conversion_state::ConversionState, ErrorInfo> {
    handle_string_as_error_info!(crate::state::get_conversion_state(state_manager))
}

#[tauri::command]
pub fn get_preferences(state_manager: State<'_, StateManager>) -> Result<crate::state::preferences_state::UserPreferencesState, ErrorInfo> {
    handle_string_as_error_info!(crate::state::get_preferences(state_manager))
}

#[tauri::command]
pub fn update_preferences(
    preferences: crate::state::preferences_state::UserPreferencesState,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> Result<(), ErrorInfo> {
    handle_string_as_error_info!(crate::state::update_preferences(preferences, state_manager, app_handle))
}

#[tauri::command]
pub fn get_global_state(state_manager: State<'_, StateManager>) -> Result<crate::state::GlobalState, ErrorInfo> {
    handle_string_as_error_info!(crate::state::get_global_state(state_manager))
}

// File management commands
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

    handle_command!(crate::state::add_file_to_list(file_info, state_manager, app_handle))
}

#[tauri::command]
pub fn remove_file_from_list(
    file_id: String,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> Result<(), ErrorInfo> {
    handle_string_as_error_info!(crate::state::remove_file_from_list(file_id, state_manager, app_handle))
}

#[tauri::command]
pub fn select_file(
    file_id: Option<String>,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> Result<(), ErrorInfo> {
    handle_string_as_error_info!(crate::state::select_file(file_id, state_manager, app_handle))
}

#[tauri::command]
pub fn clear_file_list(
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> Result<(), ErrorInfo> {
    handle_string_as_error_info!(crate::state::clear_file_list(state_manager, app_handle))
}

#[tauri::command]
pub fn save_preferences_to_file(app_handle: AppHandle) -> Result<(), ErrorInfo> {
    handle_string_as_error_info!(crate::state::save_preferences_to_file(app_handle))
}

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
    handle_string_as_error_info!(crate::state::set_selected_gpu(gpu_index, state_manager, app_handle))
}

/// State container for the VideoProcessor instance
///
/// This struct is managed by Tauri's state system and provides access to the
/// VideoProcessor instance across different commands. It allows multiple commands
/// to share the same VideoProcessor instance, maintaining task state between calls.
pub struct ProcessorState {
    /// The shared VideoProcessor instance
    pub video_processor: VideoProcessor,
}

/// Manually triggers cleanup of completed and failed tasks
///
/// This command removes old completed and failed tasks from the VideoProcessor's
/// task list based on their age. It helps prevent memory leaks by ensuring that
/// old tasks don't accumulate indefinitely.
///
/// # Parameters
/// * `processor_state` - State containing the VideoProcessor instance
///
/// # Returns
/// * `Result<(), ErrorInfo>` - Success or an error
#[tauri::command]
pub fn cleanup_video_tasks(processor_state: State<'_, ProcessorState>) -> Result<(), ErrorInfo> {
    // Clone the processor to get a mutable reference
    let mut video_processor = processor_state.video_processor.clone();

    // Call cleanup_tasks
    video_processor.cleanup_tasks();

    Ok(())
}

/// Initializes the ProcessorState with a new VideoProcessor instance
///
/// This function creates a new VideoProcessor instance and configures it with
/// the application handle. It's used during application startup to initialize
/// the shared processor state.
///
/// # Parameters
/// * `app_handle` - Reference to the Tauri AppHandle
///
/// # Returns
/// * `ProcessorState` - The initialized processor state
pub fn init_processor_state(app_handle: &tauri::AppHandle) -> ProcessorState {
    // Create a new video processor
    let mut processor = VideoProcessor::new();

    // Set AppHandle by cloning
    let app_handle_clone = app_handle.clone();
    processor.set_app_handle(app_handle_clone);

    ProcessorState {
        video_processor: processor,
    }
}

/// Registers the ProcessorState with the Tauri application
///
/// This function is called during application setup to register the ProcessorState
/// with Tauri's state management system. It initializes the processor state and
/// makes it available to all commands.
///
/// # Parameters
/// * `app` - Mutable reference to the Tauri App instance
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Success or an error
pub fn register_processor_state(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // Get app_handle from app
    let app_handle = app.handle().clone();

    // Initialize ProcessorState
    let processor_state = init_processor_state(&app_handle);

    // Register state
    app.manage(processor_state);

    Ok(())
}

/// Converts a settings HashMap into a ProcessingOptions struct
///
/// This utility function parses the settings map provided by the frontend and
/// converts it into a structured ProcessingOptions object that can be used by
/// the VideoProcessor. It handles type conversion and provides default values
/// where appropriate.
///
/// # Parameters
/// * `settings` - HashMap containing conversion settings as string key-value pairs
///
/// # Returns
/// * `Result<ProcessingOptions, String>` - Parsed options or an error message
fn make_processing_options(settings: &HashMap<String, String>) -> Result<ProcessingOptions, String> {
    // Default options
    let mut options = ProcessingOptions {
        output_format: settings.get("format").cloned().unwrap_or_else(|| "mp4".to_string()),
        output_path: String::new(), // Will be set by output_file in create_task function
        resolution: None,
        bitrate: None,
        framerate: None,
        use_gpu: settings.get("use_gpu").map(|s| s == "true").unwrap_or(false),
        gpu_codec: None,
        cpu_codec: None,
    };

    // Handle resolution
    if let (Some(width), Some(height)) = (
        settings.get("width").and_then(|w| w.parse::<u32>().ok()),
        settings.get("height").and_then(|h| h.parse::<u32>().ok())
    ) {
        options.resolution = Some((width, height));
    }

    // Handle bitrate
    if let Some(bitrate_str) = settings.get("bitrate") {
        if let Ok(bitrate) = bitrate_str.parse::<u64>() {
            options.bitrate = Some(bitrate);
        }
    }

    // Handle framerate
    if let Some(framerate_str) = settings.get("framerate") {
        if let Ok(framerate) = framerate_str.parse::<f32>() {
            options.framerate = Some(framerate);
        }
    }

    // Set codec based on GPU usage
    if options.use_gpu {
        options.gpu_codec = settings.get("gpu_codec").cloned();
    } else {
        options.cpu_codec = settings.get("cpu_codec").cloned().or(Some("h264".to_string()));
    }

    Ok(options)
}
