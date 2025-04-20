use std::io;
use tauri::Error as TauriError;
use thiserror::Error;
use uuid::Uuid;

/// Domain-specific error types for state management
#[derive(Error, Debug)]
pub enum StateError {
    #[error("Task {0} not found")]
    TaskNotFound(Uuid),

    #[error("File {0} not found")]
    FileNotFound(Uuid),

    #[error("Invalid GPU index {0}")]
    InvalidGpuIndex(i32),

    #[error("Failed to emit state change event: {0}")]
    EmitError(String),

    #[error("Failed to serialize state: {0}")]
    SerializationError(String),

    #[error("Failed to deserialize state: {0}")]
    DeserializationError(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Tauri error: {0}")]
    Tauri(#[from] TauriError),

    #[error("{0}")]
    Other(String),
}

/// Helper functions to simplify error creation
impl StateError {
    pub fn task_not_found(id: Uuid) -> Self {
        StateError::TaskNotFound(id)
    }

    pub fn file_not_found(id: Uuid) -> Self {
        StateError::FileNotFound(id)
    }

    pub fn invalid_gpu_index(index: i32) -> Self {
        StateError::InvalidGpuIndex(index)
    }

    pub fn emit_error(err: impl std::fmt::Display) -> Self {
        StateError::EmitError(err.to_string())
    }

    pub fn other(msg: impl Into<String>) -> Self {
        StateError::Other(msg.into())
    }
}

/// Type alias for Result with StateError
pub type StateResult<T> = Result<T, StateError>;
