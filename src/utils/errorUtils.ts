/**
 * Error categories for application
 */
export enum ErrorCategory {
  IO = 'io',
  Network = 'network',
  FFmpeg = 'ffmpeg',
  Validation = 'validation',
  Task = 'task',
  Preset = 'preset',
  Gpu = 'gpu',
  State = 'state',
  Other = 'other'
}

/**
 * Backend error codes mapped to frontend categories
 */
export enum BackendErrorCode {
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

/**
 * Backend error info structure
 */
export interface BackendErrorInfo {
  code: BackendErrorCode;
  message: string;
  details?: string;
}

/**
 * Application error structure
 */
export interface AppError {
  message: string;
  category: ErrorCategory;
  details?: string;
  timestamp: Date;
  originalError?: unknown;
  code?: BackendErrorCode;
}

/**
 * Create a new application error
 */
export function createError(
  category: ErrorCategory,
  message: string,
  details?: string,
  originalError?: unknown
): AppError {
  return {
    message,
    category,
    details,
    timestamp: new Date(),
    originalError
  };
}

/**
 * Log an error to console
 */
export function logError(error: AppError): void {
  console.error(`[${error.category.toUpperCase()}] ${error.message}`, {
    details: error.details,
    timestamp: error.timestamp,
    originalError: error.originalError
  });
}

/**
 * Format error message for user display
 */
export function formatErrorForUser(error: AppError | null): string {
  if (!error) return '';

  let message = error.message;

  // Add category-specific prefixes
  switch (error.category) {
    case ErrorCategory.IO:
      message = `File error: ${message}`;
      break;
    case ErrorCategory.Network:
      message = `Network error: ${message}`;
      break;
    case ErrorCategory.FFmpeg:
      message = `Processing error: ${message}`;
      break;
    case ErrorCategory.Validation:
      message = `Invalid input: ${message}`;
      break;
    case ErrorCategory.Task:
      message = `Task error: ${message}`;
      break;
    case ErrorCategory.Preset:
      message = `Preset error: ${message}`;
      break;
    case ErrorCategory.Gpu:
      message = `GPU error: ${message}`;
      break;
    default:
      message = `Error: ${message}`;
  }

  return message;
}

/**
 * Map backend error code to frontend error category
 */
export function mapErrorCodeToCategory(code: BackendErrorCode): ErrorCategory {
  // Map error codes to categories
  if (code >= 1000 && code < 2000) {
    return ErrorCategory.IO;
  } else if (code >= 2000 && code < 3000) {
    return ErrorCategory.FFmpeg;
  } else if (code >= 3000 && code < 4000) {
    return ErrorCategory.State;
  } else if (code >= 4000 && code < 5000) {
    return ErrorCategory.Preset;
  } else if (code >= 5000 && code < 6000) {
    return ErrorCategory.Task;
  } else if (code >= 6000 && code < 7000) {
    return ErrorCategory.Gpu;
  }

  return ErrorCategory.Other;
}

/**
 * Handle a backend error from Tauri
 */
export function handleBackendError(error: unknown): AppError {
  // Check if it's a structured backend error
  if (error && typeof error === 'object' && 'code' in error && 'message' in error) {
    const backendError = error as BackendErrorInfo;
    const category = mapErrorCodeToCategory(backendError.code);

    return {
      message: backendError.message,
      category,
      details: backendError.details,
      timestamp: new Date(),
      originalError: error,
      code: backendError.code
    };
  }

  // Fall back to regular categorization
  return categorizeError(error);
}

/**
 * Categorize an unknown error
 */
export function categorizeError(error: unknown): AppError {
  // Default error message
  let message = 'An unexpected error occurred';
  let category = ErrorCategory.Other;
  let details: string | undefined;

  // Handle different error types
  if (error instanceof Error) {
    message = error.message;
    details = error.stack;

    // Try to categorize based on error message
    if (message.includes('file') || message.includes('path') || message.includes('directory')) {
      category = ErrorCategory.IO;
    } else if (message.includes('network') || message.includes('fetch') || message.includes('http')) {
      category = ErrorCategory.Network;
    } else if (message.includes('ffmpeg') || message.includes('codec') || message.includes('format')) {
      category = ErrorCategory.FFmpeg;
    } else if (message.includes('preset')) {
      category = ErrorCategory.Preset;
    } else if (message.includes('gpu')) {
      category = ErrorCategory.Gpu;
    } else if (message.includes('state')) {
      category = ErrorCategory.State;
    }
  } else if (typeof error === 'string') {
    message = error;

    // Try to categorize based on message content
    if (message.includes('file') || message.includes('path') || message.includes('directory')) {
      category = ErrorCategory.IO;
    } else if (message.includes('network') || message.includes('fetch') || message.includes('http')) {
      category = ErrorCategory.Network;
    } else if (message.includes('ffmpeg') || message.includes('codec') || message.includes('format')) {
      category = ErrorCategory.FFmpeg;
    }
  } else if (error && typeof error === 'object') {
    message = JSON.stringify(error);
  }

  return createError(category, message, details, error);
}
