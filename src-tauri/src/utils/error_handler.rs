use crate::utils::error::{AppError, ErrorCode, ErrorInfo};
use tauri::AppHandle;

/// Convert any error to a serializable error info for frontend consumption
pub fn to_error_info<E: Into<AppError>>(error: E) -> ErrorInfo {
    let app_error: AppError = error.into();
    app_error.log();
    app_error.to_error_info()
}

/// Convert a string error to ErrorInfo
pub fn string_to_error_info(error: String) -> ErrorInfo {
    ErrorInfo {
        code: ErrorCode::UnknownError,
        message: error.clone(),
        details: Some(format!("String error: {}", error)),
    }
}

/// Helper function to convert AppResult to Result<T, ErrorInfo> for Tauri commands
pub fn handle_error<T, E: Into<AppError>>(result: Result<T, E>) -> Result<T, ErrorInfo> {
    result.map_err(to_error_info)
}

/// Handle error and emit event to frontend
pub fn handle_error_with_event<T, E: Into<AppError>>(
    result: Result<T, E>,
    app_handle: &AppHandle,
) -> Result<T, ErrorInfo> {
    result.map_err(|e| {
        let app_error: AppError = e.into();
        app_error.log_with_event(app_handle);
        app_error.to_error_info()
    })
}

/// Helper macro to handle errors in Tauri commands
#[macro_export]
macro_rules! handle_command {
    ($expr:expr) => {
        match $expr {
            Ok(val) => Ok(val),
            Err(err) => Err($crate::utils::error_handler::to_error_info(err)),
        }
    };
}

/// Helper function to convert AppResult to Result<T, String> for legacy Tauri commands
pub fn handle_error_string<T, E: Into<AppError>>(result: Result<T, E>) -> Result<T, String> {
    result.map_err(|e| {
        let app_error: AppError = e.into();
        app_error.log();
        app_error.to_string()
    })
}

/// Helper macro to handle errors in legacy Tauri commands (returning String errors)
#[macro_export]
macro_rules! handle_command_string {
    ($expr:expr) => {
        match $expr {
            Ok(val) => Ok(val),
            Err(err) => {
                let app_error: $crate::utils::error::AppError = err.into();
                app_error.log();
                Err(app_error.to_string())
            }
        }
    };
}

/// Helper macro to convert Result<T, String> to Result<T, ErrorInfo>
#[macro_export]
macro_rules! handle_string_as_error_info {
    ($expr:expr) => {
        match $expr {
            Ok(val) => Ok(val),
            Err(err) => Err($crate::utils::error_handler::string_to_error_info(
                err.to_string(),
            )),
        }
    };
}

/// Helper macro to handle errors in Tauri commands with event emission
#[macro_export]
macro_rules! handle_command_with_event {
    ($expr:expr, $app_handle:expr) => {
        match $expr {
            Ok(val) => Ok(val),
            Err(err) => {
                let app_error: $crate::utils::error::AppError = err.into();
                app_error.log_with_event($app_handle);
                Err(app_error.to_error_info())
            }
        }
    };
}

/// Helper macro to handle string errors in Tauri commands with event emission
#[macro_export]
macro_rules! handle_string_with_event {
    ($expr:expr, $app_handle:expr) => {
        match $expr {
            Ok(val) => Ok(val),
            Err(err) => {
                let error_str = err.to_string();
                let app_error = $crate::utils::error::AppError::from(error_str);
                app_error.log_with_event($app_handle);
                Err(app_error.to_error_info())
            }
        }
    };
}
