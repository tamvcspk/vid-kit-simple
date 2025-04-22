pub mod commands;
pub mod services;
pub mod state;
pub mod utils;

use log::info;
use tauri::Manager;
use tauri_plugin_log::{RotationStrategy, Target, TargetKind, TimezoneStrategy};

// Constants for logging configuration
const MAX_LOG_FILE_SIZE_BYTES: u128 = 10 * 1024 * 1024; // 10 MB
const LOG_FILE_NAME: &str = "app";
const LOG_TIMEZONE_STRATEGY: TimezoneStrategy = TimezoneStrategy::UseLocal;
const LOG_ROTATION_STRATEGY: RotationStrategy = RotationStrategy::KeepAll;


use state::StateManager;
use state::task_manager::TaskManager;
use utils::gpu_detector::check_gpu_availability;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // We'll initialize the logger in setup
    println!("Starting application");
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .timezone_strategy(LOG_TIMEZONE_STRATEGY)
                .rotation_strategy(LOG_ROTATION_STRATEGY)
                .max_file_size(MAX_LOG_FILE_SIZE_BYTES)
                .target(Target::new(TargetKind::Stdout))
                .target(Target::new(TargetKind::LogDir {
                    file_name: Some(LOG_FILE_NAME.to_string()),
                }))
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(StateManager::new())
        .manage(TaskManager::new(2)) // Default to 2 concurrent tasks
        .invoke_handler(tauri::generate_handler![
            // Basic commands
            commands::greet,
            // GPU detection
            check_gpu_availability,
            // Preset management has been moved to frontend
            // Video processing
            commands::get_video_info,
            // State management
            // New state management commands
            commands::get_app_info,
            commands::set_gpu,
            commands::emit_preferences_changed,
            commands::emit_conversion_state_changed,

            // DEPRECATED: Legacy state management commands - will be removed in future versions
            commands::get_app_state,
            commands::get_conversion_state,
            commands::get_preferences,
            commands::update_preferences,
            commands::save_preferences_to_file,
            commands::load_preferences_from_file,
            commands::get_global_state,

            // DEPRECATED: Legacy file management commands - will be removed in future versions
            commands::add_file_to_list,
            commands::remove_file_from_list,
            commands::select_file,
            commands::clear_file_list,
            // GPU selection - new command is set_gpu
            // Task management
            // New task management commands
            commands::create_task,
            commands::run_task,
            commands::get_tasks,
            commands::get_queue,
            commands::get_task,
            commands::pause_task,
            commands::resume_task,
            commands::cancel_task,
            commands::retry_task,
            commands::remove_task,
            commands::clear_completed_tasks,
            commands::reorder_tasks,
            commands::pause_queue,
            commands::resume_queue,
            commands::cancel_queue,
            commands::set_max_concurrent_tasks,
            commands::get_max_concurrent_tasks,
            commands::is_queue_paused,
            // Logging
            commands::get_current_log_file_path,
            commands::open_log_file,
            commands::open_log_directory,
        ])
        .setup(|app| {
            // Logger is initialized by the tauri-plugin-log plugin
            let _app_handle = app.app_handle(); // Unused for now

            info!("Application setup starting");

            // Processor state is no longer needed with the new task system

            // Initialize task manager state
            // In a real implementation, we would need to use a mutable reference to TaskManager
            // For now, we'll just log a message
            info!("Skipping task state loading (requires mutable access)");

            // Get state manager after registering all states
            let state_manager = app.state::<StateManager>();

            // Check GPU and get list of GPUs
            let (gpu_available, gpus) = match check_gpu_availability() {
                Ok(gpu_list) => {
                    let is_available = gpu_list.gpus.iter().any(|gpu| gpu.is_available);
                    (is_available, gpu_list.gpus)
                }
                Err(_) => (false, Vec::new()),
            };

            // Store the GPU count for later use
            let gpu_count = gpus.len();

            // Get FFmpeg version (could add a function to video_processor to get version)
            let ffmpeg_version = Some("FFmpeg 7.1.0".to_string()); // Replace with actual function

            // Initialize state
            state_manager.initialize(ffmpeg_version, gpu_available, gpus);

            // Load preferences from file
            let app_handle = app.app_handle().clone();
            let _ = state::load_preferences_from_file(app_handle.clone());

            // Send a startup notification
            utils::event_emitter::emit_success(
                &app_handle,
                "Application started successfully",
                Some(format!(
                    "Application initialized with {} GPUs available",
                    gpu_count
                )),
            );

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
