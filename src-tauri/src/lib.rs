pub mod gpu_detector;
pub mod preset_manager;
pub mod video_processor;

use std::sync::Mutex;
use tauri::State;

use gpu_detector::check_gpu_availability;
use preset_manager::{
    create_default_presets, delete_preset, get_preset, list_presets, save_preset,
};
use video_processor::{ProcessingOptions, VideoProcessor};

// Định nghĩa state để lưu trữ VideoProcessor
struct AppState {
    video_processor: Mutex<VideoProcessor>,
}

// Khởi tạo state
fn init_app_state() -> AppState {
    AppState {
        video_processor: Mutex::new(VideoProcessor::new()),
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_video_info(path: String) -> Result<video_processor::VideoInfo, String> {
    let processor = VideoProcessor::new();
    processor.get_video_info(&path)
}

#[tauri::command]
fn create_processing_task<'a>(
    input_file: String,
    options: ProcessingOptions,
    state: State<'a, AppState>,
) -> Result<String, String> {
    // Lấy VideoProcessor từ state
    let mut processor = state.video_processor.lock().unwrap();
    Ok(processor.create_task(&input_file, options))
}

#[tauri::command]
fn run_processing_task<'a>(task_id: String, state: State<'a, AppState>) -> Result<(), String> {
    // Lấy VideoProcessor từ state
    let mut processor = state.video_processor.lock().unwrap();
    processor.run_task(&task_id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        // Thêm state vào ứng dụng
        .manage(init_app_state())
        .invoke_handler(tauri::generate_handler![
            greet,
            check_gpu_availability,
            list_presets,
            get_preset,
            save_preset,
            delete_preset,
            create_default_presets,
            get_video_info,
            create_processing_task,
            run_processing_task
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
