pub mod gpu_detector;
pub mod preset_manager;
pub mod video_processor;
pub mod state_manager;

use std::sync::Mutex;
use tauri::{State, Manager};

use gpu_detector::check_gpu_availability;
use preset_manager::{
    create_default_presets, delete_preset, get_preset, list_presets, save_preset,
};
use video_processor::{ProcessingOptions, VideoProcessor};
use state_manager::{
    StateManager, get_app_state, get_conversion_state, get_preferences,
    update_preferences, update_conversion_progress, add_conversion_task,
    mark_task_failed, save_preferences_to_file, load_preferences_from_file,
    set_selected_gpu, add_file_to_list, remove_file_from_list, select_file,
    clear_file_list, get_global_state
};

// Wrapper cho update_conversion_progress để tự động truyền app_handle
#[tauri::command]
fn update_conversion_progress_wrapper<'a>(
    task_id: String,
    progress: f32,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    update_conversion_progress(task_id, progress, app_handle)
}

// Wrapper cho add_conversion_task để tự động truyền app_handle
#[tauri::command]
fn add_conversion_task_wrapper<'a>(
    task_id: String,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    add_conversion_task(task_id, app_handle)
}

// Wrapper cho mark_task_failed để tự động truyền app_handle
#[tauri::command]
fn mark_task_failed_wrapper<'a>(
    task_id: String,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    mark_task_failed(task_id, app_handle)
}

// Định nghĩa state để lưu trữ VideoProcessor
struct ProcessorState {
    video_processor: Mutex<VideoProcessor>,
}

// Khởi tạo state
fn init_processor_state() -> ProcessorState {
    ProcessorState {
        video_processor: Mutex::new(VideoProcessor::new()),
    }
}

// Khởi tạo state manager
fn init_state_manager() -> StateManager {
    StateManager::new()
}

// Sử dụng hàm get_global_state từ state_manager_new

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
    processor_state: State<'a, ProcessorState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    // Lấy VideoProcessor từ state
    let mut processor = processor_state.video_processor.lock().unwrap();
    let task_id = processor.create_task(&input_file, options);

    // Cập nhật state manager
    let _ = add_conversion_task(task_id.clone(), app_handle);

    Ok(task_id)
}

#[tauri::command]
fn run_processing_task<'a>(
    task_id: String,
    processor_state: State<'a, ProcessorState>,
    app_handle: tauri::AppHandle
) -> Result<(), String> {
    // Lấy VideoProcessor từ state
    let mut processor = processor_state.video_processor.lock().unwrap();

    // Chạy task và bắt lỗi nếu có
    match processor.run_task(&task_id) {
        Ok(_) => Ok(()),
        Err(e) => {
            // Đánh dấu task là thất bại trong state manager
            let _ = mark_task_failed(task_id, app_handle);
            Err(e)
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        // Thêm state vào ứng dụng
        .manage(init_processor_state())
        .manage(init_state_manager())
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
            run_processing_task,
            // State manager commands
            get_app_state,
            get_conversion_state,
            get_preferences,
            update_preferences,
            update_conversion_progress_wrapper,
            add_conversion_task_wrapper,
            mark_task_failed_wrapper,
            save_preferences_to_file,
            load_preferences_from_file,
            get_global_state,
            set_selected_gpu,
            add_file_to_list,
            remove_file_from_list,
            select_file,
            clear_file_list
        ])
        .setup(|app| {
            // Khởi tạo state manager với thông tin FFmpeg và GPU
            let state_manager = app.state::<StateManager>();

            // Kiểm tra GPU và lấy danh sách GPU
            let (gpu_available, gpus) = match check_gpu_availability() {
                Ok(gpu_list) => {
                    let is_available = gpu_list.gpus.iter().any(|gpu| gpu.is_available);
                    (is_available, gpu_list.gpus)
                },
                Err(_) => (false, Vec::new()),
            };

            // Lấy phiên bản FFmpeg (có thể thêm hàm vào video_processor để lấy phiên bản)
            let ffmpeg_version = Some("FFmpeg 6.0".to_string()); // Thay bằng hàm thực tế

            // Khởi tạo state
            state_manager.initialize(ffmpeg_version, gpu_available, gpus);

            // Load preferences từ file
            let app_handle = app.app_handle().clone();
            let _ = load_preferences_from_file(app_handle);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
