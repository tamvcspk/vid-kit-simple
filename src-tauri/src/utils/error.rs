use std::fmt;
use std::error::Error;
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
#[derive(Debug)]
pub enum AppError {
    IoError {
        source: std::io::Error,
        code: ErrorCode,
        context: Option<String>,
    },
    FFmpegError {
        message: String,
        code: ErrorCode,
        context: Option<String>,
    },
    StateError {
        message: String,
        code: ErrorCode,
        context: Option<String>,
    },
    PresetError {
        message: String,
        code: ErrorCode,
        context: Option<String>,
    },
    VideoProcessingError {
        message: String,
        code: ErrorCode,
        context: Option<String>,
    },
    GpuError {
        message: String,
        code: ErrorCode,
        context: Option<String>,
    },
    ValidationError {
        message: String,
        code: ErrorCode,
        context: Option<String>,
    },
    OtherError {
        message: String,
        code: ErrorCode,
        context: Option<String>,
    },
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
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::IoError { source, .. } => write!(f, "IO Error: {}", source),
            AppError::FFmpegError { message, .. } => write!(f, "FFmpeg Error: {}", message),
            AppError::StateError { message, .. } => write!(f, "State Error: {}", message),
            AppError::PresetError { message, .. } => write!(f, "Preset Error: {}", message),
            AppError::VideoProcessingError { message, .. } => write!(f, "Video Processing Error: {}", message),
            AppError::GpuError { message, .. } => write!(f, "GPU Error: {}", message),
            AppError::ValidationError { message, .. } => write!(f, "Validation Error: {}", message),
            AppError::OtherError { message, .. } => write!(f, "Error: {}", message),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::IoError { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        let code = match error.kind() {
            std::io::ErrorKind::NotFound => ErrorCode::FileNotFound,
            std::io::ErrorKind::PermissionDenied => ErrorCode::PermissionDenied,
            _ => ErrorCode::FileReadError,
        };

        AppError::IoError {
            source: error,
            code,
            context: None
        }
    }
}

impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::VideoProcessingError {
            message: error,
            code: ErrorCode::VideoProcessingFailed,
            context: None
        }
    }
}

impl From<&str> for AppError {
    fn from(error: &str) -> Self {
        AppError::VideoProcessingError {
            message: error.to_string(),
            code: ErrorCode::VideoProcessingFailed,
            context: None
        }
    }
}

/// Type alias for Result with AppError
pub type AppResult<T> = Result<T, AppError>;
