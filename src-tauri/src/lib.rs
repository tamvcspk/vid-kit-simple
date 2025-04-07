pub mod gpu_detector;
pub mod preset_manager;

use gpu_detector::check_gpu_availability;
use preset_manager::{list_presets, get_preset, save_preset, delete_preset, create_default_presets};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            check_gpu_availability,
            list_presets,
            get_preset,
            save_preset,
            delete_preset,
            create_default_presets
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
