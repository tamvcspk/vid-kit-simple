//! # State Management Module
//!
//! This module provides a centralized state management system for the application.
//! It handles storing, updating, and synchronizing application state across
//! different components and between the frontend and backend.
//!
//! ## State Components
//!
//! - `app_state`: Core application state (GPU info, FFmpeg version, etc.)
//! - `conversion_state`: State related to video conversion tasks and file list
//! - `preferences_state`: User preferences and settings
//! - `errors`: Error types specific to state operations
//! - `helpers`: Helper functions for state management
//!
//! ## Usage
//!
//! The state system uses Tauri's state management with thread-safe wrappers
//! around state objects. State changes are propagated to the frontend using
//! Tauri events.

/// Core application state including GPU information and FFmpeg version
pub mod app_state;

/// Conversion state including tasks, progress tracking, and file list
pub mod conversion_state;

/// Error types and utilities specific to state operations
pub mod errors;

/// Helper functions for state management operations
pub mod helpers;

/// User preferences and application settings state
pub mod preferences_state;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};

use app_state::{AppState, AppStateManager};
use conversion_state::{ConversionState, ConversionStateManager};
use errors::{StateError, StateResult};
use helpers::with_state;
use preferences_state::{PreferencesStateManager, UserPreferencesState};

/// Combined state object that includes all application state components
///
/// This struct is used to send the complete application state to the frontend
/// in a single operation. It combines app state, conversion state, and user
/// preferences into a single serializable object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalState {
    /// Core application state including GPU information
    pub app: AppState,

    /// Conversion tasks and file list state
    pub conversion: ConversionState,

    /// User preferences and settings
    pub preferences: UserPreferencesState,
}

/// Main state manager that coordinates all state components
///
/// This struct serves as the central state management system for the application.
/// It contains individual state managers for different aspects of the application
/// and provides methods to initialize and access state.
pub struct StateManager {
    /// Application state manager for core app information
    pub app: AppStateManager,

    /// Conversion state manager for tasks and file list
    pub conversion: ConversionStateManager,

    /// User preferences state manager for settings
    pub preferences: PreferencesStateManager,
}

impl StateManager {
    /// Creates a new StateManager with default values for all state components
    pub fn new() -> Self {
        Self {
            app: AppStateManager::new(),
            conversion: ConversionStateManager::new(),
            preferences: PreferencesStateManager::new(),
        }
    }

    /// Initializes the application state with system information
    ///
    /// # Parameters
    /// * `ffmpeg_version` - The detected FFmpeg version string
    /// * `gpu_available` - Whether GPU acceleration is available
    /// * `gpus` - List of detected GPUs and their capabilities
    pub fn initialize(
        &self,
        ffmpeg_version: Option<String>,
        gpu_available: bool,
        gpus: Vec<crate::utils::gpu_detector::GpuInfo>,
    ) {
        self.app.initialize(ffmpeg_version, gpu_available, gpus);
    }
}

/// Retrieves the complete application state as a single GlobalState object
///
/// This function combines all individual state components into a single GlobalState
/// object that can be sent to the frontend. It acquires locks on each state component
/// in a specific order to avoid deadlocks.
///
/// # Parameters
/// * `state_manager` - The application's StateManager instance
///
/// # Returns
/// * `StateResult<GlobalState>` - The combined state or an error
pub fn get_global_state(state_manager: State<'_, StateManager>) -> StateResult<GlobalState> {
    // Initialize locks non-simultaneously to avoid deadlock
    let app_state = state_manager.app.state.lock().clone();
    let conversion_state = state_manager.conversion.state.lock().clone();
    let preferences_state = state_manager.preferences.state.lock().clone();

    // Combine all states
    Ok(GlobalState {
        app: app_state,
        conversion: conversion_state,
        preferences: preferences_state,
    })
}

// Re-export functions from individual state modules
// Wrapper functions for individual state modules

// App state wrappers
pub fn get_app_state(state_manager: State<'_, StateManager>) -> StateResult<app_state::AppState> {
    Ok(state_manager.app.state.lock().clone())
}

pub fn set_selected_gpu(
    gpu_index: i32,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> StateResult<()> {
    with_state(&state_manager.app.state, &app_handle, "app-state-changed", |app_state| {
        // Check if the index is valid
        if gpu_index == -1 || (gpu_index >= 0 && (gpu_index as usize) < app_state.gpus.len()) {
            app_state.selected_gpu_index = gpu_index;
            Ok(())
        } else {
            Err(StateError::invalid_gpu_index(gpu_index))
        }
    })
}

// Conversion state wrappers
pub fn get_conversion_state(state_manager: State<'_, StateManager>) -> StateResult<conversion_state::ConversionState> {
    Ok(state_manager.conversion.state.lock().clone())
}

pub fn update_conversion_progress(
    task_id: String,
    progress: f32,
    app_handle: AppHandle,
) -> StateResult<()> {
    // Convert from String to Uuid for backward compatibility
    let task_uuid = conversion_state::get_task_id_from_string(&task_id)?;
    conversion_state::update_conversion_progress(task_uuid, progress, app_handle)
}

/// Adds a new conversion task to the state
///
/// This function creates a new task in the conversion state and associates it
/// with the given task ID. It emits a state change event to notify the frontend.
///
/// # Parameters
/// * `task_id` - String identifier for the task (for backward compatibility)
/// * `app_handle` - Tauri AppHandle for accessing state and emitting events
///
/// # Returns
/// * `StateResult<()>` - Success or an error
pub fn add_conversion_task(
    task_id: String, // Keep this parameter for backward compatibility
    app_handle: AppHandle,
) -> StateResult<()> {
    // Convert task_id from String to Uuid
    let task_uuid = match uuid::Uuid::parse_str(&task_id) {
        Ok(uuid) => Some(uuid),
        Err(_) => None, // If parsing fails, assume null
    };

    // Get the state manager from the app handle
    let state_manager = app_handle.state::<ConversionStateManager>();

    // Create a new task in the conversion state
    with_state(&state_manager.state, &app_handle, "conversion-state-changed", |state| {
        let task = conversion_state::TaskState {
            id: uuid::Uuid::new_v4(),
            progress: 0.0,
            status: conversion_state::TaskStatus::Pending,
            file_id: task_uuid,
            output_path: None,
            error_message: None,
        };

        state.tasks.insert(task.id, task.clone());
        Ok(()) // Return unit type to match the expected return type
    })
}

pub fn mark_task_failed(
    task_id: String,
    app_handle: AppHandle,
) -> StateResult<()> {
    let task_uuid = conversion_state::get_task_id_from_string(&task_id)?;
    conversion_state::mark_task_failed(task_uuid, None, app_handle)
}

pub fn add_file_to_list(
    file_info: conversion_state::FileInfo,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> StateResult<()> {
    conversion_state::add_file_to_list(file_info, &state_manager.conversion, app_handle)
}

pub fn remove_file_from_list(
    file_id: String,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> StateResult<()> {
    let file_uuid = conversion_state::get_file_id_from_string(&file_id)?;
    conversion_state::remove_file_from_list(file_uuid, &state_manager.conversion, app_handle)
}

pub fn select_file(
    file_id: Option<String>,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> StateResult<()> {
    let file_uuid = match file_id {
        Some(id) => Some(conversion_state::get_file_id_from_string(&id)?),
        None => None,
    };
    conversion_state::select_file(file_uuid, &state_manager.conversion, app_handle)
}

pub fn clear_file_list(
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> StateResult<()> {
    conversion_state::clear_file_list(&state_manager.conversion, app_handle)
}

// Preferences wrappers
pub fn get_preferences(state_manager: State<'_, StateManager>) -> StateResult<preferences_state::UserPreferencesState> {
    Ok(state_manager.preferences.state.lock().clone())
}

pub fn update_preferences(
    new_preferences: preferences_state::UserPreferencesState,
    state_manager: State<'_, StateManager>,
    app_handle: AppHandle,
) -> StateResult<()> {
    with_state(&state_manager.preferences.state, &app_handle, "preferences-changed", |preferences| {
        // Update preferences
        *preferences = new_preferences.clone();
        Ok(())
    })
}

pub fn save_preferences_to_file(app_handle: AppHandle) -> StateResult<()> {
    preferences_state::save_preferences_to_file(app_handle)
}

pub fn load_preferences_from_file(app_handle: AppHandle) -> StateResult<()> {
    preferences_state::load_preferences_from_file(app_handle)
}

// Re-export types from individual state modules
pub use conversion_state::FileInfo;
