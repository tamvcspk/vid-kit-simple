use std::fmt;

#[derive(Debug)]
pub enum AppError {
    IoError(std::io::Error),
    FFmpegError(String),
    StateError(String),
    PresetError(String),
    VideoProcessingError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO Error: {}", e),
            AppError::FFmpegError(e) => write!(f, "FFmpeg Error: {}", e),
            AppError::StateError(e) => write!(f, "State Error: {}", e),
            AppError::PresetError(e) => write!(f, "Preset Error: {}", e),
            AppError::VideoProcessingError(e) => write!(f, "Video Processing Error: {}", e),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IoError(error)
    }
}

impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::VideoProcessingError(error)
    }
}
