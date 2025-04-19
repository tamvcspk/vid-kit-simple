use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use std::process::Command;



/// Get the logs directory path
///
/// This function returns the path to the logs directory without creating it.
///
/// # Arguments
/// * `app_handle` - The Tauri application handle, used to get the app data directory
///
/// # Returns
/// * `Result<PathBuf, String>` - The path to the logs directory if successful, or an error
fn get_logs_directory_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    // Get the app data directory
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    // Return logs directory path
    Ok(app_data_dir.join("logs"))
}

/// Ensure the logs directory exists
///
/// This function creates the logs directory if it doesn't exist.
/// It's called before initializing the logger to ensure the directory exists.
///
/// # Arguments
/// * `app_handle` - The Tauri application handle, used to get the app data directory
///
/// # Returns
/// * `Result<PathBuf, String>` - The path to the logs directory if successful, or an error
pub fn ensure_logs_directory(app_handle: &AppHandle) -> Result<PathBuf, String> {
    // Get app data directory
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    // Get logs directory path
    let logs_dir = get_logs_directory_path(app_handle)?;

    // Create logs directory
    fs::create_dir_all(&logs_dir)
        .map_err(|e| format!("Failed to create logs directory: {}", e))?;

    // Set APP_DATA_DIR environment variable for log4rs
    std::env::set_var("APP_DATA_DIR", app_data_dir.to_string_lossy().to_string());

    Ok(logs_dir)
}

/// Get the path to the current log file
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

/// Open the current log file in the default text editor
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
        return Err(format!("Log file does not exist at: {}", log_file_path.display()));
    }

    // Open the log file with the default application
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/c", "start", "", log_file_path.to_string_lossy().as_ref()])
            .spawn()
            .map_err(|e| format!("Failed to open log file: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(log_file_path.to_string_lossy().as_ref())
            .spawn()
            .map_err(|e| format!("Failed to open log file: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(log_file_path.to_string_lossy().as_ref())
            .spawn()
            .map_err(|e| format!("Failed to open log file: {}", e))?;
    }

    Ok(true)
}

/// Open the log directory in the file explorer
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

    // Open the log directory with the default file explorer
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(logs_dir.to_string_lossy().as_ref())
            .spawn()
            .map_err(|e| format!("Failed to open log directory: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(logs_dir.to_string_lossy().as_ref())
            .spawn()
            .map_err(|e| format!("Failed to open log directory: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(logs_dir.to_string_lossy().as_ref())
            .spawn()
            .map_err(|e| format!("Failed to open log directory: {}", e))?;
    }

    Ok(true)
}
