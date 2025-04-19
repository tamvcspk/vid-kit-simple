pub mod commands;
pub mod services;
pub mod state;
pub mod utils;

use tauri::Manager;
use log;
use env_logger;

use commands::init_processor_state;
use state::StateManager;
use utils::gpu_detector::check_gpu_availability;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logger
    env_logger::init();
    log::info!("Starting application");
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(StateManager::new())
        .invoke_handler(tauri::generate_handler![
            // Basic commands
            commands::greet,

            // GPU detection
            check_gpu_availability,

            // Preset management
            commands::list_presets,
            commands::get_preset,
            commands::save_preset,
            commands::delete_preset,
            commands::create_default_presets,

            // Video processing
            commands::get_video_info,
            commands::create_processing_task,
            commands::run_processing_task,

            // State management
            commands::get_app_state,
            commands::get_conversion_state,
            commands::get_preferences,
            commands::update_preferences,
            commands::update_conversion_progress_wrapper,
            commands::add_conversion_task_wrapper,
            commands::mark_task_failed_wrapper,
            commands::save_preferences_to_file,
            commands::load_preferences_from_file,
            commands::get_global_state,

            // File management
            commands::add_file_to_list,
            commands::remove_file_from_list,
            commands::select_file,
            commands::clear_file_list,

            // GPU selection
            commands::set_selected_gpu,

            // Task management
            commands::cleanup_video_tasks,
        ])
        .setup(|app| {
            // Initialize processor state with app handle
            let app_handle = app.app_handle();
            let processor_state = init_processor_state(&app_handle);
            app.manage(processor_state);

            // Get state manager after registering all states
            let state_manager = app.state::<StateManager>();

            // Check GPU and get list of GPUs
            let (gpu_available, gpus) = match check_gpu_availability() {
                Ok(gpu_list) => {
                    let is_available = gpu_list.gpus.iter().any(|gpu| gpu.is_available);
                    (is_available, gpu_list.gpus)
                },
                Err(_) => (false, Vec::new()),
            };

            // Store the GPU count for later use
            let gpu_count = gpus.len();

            // Get FFmpeg version (could add a function to video_processor to get version)
            let ffmpeg_version = Some("FFmpeg 6.0".to_string()); // Replace with actual function

            // Initialize state
            state_manager.initialize(ffmpeg_version, gpu_available, gpus);

            // Load preferences from file
            let app_handle = app.app_handle().clone();
            let _ = state::load_preferences_from_file(app_handle.clone());

            // Send a startup notification
            utils::event_emitter::emit_success(
                &app_handle,
                "Application started successfully",
                Some(format!("Application initialized with {} GPUs available", gpu_count))
            );

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
