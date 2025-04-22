use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;

use crate::state::errors::{StateError, StateResult};
use crate::state::helpers::with_state;

// Legacy task status and task state have been removed as they are replaced by the new task system

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
    // tasks field has been removed as it is replaced by the new task system
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
                files: Vec::new(),
                selected_file_id: None,
            }),
        }
    }
}

// State access functions
pub fn get_conversion_state(
    state_manager: State<'_, ConversionStateManager>,
) -> StateResult<ConversionState> {
    Ok(state_manager.state.lock().clone())
}

// Legacy update_conversion_progress function has been removed as it is replaced by the new task system

// Legacy add_conversion_task function has been removed as it is replaced by the new task system

// Legacy start_task function has been removed as it is replaced by the new task system

// Legacy mark_task_failed function has been removed as it is replaced by the new task system

// Legacy mark_task_completed function has been removed as it is replaced by the new task system

// File management functions
pub fn add_file_to_list(
    file_info: FileInfo,
    state_manager: &ConversionStateManager,
    app_handle: AppHandle,
) -> StateResult<()> {
    with_state(
        &state_manager.state,
        &app_handle,
        "conversion-state-changed",
        |state| {
            // Check if the file already exists in the list by path
            if !state.files.iter().any(|f| f.path == file_info.path) {
                state.files.push(file_info);

                // If no file is selected, select the first one
                if state.selected_file_id.is_none() && !state.files.is_empty() {
                    state.selected_file_id = Some(state.files[0].id);
                }
            }

            Ok(())
        },
    )
}

pub fn remove_file_from_list(
    file_id: Uuid,
    state_manager: &ConversionStateManager,
    app_handle: AppHandle,
) -> StateResult<()> {
    with_state(
        &state_manager.state,
        &app_handle,
        "conversion-state-changed",
        |state| {
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
        },
    )
}

pub fn select_file(
    file_id: Option<Uuid>,
    state_manager: &ConversionStateManager,
    app_handle: AppHandle,
) -> StateResult<()> {
    with_state(
        &state_manager.state,
        &app_handle,
        "conversion-state-changed",
        |state| {
            match file_id {
                Some(id) => {
                    // Check if the file exists in the list
                    if state.files.iter().any(|f| f.id == id) {
                        state.selected_file_id = Some(id);
                    } else {
                        return Err(StateError::file_not_found(id));
                    }
                }
                None => {
                    state.selected_file_id = None;
                }
            }

            Ok(())
        },
    )
}

pub fn clear_file_list(
    state_manager: &ConversionStateManager,
    app_handle: AppHandle,
) -> StateResult<()> {
    with_state(
        &state_manager.state,
        &app_handle,
        "conversion-state-changed",
        |state| {
            state.files.clear();
            state.selected_file_id = None;

            Ok(())
        },
    )
}

// Function to convert String -> Uuid for backward compatibility
pub fn get_file_id_from_string(file_id_str: &str) -> StateResult<Uuid> {
    Uuid::parse_str(file_id_str)
        .map_err(|_| StateError::other(format!("Invalid file ID: {}", file_id_str)))
}
