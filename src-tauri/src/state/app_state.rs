use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};

use crate::state::errors::StateResult;
use crate::state::helpers::with_state;
use crate::utils::gpu_detector::GpuInfo;

// Define AppState structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppState {
    pub is_initialized: bool,
    pub app_version: String,
    pub ffmpeg_version: Option<String>,
    pub gpu_available: bool,
    pub gpus: Vec<GpuInfo>,
    pub selected_gpu_index: i32, // -1 for CPU, 0+ for GPU
}

// AppState manager
pub struct AppStateManager {
    pub state: Mutex<AppState>,
}

impl AppStateManager {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(AppState {
                is_initialized: false,
                app_version: env!("CARGO_PKG_VERSION").to_string(),
                ffmpeg_version: None,
                gpu_available: false,
                gpus: Vec::new(),
                selected_gpu_index: -1, // Default is CPU
            }),
        }
    }

    // Initialize state with default values
    pub fn initialize(
        &self,
        ffmpeg_version: Option<String>,
        gpu_available: bool,
        gpus: Vec<GpuInfo>,
    ) {
        // Get lock
        let mut app = self.state.lock();

        app.is_initialized = true;
        app.ffmpeg_version = ffmpeg_version;
        app.gpu_available = gpu_available;
        app.gpus = gpus;

        // Reset index if needed
        if !app.gpu_available
            || app.selected_gpu_index >= 0
                && (app.selected_gpu_index as usize >= app.gpus.len()
                    || !app
                        .gpus
                        .get(app.selected_gpu_index as usize)
                        .map(|g| g.is_available)
                        .unwrap_or(false))
        {
            app.selected_gpu_index = -1; // Reset to CPU
        }

        // If GPU is available, select the first available GPU
        if app.gpu_available && app.selected_gpu_index == -1 {
            if let Some((i, _)) = app.gpus.iter().enumerate().find(|(_, g)| g.is_available) {
                app.selected_gpu_index = i as i32;
            }
        }
    }
}

// State access functions
pub fn get_app_state(state_manager: State<'_, AppStateManager>) -> StateResult<AppState> {
    Ok(state_manager.state.lock().clone())
}

pub fn set_selected_gpu(
    gpu_index: i32,
    state_manager: State<'_, AppStateManager>,
    app_handle: AppHandle,
) -> StateResult<()> {
    with_state(
        &state_manager.state,
        &app_handle,
        "app-state-changed",
        |app_state| {
            // Check if the index is valid
            if gpu_index == -1 || (gpu_index >= 0 && (gpu_index as usize) < app_state.gpus.len()) {
                app_state.selected_gpu_index = gpu_index;
                Ok(())
            } else {
                Err(crate::state::errors::StateError::invalid_gpu_index(
                    gpu_index,
                ))
            }
        },
    )
}
