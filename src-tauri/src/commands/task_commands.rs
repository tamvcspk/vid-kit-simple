use std::collections::HashMap;
use tauri::{AppHandle, State};

use crate::state::task_manager::{TaskManager, Task};
use crate::utils::error::{ErrorCode, ErrorInfo};
use crate::utils::error_handler::handle_error_with_event;

/// Create a new task
#[tauri::command]
pub fn create_task(
    input_path: String,
    output_path: String,
    config: HashMap<String, String>,
    task_type: String,
    _app_handle: AppHandle,
    task_manager: State<'_, TaskManager>,
) -> Result<String, ErrorInfo> {
    // Validate task type
    if !["convert", "split", "edit", "sanitize"].contains(&task_type.as_str()) {
        return Err(ErrorInfo {
            code: ErrorCode::InvalidArgument,
            message: format!("Invalid task type: {}", task_type),
            details: Some("Task type must be one of: convert, split, edit, sanitize".to_string()),
        });
    }

    // Create task
    let manager = task_manager.inner();
    match manager.create_task(input_path, output_path, task_type, config) {
        Ok(task_id) => Ok(task_id),
        Err(e) => {
            Err(ErrorInfo {
                code: ErrorCode::StateMutationError,
                message: format!("Failed to create task: {}", e),
                details: Some("Error creating task in task manager".to_string()),
            })
        }
    }
}

/// Run a task
#[tauri::command]
pub fn run_task(
    task_id: String,
    app_handle: AppHandle,
    task_manager: State<'_, TaskManager>,
) -> Result<(), ErrorInfo> {
    // Start task
    let manager = task_manager.inner();
    handle_error_with_event(
        manager.start_task(&task_id, &app_handle),
        &app_handle
    )
}

/// Get all tasks
#[tauri::command]
pub fn get_tasks(
    task_manager: State<'_, TaskManager>,
) -> Result<Vec<Task>, ErrorInfo> {
    Ok(task_manager.inner().get_all_tasks())
}

/// Get task queue
#[tauri::command]
pub fn get_queue(
    task_manager: State<'_, TaskManager>,
) -> Result<Vec<String>, ErrorInfo> {
    Ok(task_manager.inner().get_queue())
}

/// Get a task by ID
#[tauri::command]
pub fn get_task(
    task_id: String,
    task_manager: State<'_, TaskManager>,
) -> Result<Task, ErrorInfo> {
    // Get task
    match task_manager.inner().get_task(&task_id) {
        Ok(task) => Ok(task),
        Err(e) => {
            Err(ErrorInfo {
                code: ErrorCode::TaskNotFound,
                message: format!("Task not found: {}", e),
                details: Some(format!("Task with ID {} not found", task_id)),
            })
        }
    }
}

/// Pause a task
#[tauri::command]
pub fn pause_task(
    task_id: String,
    app_handle: AppHandle,
    task_manager: State<'_, TaskManager>,
) -> Result<(), ErrorInfo> {
    // Pause task
    let manager = task_manager.inner();
    handle_error_with_event(
        manager.pause_task(&task_id, &app_handle),
        &app_handle
    )
}

/// Resume a task
#[tauri::command]
pub fn resume_task(
    task_id: String,
    app_handle: AppHandle,
    task_manager: State<'_, TaskManager>,
) -> Result<(), ErrorInfo> {
    // Resume task
    let manager = task_manager.inner();
    handle_error_with_event(
        manager.resume_task(&task_id, &app_handle),
        &app_handle
    )
}

/// Cancel a task
#[tauri::command]
pub fn cancel_task(
    task_id: String,
    app_handle: AppHandle,
    task_manager: State<'_, TaskManager>,
) -> Result<(), ErrorInfo> {
    // Cancel task
    let manager = task_manager.inner();
    handle_error_with_event(
        manager.cancel_task(&task_id, &app_handle),
        &app_handle
    )
}

/// Retry a task
#[tauri::command]
pub fn retry_task(
    task_id: String,
    app_handle: AppHandle,
    task_manager: State<'_, TaskManager>,
) -> Result<(), ErrorInfo> {
    // Retry task
    let manager = task_manager.inner();
    handle_error_with_event(
        manager.retry_task(&task_id, &app_handle),
        &app_handle
    )
}

/// Remove a task
#[tauri::command]
pub fn remove_task(
    task_id: String,
    app_handle: AppHandle,
    task_manager: State<'_, TaskManager>,
) -> Result<(), ErrorInfo> {
    // Remove task
    let manager = task_manager.inner();
    handle_error_with_event(
        manager.remove_task(&task_id, &app_handle),
        &app_handle
    )
}

/// Clear completed tasks
#[tauri::command]
pub fn clear_completed_tasks(
    app_handle: AppHandle,
    task_manager: State<'_, TaskManager>,
) -> Result<(), ErrorInfo> {
    // Clear completed tasks
    let manager = task_manager.inner();
    handle_error_with_event(
        manager.clear_completed_tasks(&app_handle),
        &app_handle
    )
}

/// Reorder tasks
#[tauri::command]
pub fn reorder_tasks(
    new_order: Vec<String>,
    app_handle: AppHandle,
    task_manager: State<'_, TaskManager>,
) -> Result<(), ErrorInfo> {
    // Reorder tasks
    let manager = task_manager.inner();
    handle_error_with_event(
        manager.reorder_tasks(new_order, &app_handle),
        &app_handle
    )
}

/// Pause the task queue
#[tauri::command]
pub fn pause_queue(
    app_handle: AppHandle,
    task_manager: State<'_, TaskManager>,
) -> Result<(), ErrorInfo> {
    // Pause queue
    let manager = task_manager.inner();
    handle_error_with_event(
        manager.pause_queue(&app_handle),
        &app_handle
    )
}

/// Resume the task queue
#[tauri::command]
pub fn resume_queue(
    app_handle: AppHandle,
    task_manager: State<'_, TaskManager>,
) -> Result<(), ErrorInfo> {
    // Resume queue
    let manager = task_manager.inner();
    handle_error_with_event(
        manager.resume_queue(&app_handle),
        &app_handle
    )
}

/// Cancel all tasks in the queue
#[tauri::command]
pub fn cancel_queue(
    app_handle: AppHandle,
    task_manager: State<'_, TaskManager>,
) -> Result<(), ErrorInfo> {
    // Cancel queue
    let manager = task_manager.inner();
    handle_error_with_event(
        manager.cancel_queue(&app_handle),
        &app_handle
    )
}

/// Set the maximum number of concurrent tasks
#[tauri::command]
pub fn set_max_concurrent_tasks(
    max: usize,
    app_handle: AppHandle,
    task_manager: State<'_, TaskManager>,
) -> Result<(), ErrorInfo> {
    // Set max concurrent tasks
    let manager = task_manager.inner();
    handle_error_with_event(
        manager.set_max_concurrent_tasks(max, &app_handle),
        &app_handle
    )
}

/// Get the maximum number of concurrent tasks
#[tauri::command]
pub fn get_max_concurrent_tasks(
    task_manager: State<'_, TaskManager>,
) -> Result<usize, ErrorInfo> {
    Ok(task_manager.inner().get_max_concurrent_tasks())
}

/// Check if the queue is paused
#[tauri::command]
pub fn is_queue_paused(
    task_manager: State<'_, TaskManager>,
) -> Result<bool, ErrorInfo> {
    Ok(task_manager.inner().is_queue_paused())
}
