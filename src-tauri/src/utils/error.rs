use std::fmt;
use thiserror::Error;
use serde::{Serialize, Deserialize};

/// Error codes for categorizing errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCode {
    // IO related errors (1000-1999)
    FileNotFound = 1000,
    PermissionDenied = 1001,
    FileReadError = 1002,
    FileWriteError = 1003,
    DirectoryError = 1004,

    // FFmpeg related errors (2000-2999)
    FFmpegInitError = 2000,
    CodecNotSupported = 2001,
    EncodingError = 2002,
    DecodingError = 2003,
    FormatError = 2004,

    // State management errors (3000-3999)
    StateAccessError = 3000,
    StateMutationError = 3001,
    StateSerializationError = 3002,
    TaskNotFound = 3003,
    FileNotFound3 = 3004,

    // Preset management errors (4000-4999)
    PresetNotFound = 4000,
    PresetValidationError = 4001,
    PresetSaveError = 4002,

    // Video processing errors (5000-5999)
    VideoInfoError = 5000,
    VideoProcessingFailed = 5001,
    InvalidVideoFormat = 5002,

    // GPU related errors (6000-6999)
    GpuNotAvailable = 6000,
    GpuInitError = 6001,

    // General errors (9000-9999)
    UnknownError = 9000,
    NotImplemented = 9001,
    InvalidArgument = 9002,
}

/// Structured error information for frontend consumption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<String>,
}

/// Main application error type
#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO Error: {source}")]
    IoError {
        source: std::io::Error,
        code: ErrorCode,
        context: Option<String>,
    },

    #[error("FFmpeg Error: {message}")]
    FFmpegError {
        message: String,
        code: ErrorCode,
        context: Option<String>,
    },

    #[error("State Error: {message}")]
    StateError {
        message: String,
        code: ErrorCode,
        context: Option<String>,
    },

    #[error("Preset Error: {message}")]
    PresetError {
        message: String,
        code: ErrorCode,
        context: Option<String>,
    },

    #[error("Video Processing Error: {message}")]
    VideoProcessingError {
        message: String,
        code: ErrorCode,
        context: Option<String>,
    },

    #[error("GPU Error: {message}")]
    GpuError {
        message: String,
        code: ErrorCode,
        context: Option<String>,
    },

    #[error("Validation Error: {message}")]
    ValidationError {
        message: String,
        code: ErrorCode,
        context: Option<String>,
    },

    #[error("{message}")]
    OtherError {
        message: String,
        code: ErrorCode,
        context: Option<String>,
    },

    #[error("State error: {0}")]
    DomainStateError(#[from] crate::state::errors::StateError),
}

impl AppError {
    /// Create a new IO error
    pub fn io_error(source: std::io::Error, code: ErrorCode, context: Option<String>) -> Self {
        AppError::IoError { source, code, context }
    }

    /// Create a new FFmpeg error
    pub fn ffmpeg_error(message: impl Into<String>, code: ErrorCode, context: Option<String>) -> Self {
        AppError::FFmpegError {
            message: message.into(),
            code,
            context
        }
    }

    /// Create a new state error
    pub fn state_error(message: impl Into<String>, code: ErrorCode, context: Option<String>) -> Self {
        AppError::StateError {
            message: message.into(),
            code,
            context
        }
    }

    /// Create a new preset error
    pub fn preset_error(message: impl Into<String>, code: ErrorCode, context: Option<String>) -> Self {
        AppError::PresetError {
            message: message.into(),
            code,
            context
        }
    }

    /// Create a new video processing error
    pub fn video_error(message: impl Into<String>, code: ErrorCode, context: Option<String>) -> Self {
        AppError::VideoProcessingError {
            message: message.into(),
            code,
            context
        }
    }

    /// Create a new GPU error
    pub fn gpu_error(message: impl Into<String>, code: ErrorCode, context: Option<String>) -> Self {
        AppError::GpuError {
            message: message.into(),
            code,
            context
        }
    }

    /// Create a new validation error
    pub fn validation_error(message: impl Into<String>, code: ErrorCode, context: Option<String>) -> Self {
        AppError::ValidationError {
            message: message.into(),
            code,
            context
        }
    }

    /// Create a new generic error
    pub fn other_error(message: impl Into<String>, code: ErrorCode, context: Option<String>) -> Self {
        AppError::OtherError {
            message: message.into(),
            code,
            context
        }
    }

    /// Get the error code
    pub fn code(&self) -> ErrorCode {
        match self {
            AppError::IoError { code, .. } => *code,
            AppError::FFmpegError { code, .. } => *code,
            AppError::StateError { code, .. } => *code,
            AppError::PresetError { code, .. } => *code,
            AppError::VideoProcessingError { code, .. } => *code,
            AppError::GpuError { code, .. } => *code,
            AppError::ValidationError { code, .. } => *code,
            AppError::OtherError { code, .. } => *code,
            AppError::DomainStateError(_) => ErrorCode::StateAccessError,
        }
    }

    /// Convert to a structured error info for frontend consumption
    pub fn to_error_info(&self) -> ErrorInfo {
        ErrorInfo {
            code: self.code(),
            message: self.to_string(),
            details: match self {
                AppError::IoError { context, .. } => context.clone(),
                AppError::FFmpegError { context, .. } => context.clone(),
                AppError::StateError { context, .. } => context.clone(),
                AppError::PresetError { context, .. } => context.clone(),
                AppError::VideoProcessingError { context, .. } => context.clone(),
                AppError::GpuError { context, .. } => context.clone(),
                AppError::ValidationError { context, .. } => context.clone(),
                AppError::OtherError { context, .. } => context.clone(),
                AppError::DomainStateError(_) => None,
            },
        }
    }

    /// Log the error to console
    pub fn log(&self) {
        eprintln!("ERROR [{}]: {}", self.code() as u32, self);
        match self {
            AppError::IoError { context, .. } => {
                if let Some(ctx) = context {
                    eprintln!("Context: {}", ctx);
                }
            },
            AppError::FFmpegError { context, .. } => {
                if let Some(ctx) = context {
                    eprintln!("Context: {}", ctx);
                }
            },
            AppError::StateError { context, .. } => {
                if let Some(ctx) = context {
                    eprintln!("Context: {}", ctx);
                }
            },
            AppError::PresetError { context, .. } => {
                if let Some(ctx) = context {
                    eprintln!("Context: {}", ctx);
                }
            },
            AppError::VideoProcessingError { context, .. } => {
                if let Some(ctx) = context {
                    eprintln!("Context: {}", ctx);
                }
            },
            AppError::GpuError { context, .. } => {
                if let Some(ctx) = context {
                    eprintln!("Context: {}", ctx);
                }
            },
            AppError::ValidationError { context, .. } => {
                if let Some(ctx) = context {
                    eprintln!("Context: {}", ctx);
                }
            },
            AppError::OtherError { context, .. } => {
                if let Some(ctx) = context {
                    eprintln!("Context: {}", ctx);
                }
            },
            AppError::DomainStateError(err) => {
                eprintln!("State Error Context: {:?}", err);
            },
        }
    }
}

// We can keep these From implementations
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        let code = match error.kind() {
            std::io::ErrorKind::NotFound => ErrorCode::FileNotFound,
            std::io::ErrorKind::PermissionDenied => ErrorCode::PermissionDenied,
            _ => ErrorCode::FileReadError,
        };

        let context = match error.kind() {
            std::io::ErrorKind::NotFound => Some("File or directory not found".to_string()),
            std::io::ErrorKind::PermissionDenied => Some("Permission denied when accessing file".to_string()),
            std::io::ErrorKind::ConnectionRefused => Some("Connection refused".to_string()),
            std::io::ErrorKind::ConnectionReset => Some("Connection reset".to_string()),
            std::io::ErrorKind::ConnectionAborted => Some("Connection aborted".to_string()),
            std::io::ErrorKind::NotConnected => Some("Not connected".to_string()),
            std::io::ErrorKind::AddrInUse => Some("Address already in use".to_string()),
            std::io::ErrorKind::AddrNotAvailable => Some("Address not available".to_string()),
            std::io::ErrorKind::BrokenPipe => Some("Broken pipe".to_string()),
            std::io::ErrorKind::AlreadyExists => Some("File already exists".to_string()),
            std::io::ErrorKind::WouldBlock => Some("Operation would block".to_string()),
            std::io::ErrorKind::InvalidInput => Some("Invalid input parameter".to_string()),
            std::io::ErrorKind::InvalidData => Some("Invalid data".to_string()),
            std::io::ErrorKind::TimedOut => Some("Operation timed out".to_string()),
            std::io::ErrorKind::WriteZero => Some("Write returned zero bytes".to_string()),
            std::io::ErrorKind::Interrupted => Some("Operation interrupted".to_string()),
            std::io::ErrorKind::Unsupported => Some("Operation not supported".to_string()),
            std::io::ErrorKind::UnexpectedEof => Some("Unexpected end of file".to_string()),
            std::io::ErrorKind::OutOfMemory => Some("Out of memory".to_string()),
            _ => Some("I/O error occurred".to_string()),
        };

        AppError::IoError {
            source: error,
            code,
            context
        }
    }
}

impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::VideoProcessingError {
            message: error,
            code: ErrorCode::VideoProcessingFailed,
            context: Some("Error during video processing operation".to_string())
        }
    }
}

impl From<&str> for AppError {
    fn from(error: &str) -> Self {
        AppError::VideoProcessingError {
            message: error.to_string(),
            code: ErrorCode::VideoProcessingFailed,
            context: Some("Error during video processing operation".to_string())
        }
    }
}

// Add From<VideoError> for AppError
impl From<crate::services::video_processor::VideoError> for AppError {
    fn from(error: crate::services::video_processor::VideoError) -> Self {
        match error {
            crate::services::video_processor::VideoError::Io(e) => AppError::IoError {
                source: e,
                code: ErrorCode::FileReadError,
                context: Some("Video file access error".to_string())
            },
            crate::services::video_processor::VideoError::Ffmpeg(msg) => AppError::FFmpegError {
                message: msg,
                code: ErrorCode::FFmpegInitError,
                context: Some("FFmpeg operation failed".to_string())
            },
            crate::services::video_processor::VideoError::TaskNotFound(id) => AppError::OtherError {
                message: format!("Task not found: {}", id),
                code: ErrorCode::TaskNotFound,
                context: Some("Video processing task lookup failed".to_string())
            },
            crate::services::video_processor::VideoError::NoVideoStream(path) => AppError::VideoProcessingError {
                message: format!("No video stream found in {:?}", path),
                code: ErrorCode::InvalidVideoFormat,
                context: Some(format!("File does not contain a valid video stream: {:?}", path))
            },
            crate::services::video_processor::VideoError::Codec(msg) => AppError::VideoProcessingError {
                message: msg,
                code: ErrorCode::CodecNotSupported,
                context: Some("Video codec not supported or not found".to_string())
            },
            crate::services::video_processor::VideoError::Encoder(msg) => AppError::VideoProcessingError {
                message: msg,
                code: ErrorCode::EncodingError,
                context: Some("Video encoding operation failed".to_string())
            },
            crate::services::video_processor::VideoError::Decoder(msg) => AppError::VideoProcessingError {
                message: msg,
                code: ErrorCode::DecodingError,
                context: Some("Video decoding operation failed".to_string())
            },
            crate::services::video_processor::VideoError::InvalidParam(msg) => AppError::ValidationError {
                message: msg,
                code: ErrorCode::InvalidArgument,
                context: Some("Invalid parameter for video processing".to_string())
            },
            crate::services::video_processor::VideoError::ThreadPool(msg) => AppError::VideoProcessingError {
                message: msg,
                code: ErrorCode::VideoProcessingFailed,
                context: Some("Thread pool error during video processing".to_string())
            },
            crate::services::video_processor::VideoError::State(e) => AppError::DomainStateError(e),
            crate::services::video_processor::VideoError::Other(msg) => AppError::OtherError {
                message: msg,
                code: ErrorCode::VideoProcessingFailed,
                context: Some("Unexpected error during video processing".to_string())
            },
        }
    }
}

/// Type alias for Result with AppError
pub type AppResult<T> = Result<T, AppError>;
