use std::path::PathBuf;
use parking_lot::Mutex;
use serde::{Serialize, Deserialize};
use tauri::{Manager, State, AppHandle};
use log::{info};

use crate::state::errors::{StateError, StateResult};
use crate::state::helpers::with_state;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserPreferencesState {
    pub default_output_dir: Option<PathBuf>,
    pub default_format: String,
    pub use_gpu: bool,
    pub theme: String,
}

// PreferencesState manager
pub struct PreferencesStateManager {
    pub state: Mutex<UserPreferencesState>,
}

impl PreferencesStateManager {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(UserPreferencesState {
                default_output_dir: None,
                default_format: "mp4".to_string(),
                use_gpu: false,
                theme: "light".to_string(),
            }),
        }
    }
}

// State access functions
pub fn get_preferences(state_manager: State<'_, PreferencesStateManager>) -> StateResult<UserPreferencesState> {
    Ok(state_manager.state.lock().clone())
}

pub fn update_preferences(
    new_preferences: UserPreferencesState,
    state_manager: State<'_, PreferencesStateManager>,
    app_handle: AppHandle,
) -> StateResult<()> {
    with_state(&state_manager.state, &app_handle, "preferences-changed", |preferences| {
        // Update preferences
        *preferences = new_preferences.clone();
        Ok(())
    })
}

// Preferences file operations
pub fn save_preferences_to_file(app_handle: AppHandle) -> StateResult<()> {
    let state = app_handle.state::<PreferencesStateManager>();
    let preferences = state.state.lock();

    // Serialize preferences
    let preferences_json = serde_json::to_string_pretty(&*preferences)
        .map_err(|e| StateError::other(format!("Failed to serialize preferences: {}", e)))?;

    // Get path to configuration directory
    let app_dir = app_handle.path().app_data_dir()
        .map_err(|e| StateError::other(format!("Failed to get app directory: {}", e)))?;

    let config_file = app_dir.join("preferences.json");

    // Ensure directory exists
    if let Some(parent) = config_file.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Save file
    std::fs::write(&config_file, preferences_json)?;

    info!("Preferences saved to {}", config_file.display());
    Ok(())
}

pub fn load_preferences_from_file(app_handle: AppHandle) -> StateResult<()> {
    // Get path to configuration directory
    let app_dir = app_handle.path().app_data_dir()
        .map_err(|e| StateError::other(format!("Failed to get app directory: {}", e)))?;

    let config_file = app_dir.join("preferences.json");

    // Check if file exists
    if !config_file.exists() {
        info!("Preferences file does not exist, using defaults");
        return Ok(());
    }

    // Read file
    let preferences_json = std::fs::read_to_string(&config_file)?;

    // Parse JSON
    let loaded_preferences: UserPreferencesState = serde_json::from_str(&preferences_json)
        .map_err(|e| StateError::other(format!("Failed to parse preferences: {}", e)))?;

    // Update state and emit event
    let state = app_handle.state::<PreferencesStateManager>();
    
    with_state(&state.state, &app_handle, "preferences-changed", |preferences| {
        *preferences = loaded_preferences.clone();
        Ok(())
    })?;

    info!("Preferences loaded from {}", config_file.display());
    Ok(())
}
