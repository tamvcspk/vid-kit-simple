use std::path::PathBuf;
use thiserror::Error;

/// Specialized errors for VideoProcessor
#[derive(Error, Debug)]
pub enum VideoError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("FFmpeg error: {0}")]
    Ffmpeg(String),

    #[error("Task {0} not found")]
    TaskNotFound(String),

    #[error("Video stream not found in {0}")]
    NoVideoStream(PathBuf),

    #[error("Codec error: {0}")]
    Codec(String),

    #[error("Encoder error: {0}")]
    Encoder(String),

    #[error("Decoder error: {0}")]
    Decoder(String),

    #[error("Invalid parameter: {0}")]
    InvalidParam(String),

    #[error("Thread pool error: {0}")]
    ThreadPool(String),

    #[error("State error: {0}")]
    State(#[from] crate::state::errors::StateError),

    #[error("{0}")]
    Other(String),
}

/// Helper methods for VideoError
impl VideoError {
    pub fn ffmpeg(msg: impl Into<String>) -> Self {
        VideoError::Ffmpeg(msg.into())
    }

    pub fn codec(msg: impl Into<String>) -> Self {
        VideoError::Codec(msg.into())
    }

    pub fn encoder(msg: impl Into<String>) -> Self {
        VideoError::Encoder(msg.into())
    }

    pub fn decoder(msg: impl Into<String>) -> Self {
        VideoError::Decoder(msg.into())
    }

    pub fn invalid_param(msg: impl Into<String>) -> Self {
        VideoError::InvalidParam(msg.into())
    }

    pub fn task_not_found(id: impl Into<String>) -> Self {
        VideoError::TaskNotFound(id.into())
    }

    pub fn other(msg: impl Into<String>) -> Self {
        VideoError::Other(msg.into())
    }
}

/// Convert FFmpeg Error to VideoError
impl From<ffmpeg_next::Error> for VideoError {
    fn from(err: ffmpeg_next::Error) -> Self {
        VideoError::Ffmpeg(err.to_string())
    }
}

/// Type alias for Result with VideoError
pub type VideoResult<T> = Result<T, VideoError>;