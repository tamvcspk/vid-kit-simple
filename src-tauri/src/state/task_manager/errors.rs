use thiserror::Error;

/// Error types specific to task management
#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Task {0} not found")]
    TaskNotFound(String),

    #[error("Invalid task status: {0}")]
    InvalidStatus(String),

    #[error("Task processing failed: {0}")]
    ProcessingFailed(String),

    #[error("Unsupported task type: {0}")]
    UnsupportedTaskType(String),

    #[error("Failed to save task state: {0}")]
    StoreSaveError(String),

    #[error("Failed to load task state: {0}")]
    StoreLoadError(String),

    #[error("Task was canceled")]
    Canceled,

    #[error("{0}")]
    Other(String),
}

/// Type alias for Result with TaskError
pub type TaskResult<T> = Result<T, TaskError>;
