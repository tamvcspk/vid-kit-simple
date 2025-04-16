use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use tauri::{Manager, State, AppHandle, Emitter};

// Thông tin về GPU
use crate::utils::gpu_detector::GpuInfo;

// Định nghĩa các loại state khác nhau
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppState {
    pub is_initialized: bool,
    pub app_version: String,
    pub ffmpeg_version: Option<String>,
    pub gpu_available: bool,
    pub gpus: Vec<GpuInfo>,
    pub selected_gpu_index: i32, // -1 cho CPU, 0+ cho GPU
}

// Thông tin về file video
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub size: u64,
    pub file_type: String,
    pub duration: Option<f64>,
    pub resolution: Option<(u32, u32)>,
    pub thumbnail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConversionState {
    pub active_tasks: Vec<String>,
    pub completed_tasks: Vec<String>,
    pub failed_tasks: Vec<String>,
    pub current_progress: f32,
    pub files: Vec<FileInfo>,
    pub selected_file_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserPreferencesState {
    pub default_output_dir: Option<String>,
    pub default_format: String,
    pub use_gpu: bool,
    pub theme: String,
}

// GlobalState kết hợp tất cả các state để trả về cho frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalState {
    pub app: AppState,
    pub conversion: ConversionState,
    pub preferences: UserPreferencesState,
}

// Struct chính để quản lý tất cả state
pub struct StateManager {
    pub app: Mutex<AppState>,
    pub conversion: Mutex<ConversionState>,
    pub preferences: Mutex<UserPreferencesState>,
}

// Các hàm để khởi tạo state
impl StateManager {
    pub fn new() -> Self {
        Self {
            app: Mutex::new(AppState {
                is_initialized: false,
                app_version: env!("CARGO_PKG_VERSION").to_string(),
                ffmpeg_version: None,
                gpu_available: false,
                gpus: Vec::new(),
                selected_gpu_index: -1, // Mặc định là CPU
            }),
            conversion: Mutex::new(ConversionState {
                active_tasks: Vec::new(),
                completed_tasks: Vec::new(),
                failed_tasks: Vec::new(),
                current_progress: 0.0,
                files: Vec::new(),
                selected_file_id: None,
            }),
            preferences: Mutex::new(UserPreferencesState {
                default_output_dir: None,
                default_format: "mp4".to_string(),
                use_gpu: false,
                theme: "light".to_string(),
            }),
        }
    }

    // Khởi tạo state với các giá trị mặc định
    pub fn initialize(&self, ffmpeg_version: Option<String>, gpu_available: bool, gpus: Vec<GpuInfo>) {
        let mut app = self.app.lock().unwrap();
        app.is_initialized = true;
        app.ffmpeg_version = ffmpeg_version;
        app.gpu_available = gpu_available;
        app.gpus = gpus;

        // Nếu có GPU khả dụng, chọn GPU đầu tiên có sẵn
        if gpu_available {
            for (i, gpu) in app.gpus.iter().enumerate() {
                if gpu.is_available {
                    app.selected_gpu_index = i as i32;
                    break;
                }
            }
        } else {
            app.selected_gpu_index = -1; // Sử dụng CPU nếu không có GPU
        }
    }
}

// Các hàm truy cập state

pub fn get_app_state(state_manager: State<'_, StateManager>) -> Result<AppState, String> {
    let lock_result = state_manager.app.lock();
    match lock_result {
        Ok(app_state) => Ok(app_state.clone()),
        Err(_) => Err("Failed to acquire app state lock".to_string()),
    }
}

pub fn set_selected_gpu(
    gpu_index: i32,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> Result<(), String> {
    let lock_result = state_manager.app.lock();
    let result = match lock_result {
        Ok(mut app_state) => {
            // Kiểm tra xem index có hợp lệ không
            if gpu_index == -1 || (gpu_index >= 0 && (gpu_index as usize) < app_state.gpus.len()) {
                app_state.selected_gpu_index = gpu_index;

                // Emit sự kiện thông báo state đã thay đổi
                let _ = app_handle.emit("app-state-changed", app_state.clone());

                Ok(())
            } else {
                Err(format!("Invalid GPU index: {}", gpu_index))
            }
        },
        Err(_) => Err("Failed to acquire app state lock".to_string()),
    };
    result
}

pub fn get_conversion_state(state_manager: State<'_, StateManager>) -> Result<ConversionState, String> {
    let lock_result = state_manager.conversion.lock();
    match lock_result {
        Ok(conversion_state) => Ok(conversion_state.clone()),
        Err(_) => Err("Failed to acquire conversion state lock".to_string()),
    }
}

pub fn get_preferences(state_manager: State<'_, StateManager>) -> Result<UserPreferencesState, String> {
    let lock_result = state_manager.preferences.lock();
    match lock_result {
        Ok(preferences) => Ok(preferences.clone()),
        Err(_) => Err("Failed to acquire preferences lock".to_string()),
    }
}

pub fn update_preferences(
    new_preferences: UserPreferencesState,
    state_manager: State<'_, StateManager>,
) -> Result<(), String> {
    let lock_result = state_manager.preferences.lock();
    let result = match lock_result {
        Ok(mut preferences) => {
            *preferences = new_preferences.clone();

            // Emit sự kiện thông báo preferences đã thay đổi
            // let _ = app_handle.emit("preferences-changed", new_preferences);

            Ok(())
        },
        Err(_) => Err("Failed to acquire preferences lock".to_string()),
    };
    result
}

pub fn update_conversion_progress(
    task_id: String,
    progress: f32,
    app_handle: AppHandle,
) -> Result<(), String> {
    let state = app_handle.state::<StateManager>();
    let lock_result = state.conversion.lock();
    let result = match lock_result {
        Ok(mut conversion) => {
            // Cập nhật tiến độ cho task hiện tại
            conversion.current_progress = progress;

            // Nếu hoàn thành, chuyển task từ active sang completed
            if progress >= 100.0 {
                if let Some(pos) = conversion.active_tasks.iter().position(|id| id == &task_id) {
                    conversion.active_tasks.remove(pos);
                    conversion.completed_tasks.push(task_id.clone());
                }
            }

            // Emit sự kiện cho frontend
            #[derive(Serialize, Clone)]
            struct Progress {
                task_id: String,
                progress: f32,
            }

            let progress_data = Progress {
                task_id,
                progress,
            };

            // Emit sự kiện cập nhật tiến độ
            let _ = app_handle.emit("conversion-progress", progress_data);

            // Emit sự kiện thông báo conversion state đã thay đổi
            let _ = app_handle.emit("conversion-state-changed", conversion.clone());

            Ok(())
        },
        Err(_) => Err("Failed to acquire conversion state lock".to_string()),
    };
    result
}

pub fn add_conversion_task(
    task_id: String,
    app_handle: AppHandle,
) -> Result<(), String> {
    let state = app_handle.state::<StateManager>();
    let lock_result = state.conversion.lock();
    let result = match lock_result {
        Ok(mut conversion) => {
            conversion.active_tasks.push(task_id);
            conversion.current_progress = 0.0;

            // Emit sự kiện thông báo conversion state đã thay đổi
            let _ = app_handle.emit("conversion-state-changed", conversion.clone());

            Ok(())
        },
        Err(_) => Err("Failed to acquire conversion state lock".to_string()),
    };
    result
}

pub fn mark_task_failed(
    task_id: String,
    app_handle: AppHandle,
) -> Result<(), String> {
    let state = app_handle.state::<StateManager>();
    let lock_result = state.conversion.lock();
    let result = match lock_result {
        Ok(mut conversion) => {
            if let Some(pos) = conversion.active_tasks.iter().position(|id| id == &task_id) {
                conversion.active_tasks.remove(pos);
                conversion.failed_tasks.push(task_id);
            }

            // Emit sự kiện thông báo conversion state đã thay đổi
            let _ = app_handle.emit("conversion-state-changed", conversion.clone());

            Ok(())
        },
        Err(_) => Err("Failed to acquire conversion state lock".to_string()),
    };
    result
}

// File management functions
pub fn add_file_to_list(
    file_info: FileInfo,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> Result<(), String> {
    let lock_result = state_manager.conversion.lock();
    let result = match lock_result {
        Ok(mut conversion) => {
            // Kiểm tra xem file đã tồn tại trong danh sách chưa
            if !conversion.files.iter().any(|f| f.path == file_info.path) {
                conversion.files.push(file_info);

                // Nếu chưa có file nào được chọn, chọn file đầu tiên
                if conversion.selected_file_id.is_none() && !conversion.files.is_empty() {
                    conversion.selected_file_id = Some(conversion.files[0].id.clone());
                }
            }

            // Emit sự kiện thông báo conversion state đã thay đổi
            let _ = app_handle.emit("conversion-state-changed", conversion.clone());

            Ok(())
        },
        Err(_) => Err("Failed to acquire conversion state lock".to_string()),
    };
    result
}

pub fn remove_file_from_list(
    file_id: String,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> Result<(), String> {
    let lock_result = state_manager.conversion.lock();
    let result = match lock_result {
        Ok(mut conversion) => {
            // Tìm vị trí của file trong danh sách
            if let Some(index) = conversion.files.iter().position(|f| f.id == file_id) {
                conversion.files.remove(index);

                // Nếu file bị xóa là file đang được chọn
                if conversion.selected_file_id.as_ref() == Some(&file_id) {
                    // Chọn file đầu tiên trong danh sách nếu còn file nào
                    conversion.selected_file_id = if !conversion.files.is_empty() {
                        Some(conversion.files[0].id.clone())
                    } else {
                        None
                    };
                }
            }

            // Emit sự kiện thông báo conversion state đã thay đổi
            let _ = app_handle.emit("conversion-state-changed", conversion.clone());

            Ok(())
        },
        Err(_) => Err("Failed to acquire conversion state lock".to_string()),
    };
    result
}

pub fn select_file(
    file_id: Option<String>,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> Result<(), String> {
    let lock_result = state_manager.conversion.lock();
    let result = match lock_result {
        Ok(mut conversion) => {
            match file_id {
                Some(id) => {
                    // Kiểm tra xem file có tồn tại trong danh sách không
                    if conversion.files.iter().any(|f| f.id == id) {
                        conversion.selected_file_id = Some(id);
                    } else {
                        return Err(format!("File with id {} not found", id));
                    }
                },
                None => {
                    conversion.selected_file_id = None;
                }
            }

            // Emit sự kiện thông báo conversion state đã thay đổi
            let _ = app_handle.emit("conversion-state-changed", conversion.clone());

            Ok(())
        },
        Err(_) => Err("Failed to acquire conversion state lock".to_string()),
    };
    result
}

pub fn clear_file_list(
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> Result<(), String> {
    let lock_result = state_manager.conversion.lock();
    let result = match lock_result {
        Ok(mut conversion) => {
            conversion.files.clear();
            conversion.selected_file_id = None;

            // Emit sự kiện thông báo conversion state đã thay đổi
            let _ = app_handle.emit("conversion-state-changed", conversion.clone());

            Ok(())
        },
        Err(_) => Err("Failed to acquire conversion state lock".to_string()),
    };
    result
}

// Preferences file operations
pub fn save_preferences_to_file(app_handle: AppHandle) -> Result<(), String> {
    let state = app_handle.state::<StateManager>();
    let preferences_lock = state.preferences.lock();
    let preferences = preferences_lock.map_err(|_| "Failed to acquire preferences lock".to_string())?;
    let preferences_json = serde_json::to_string_pretty(&*preferences)
        .map_err(|e| format!("Failed to serialize preferences: {}", e))?;

    // Lấy đường dẫn đến thư mục cấu hình
    let app_dir = app_handle.path().app_data_dir()
        .map_err(|_| "Failed to get app directory".to_string())?;

    let config_file = app_dir.join("preferences.json");

    // Đảm bảo thư mục tồn tại
    if let Some(parent) = config_file.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    // Lưu file
    std::fs::write(config_file, preferences_json)
        .map_err(|e| format!("Failed to write preferences file: {}", e))?;

    Ok(())
}

pub fn load_preferences_from_file(app_handle: AppHandle) -> Result<(), String> {
    // Lấy đường dẫn đến thư mục cấu hình
    let app_dir = app_handle.path().app_data_dir()
        .map_err(|_| "Failed to get app directory".to_string())?;

    let config_file = app_dir.join("preferences.json");

    // Kiểm tra xem file có tồn tại không
    if !config_file.exists() {
        return Ok(());
    }

    // Đọc file
    let preferences_json = std::fs::read_to_string(config_file)
        .map_err(|e| format!("Failed to read preferences file: {}", e))?;

    // Parse JSON
    let loaded_preferences: UserPreferencesState = serde_json::from_str(&preferences_json)
        .map_err(|e| format!("Failed to parse preferences: {}", e))?;

    // Cập nhật state
    let state = app_handle.state::<StateManager>();
    let preferences_lock = state.preferences.lock();
    let mut preferences = preferences_lock.map_err(|_| "Failed to acquire preferences lock".to_string())?;
    *preferences = loaded_preferences.clone();

    // Emit sự kiện thông báo preferences đã thay đổi
    let _ = app_handle.emit("preferences-changed", loaded_preferences);

    Ok(())
}

// Global state access
pub fn get_global_state(state_manager: State<'_, StateManager>) -> Result<GlobalState, String> {
    let app_lock = state_manager.app.lock();
    let app_state = app_lock.map_err(|_| "Failed to acquire app state lock".to_string())?;

    let conversion_lock = state_manager.conversion.lock();
    let conversion_state = conversion_lock.map_err(|_| "Failed to acquire conversion state lock".to_string())?;

    let preferences_lock = state_manager.preferences.lock();
    let preferences_state = preferences_lock.map_err(|_| "Failed to acquire preferences lock".to_string())?;

    Ok(GlobalState {
        app: app_state.clone(),
        conversion: conversion_state.clone(),
        preferences: preferences_state.clone(),
    })
}
