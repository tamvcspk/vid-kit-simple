use crate::utils::error::AppError;
use serde::Serialize;
use tauri::{AppHandle, Emitter};

/// Emit an error event to the frontend
pub fn emit_error(app_handle: &AppHandle, error: &AppError) {
    // Convert to ErrorInfo for serialization
    let error_info = error.to_error_info();

    // Emit the error event
    if let Err(e) = app_handle.emit("backend-error", error_info) {
        log::error!("Failed to emit error event: {}", e);
    }
}

/// Emit a notification event to the frontend
#[derive(Serialize, Clone)]
pub struct NotificationEvent {
    pub level: String,
    pub message: String,
    pub details: Option<String>,
}

/// Emit a notification event to the frontend
pub fn emit_notification(
    app_handle: &AppHandle,
    level: &str,
    message: &str,
    details: Option<String>,
) {
    let notification = NotificationEvent {
        level: level.to_string(),
        message: message.to_string(),
        details,
    };

    if let Err(e) = app_handle.emit("backend-notification", notification) {
        log::error!("Failed to emit notification event: {}", e);
    }
}

/// Emit an info notification
pub fn emit_info(app_handle: &AppHandle, message: &str, details: Option<String>) {
    emit_notification(app_handle, "info", message, details);
}

/// Emit a success notification
pub fn emit_success(app_handle: &AppHandle, message: &str, details: Option<String>) {
    emit_notification(app_handle, "success", message, details);
}

/// Emit a warning notification
pub fn emit_warning(app_handle: &AppHandle, message: &str, details: Option<String>) {
    emit_notification(app_handle, "warn", message, details);
}
