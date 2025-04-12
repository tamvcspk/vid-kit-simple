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
  Other = 'other'
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
    }
  } else if (typeof error === 'string') {
    message = error;
  } else if (error && typeof error === 'object') {
    message = JSON.stringify(error);
  }
  
  return createError(category, message, details, error);
}
