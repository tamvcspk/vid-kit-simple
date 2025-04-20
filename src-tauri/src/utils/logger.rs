use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tauri_plugin_opener::open_path;

/// Get the logs directory path using Tauri's app_log_dir
///
/// This function returns the path to the logs directory without creating it.
/// It uses Tauri's app_log_dir which follows platform standards:
/// - Linux: $XDG_DATA_HOME/{bundleIdentifier}/logs or $HOME/.local/share/{bundleIdentifier}/logs
/// - macOS: {homeDir}/Library/Logs/{bundleIdentifier}
/// - Windows: {FOLDERID_LocalAppData}/{bundleIdentifier}/logs
///
/// # Arguments
/// * `app_handle` - The Tauri application handle
///
/// # Returns
/// * `Result<PathBuf, String>` - The path to the logs directory if successful, or an error
fn get_logs_directory_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    // Sử dụng đường dẫn log tiêu chuẩn của Tauri Logging plugin
    // Đường dẫn này sẽ tự động được điều chỉnh theo nền tảng
    let app_data_dir = app_handle
        .path()
        .app_log_dir()
        .map_err(|e| format!("Failed to get app log directory: {}", e))?;

    Ok(app_data_dir)
}

// ensure_logs_directory function removed as it's no longer needed with tauri-plugin-log
// The plugin automatically creates the log directory

/// Get the path to the current log file
///
/// This function returns the path to the current log file based on the Tauri Logging plugin's
/// standard paths and file naming conventions.
///
/// # Arguments
/// * `app_handle` - The Tauri application handle
///
/// # Returns
/// * `Result<PathBuf, String>` - The path to the current log file or an error
pub fn get_current_log_file_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let logs_dir = get_logs_directory_path(app_handle)?;

    // Return the main log file path
    let log_file = logs_dir.join("app.log");

    // Check if the main log file exists
    if !log_file.exists() {
        // Check if any rotated log files exist
        for i in 1..=5 {
            let rotated_file = logs_dir.join(format!("app_{}.log", i));
            if rotated_file.exists() {
                return Ok(rotated_file);
            }
        }
    }

    Ok(log_file)
}

/// Open the current log file in the default text editor using tauri_plugin_opener
///
/// # Arguments
/// * `app_handle` - The Tauri application handle
///
/// # Returns
/// * `Result<bool, String>` - True if the log file was opened successfully, or an error
pub fn open_log_file(app_handle: &AppHandle) -> Result<bool, String> {
    // Get the log file path without checking if it exists
    let log_file_path = get_current_log_file_path(app_handle)?;

    // Check if the file exists
    if !log_file_path.exists() {
        return Err(format!(
            "Log file does not exist at: {}",
            log_file_path.display()
        ));
    }

    // Open the log file with tauri_plugin_opener
    open_path(log_file_path, None::<&str>)
        .map_err(|e| format!("Failed to open log file: {}", e))?;

    Ok(true)
}

/// Open the log directory in the file explorer using tauri_plugin_opener
///
/// # Arguments
/// * `app_handle` - The Tauri application handle
///
/// # Returns
/// * `Result<bool, String>` - True if the log directory was opened successfully, or an error
pub fn open_log_directory(app_handle: &AppHandle) -> Result<bool, String> {
    let logs_dir = get_logs_directory_path(app_handle)?;

    // Create directory if it doesn't exist
    if !logs_dir.exists() {
        fs::create_dir_all(&logs_dir)
            .map_err(|e| format!("Failed to create logs directory: {}", e))?;
    }

    // Open the log directory with tauri_plugin_opener
    open_path(logs_dir, None::<&str>)
        .map_err(|e| format!("Failed to open log directory: {}", e))?;

    Ok(true)
}
