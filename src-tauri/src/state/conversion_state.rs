use std::collections::HashMap;
use std::path::PathBuf;
use parking_lot::Mutex;
use serde::{Serialize, Deserialize};
use tauri::{Manager, State, AppHandle, Emitter};
use uuid::Uuid;

use crate::state::errors::{StateError, StateResult};
use crate::state::helpers::with_state;

/// Task status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Task information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskState {
    pub id: Uuid,
    pub progress: f32,
    pub status: TaskStatus,
    pub file_id: Option<Uuid>,
    pub output_path: Option<PathBuf>,
    pub error_message: Option<String>,
}

/// Video file information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub id: Uuid,
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub file_type: String,
    pub duration: Option<f64>,
    pub resolution: Option<(u32, u32)>,
    pub thumbnail: Option<String>,
}

/// General state for conversion
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConversionState {
    pub tasks: HashMap<Uuid, TaskState>,
    pub files: Vec<FileInfo>,
    pub selected_file_id: Option<Uuid>,
}

/// ConversionState manager
pub struct ConversionStateManager {
    pub state: Mutex<ConversionState>,
}

impl ConversionStateManager {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(ConversionState {
                tasks: HashMap::new(),
                files: Vec::new(),
                selected_file_id: None,
            }),
        }
    }
}

// Helper function to get active tasks
fn get_active_tasks(state: &ConversionState) -> Vec<Uuid> {
    state.tasks.iter()
        .filter(|(_, task)| task.status == TaskStatus::Running)
        .map(|(id, _)| *id)
        .collect()
}

// Helper function to get completed tasks
fn get_completed_tasks(state: &ConversionState) -> Vec<Uuid> {
    state.tasks.iter()
        .filter(|(_, task)| task.status == TaskStatus::Completed)
        .map(|(id, _)| *id)
        .collect()
}

// Helper function to get failed tasks
fn get_failed_tasks(state: &ConversionState) -> Vec<Uuid> {
    state.tasks.iter()
        .filter(|(_, task)| task.status == TaskStatus::Failed)
        .map(|(id, _)| *id)
        .collect()
}

// State access functions
pub fn get_conversion_state(state_manager: State<'_, ConversionStateManager>) -> StateResult<ConversionState> {
    Ok(state_manager.state.lock().clone())
}

pub fn update_conversion_progress(
    task_id: Uuid,
    progress: f32,
    app_handle: AppHandle,
) -> StateResult<()> {
    let state_manager = app_handle.state::<ConversionStateManager>();

    with_state(&state_manager.state, &app_handle, "conversion-state-changed", |state| {
        // Update task progress
        let task = state.tasks.get_mut(&task_id).ok_or(StateError::task_not_found(task_id))?;
        task.progress = progress;

        // Update task status if completed
        if progress >= 100.0 {
            task.status = TaskStatus::Completed;
        }

        Ok(())
    })?;

    // Prepare progress data for specific event
    #[derive(Serialize, Clone)]
    struct Progress {
        task_id: String,
        progress: f32,
    }

    let progress_data = Progress {
        task_id: task_id.to_string(),
        progress,
    };

    // Emit task-specific progress event
    app_handle.emit("conversion-progress", progress_data)
        .map_err(StateError::from)?;

    Ok(())
}

pub fn add_conversion_task(
    file_id: Option<Uuid>,
    app_handle: AppHandle,
) -> StateResult<Uuid> {
    let state_manager = app_handle.state::<ConversionStateManager>();
    let task_id = Uuid::new_v4();

    with_state(&state_manager.state, &app_handle, "conversion-state-changed", |state| {
        let task = TaskState {
            id: task_id,
            progress: 0.0,
            status: TaskStatus::Pending,
            file_id,
            output_path: None,
            error_message: None,
        };

        state.tasks.insert(task_id, task);
        Ok(())
    })?;

    Ok(task_id)
}

pub fn start_task(
    task_id: Uuid,
    app_handle: AppHandle,
) -> StateResult<()> {
    let state_manager = app_handle.state::<ConversionStateManager>();

    with_state(&state_manager.state, &app_handle, "conversion-state-changed", |state| {
        let task = state.tasks.get_mut(&task_id).ok_or(StateError::task_not_found(task_id))?;
        task.status = TaskStatus::Running;
        Ok(())
    })
}

pub fn mark_task_failed(
    task_id: Uuid,
    error_message: Option<String>,
    app_handle: AppHandle,
) -> StateResult<()> {
    let state_manager = app_handle.state::<ConversionStateManager>();

    with_state(&state_manager.state, &app_handle, "conversion-state-changed", |state| {
        let task = state.tasks.get_mut(&task_id).ok_or(StateError::task_not_found(task_id))?;
        task.status = TaskStatus::Failed;
        task.error_message = error_message;
        Ok(())
    })
}

pub fn mark_task_completed(
    task_id: Uuid,
    output_path: PathBuf,
    app_handle: AppHandle,
) -> StateResult<()> {
    let state_manager = app_handle.state::<ConversionStateManager>();

    with_state(&state_manager.state, &app_handle, "conversion-state-changed", |state| {
        let task = state.tasks.get_mut(&task_id).ok_or(StateError::task_not_found(task_id))?;
        task.status = TaskStatus::Completed;
        task.progress = 100.0;
        task.output_path = Some(output_path);
        Ok(())
    })
}

// File management functions
pub fn add_file_to_list(
    file_info: FileInfo,
    state_manager: &ConversionStateManager,
    app_handle: AppHandle,
) -> StateResult<()> {
    with_state(&state_manager.state, &app_handle, "conversion-state-changed", |state| {
        // Check if the file already exists in the list by path
        if !state.files.iter().any(|f| f.path == file_info.path) {
            state.files.push(file_info);

            // If no file is selected, select the first one
            if state.selected_file_id.is_none() && !state.files.is_empty() {
                state.selected_file_id = Some(state.files[0].id);
            }
        }

        Ok(())
    })
}

pub fn remove_file_from_list(
    file_id: Uuid,
    state_manager: &ConversionStateManager,
    app_handle: AppHandle,
) -> StateResult<()> {
    with_state(&state_manager.state, &app_handle, "conversion-state-changed", |state| {
        // Find the position of the file in the list
        if let Some(index) = state.files.iter().position(|f| f.id == file_id) {
            state.files.remove(index);

            // If the deleted file is the currently selected file
            if state.selected_file_id == Some(file_id) {
                // Select the first file in the list if any files remain
                state.selected_file_id = if !state.files.is_empty() {
                    Some(state.files[0].id)
                } else {
                    None
                };
            }
        } else {
            return Err(StateError::file_not_found(file_id));
        }

        Ok(())
    })
}

pub fn select_file(
    file_id: Option<Uuid>,
    state_manager: &ConversionStateManager,
    app_handle: AppHandle,
) -> StateResult<()> {
    with_state(&state_manager.state, &app_handle, "conversion-state-changed", |state| {
        match file_id {
            Some(id) => {
                // Check if the file exists in the list
                if state.files.iter().any(|f| f.id == id) {
                    state.selected_file_id = Some(id);
                } else {
                    return Err(StateError::file_not_found(id));
                }
            },
            None => {
                state.selected_file_id = None;
            }
        }

        Ok(())
    })
}

pub fn clear_file_list(
    state_manager: &ConversionStateManager,
    app_handle: AppHandle,
) -> StateResult<()> {
    with_state(&state_manager.state, &app_handle, "conversion-state-changed", |state| {
        state.files.clear();
        state.selected_file_id = None;

        Ok(())
    })
}

// Functions to convert String <-> Uuid for backward compatibility
pub fn get_task_id_from_string(task_id_str: &str) -> StateResult<Uuid> {
    Uuid::parse_str(task_id_str).map_err(|_| StateError::other(format!("Invalid task ID: {}", task_id_str)))
}

pub fn get_file_id_from_string(file_id_str: &str) -> StateResult<Uuid> {
    Uuid::parse_str(file_id_str).map_err(|_| StateError::other(format!("Invalid file ID: {}", file_id_str)))
}
