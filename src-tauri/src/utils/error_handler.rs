use crate::utils::error::{AppError, ErrorInfo};

/// Convert any error to a serializable error info for frontend consumption
pub fn to_error_info<E: Into<AppError>>(error: E) -> ErrorInfo {
    let app_error: AppError = error.into();
    app_error.log();
    app_error.to_error_info()
}

/// Helper function to convert AppResult to Result<T, ErrorInfo> for Tauri commands
pub fn handle_error<T, E: Into<AppError>>(result: Result<T, E>) -> Result<T, ErrorInfo> {
    result.map_err(|e| to_error_info(e))
}

/// Helper function to convert AppResult to Result<T, String> for legacy Tauri commands
pub fn handle_error_string<T, E: Into<AppError>>(result: Result<T, E>) -> Result<T, String> {
    result.map_err(|e| {
        let app_error: AppError = e.into();
        app_error.log();
        app_error.to_string()
    })
}

/// Helper macro to handle errors in Tauri commands
#[macro_export]
macro_rules! handle_command {
    ($expr:expr) => {
        match $expr {
            Ok(val) => Ok(val),
            Err(err) => {
                let app_error: $crate::utils::error::AppError = err.into();
                app_error.log();
                Err(app_error.to_error_info())
            }
        }
    };
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
