use tauri::{AppHandle, State};
use std::sync::Mutex;

use crate::services::video_processor::{ProcessingOptions, VideoProcessor, VideoInfo};
use crate::services::preset_manager::{ConversionPreset};
use crate::state::StateManager;

// Basic greeting command for testing
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Video processing commands
#[tauri::command]
pub fn get_video_info(path: String) -> Result<VideoInfo, String> {
    let processor = VideoProcessor::new();
    processor.get_video_info(&path)
}

#[tauri::command]
pub fn create_processing_task(
    input_file: String,
    options: ProcessingOptions,
    processor_state: State<'_, ProcessorState>,
) -> Result<String, String> {
    let mut processor = processor_state.video_processor.lock().unwrap();
    Ok(processor.create_task(&input_file, options))
}

#[tauri::command]
pub fn run_processing_task(
    task_id: String,
    processor_state: State<'_, ProcessorState>,
    app_handle: AppHandle
) -> Result<(), String> {
    // Lấy VideoProcessor từ state
    let mut processor = processor_state.video_processor.lock().unwrap();

    // Chạy task và bắt lỗi nếu có
    match processor.run_task(&task_id) {
        Ok(_) => Ok(()),
        Err(e) => {
            // Đánh dấu task là thất bại trong state manager
            let _ = crate::state::mark_task_failed(task_id, app_handle);
            Err(e)
        }
    }
}

// Preset management commands
#[tauri::command]
pub fn list_presets(app_handle: AppHandle) -> Result<Vec<ConversionPreset>, String> {
    let manager = match crate::services::preset_manager::PresetManager::from_app_handle(&app_handle) {
        Ok(manager) => manager,
        Err(e) => return Err(format!("Failed to create preset manager: {}", e)),
    };

    manager.list_presets().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_preset(id: String, app_handle: AppHandle) -> Result<ConversionPreset, String> {
    let manager = match crate::services::preset_manager::PresetManager::from_app_handle(&app_handle) {
        Ok(manager) => manager,
        Err(e) => return Err(format!("Failed to create preset manager: {}", e)),
    };

    manager.get_preset(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_preset(preset: ConversionPreset, app_handle: AppHandle) -> Result<(), String> {
    let manager = match crate::services::preset_manager::PresetManager::from_app_handle(&app_handle) {
        Ok(manager) => manager,
        Err(e) => return Err(format!("Failed to create preset manager: {}", e)),
    };

    manager.save_preset(&preset).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_preset(id: String, app_handle: AppHandle) -> Result<(), String> {
    let manager = match crate::services::preset_manager::PresetManager::from_app_handle(&app_handle) {
        Ok(manager) => manager,
        Err(e) => return Err(format!("Failed to create preset manager: {}", e)),
    };

    manager.delete_preset(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_default_presets(app_handle: AppHandle) -> Result<(), String> {
    let manager = match crate::services::preset_manager::PresetManager::from_app_handle(&app_handle) {
        Ok(manager) => manager,
        Err(e) => return Err(format!("Failed to create preset manager: {}", e)),
    };

    manager.create_default_presets().map_err(|e| e.to_string())
}

// State management wrapper commands
#[tauri::command]
pub fn update_conversion_progress_wrapper(
    task_id: String,
    progress: f32,
    app_handle: AppHandle,
) -> Result<(), String> {
    crate::state::update_conversion_progress(task_id, progress, app_handle)
}

#[tauri::command]
pub fn add_conversion_task_wrapper(
    task_id: String,
    app_handle: AppHandle,
) -> Result<(), String> {
    crate::state::add_conversion_task(task_id, app_handle)
}

#[tauri::command]
pub fn mark_task_failed_wrapper(
    task_id: String,
    app_handle: AppHandle,
) -> Result<(), String> {
    crate::state::mark_task_failed(task_id, app_handle)
}

// State access commands
#[tauri::command]
pub fn get_app_state(state_manager: State<'_, StateManager>) -> Result<crate::state::AppState, String> {
    Ok(state_manager.app.lock().unwrap().clone())
}

#[tauri::command]
pub fn get_conversion_state(state_manager: State<'_, StateManager>) -> Result<crate::state::ConversionState, String> {
    Ok(state_manager.conversion.lock().unwrap().clone())
}

#[tauri::command]
pub fn get_preferences(state_manager: State<'_, StateManager>) -> Result<crate::state::UserPreferencesState, String> {
    Ok(state_manager.preferences.lock().unwrap().clone())
}

#[tauri::command]
pub fn update_preferences(
    preferences: crate::state::UserPreferencesState,
    state_manager: State<'_, StateManager>,
) -> Result<(), String> {
    let mut prefs = state_manager.preferences.lock().unwrap();
    *prefs = preferences;
    Ok(())
}

#[tauri::command]
pub fn get_global_state(state_manager: State<'_, StateManager>) -> Result<crate::state::GlobalState, String> {
    let app = state_manager.app.lock().unwrap().clone();
    let conversion = state_manager.conversion.lock().unwrap().clone();
    let preferences = state_manager.preferences.lock().unwrap().clone();

    Ok(crate::state::GlobalState {
        app,
        conversion,
        preferences,
    })
}

// File management commands
#[tauri::command]
pub fn add_file_to_list(
    id: String,
    name: String,
    path: String,
    size: u64,
    fileType: String,
    duration: Option<f64>,
    resolution: Option<(u32, u32)>,
    thumbnail: Option<String>,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> Result<(), String> {
    let file_info = crate::state::FileInfo {
        id,
        name,
        path,
        size,
        file_type: fileType,
        duration,
        resolution,
        thumbnail,
    };
    crate::state::add_file_to_list(file_info, state_manager, app_handle)
}

#[tauri::command]
pub fn remove_file_from_list(
    file_id: String,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> Result<(), String> {
    crate::state::remove_file_from_list(file_id, state_manager, app_handle)
}

#[tauri::command]
pub fn select_file(
    file_id: Option<String>,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> Result<(), String> {
    crate::state::select_file(file_id, state_manager, app_handle)
}

#[tauri::command]
pub fn clear_file_list(
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> Result<(), String> {
    crate::state::clear_file_list(state_manager, app_handle)
}

#[tauri::command]
pub fn save_preferences_to_file(app_handle: AppHandle) -> Result<(), String> {
    crate::state::save_preferences_to_file(app_handle)
}

#[tauri::command]
pub fn load_preferences_from_file(app_handle: AppHandle) -> Result<(), String> {
    crate::state::load_preferences_from_file(app_handle)
}

#[tauri::command]
pub fn set_selected_gpu(
    gpu_index: i32,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> Result<(), String> {
    crate::state::set_selected_gpu(gpu_index, state_manager, app_handle)
}

// Define state for VideoProcessor
pub struct ProcessorState {
    pub video_processor: Mutex<VideoProcessor>,
}

// Initialize state
pub fn init_processor_state() -> ProcessorState {
    ProcessorState {
        video_processor: Mutex::new(VideoProcessor::new()),
    }
}
