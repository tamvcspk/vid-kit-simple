import { AppError, ErrorCategory, createError, logError, categorizeError } from '../utils';

/**
 * Base service class with standardized error handling
 */
export abstract class BaseService {
  /**
   * Handle an error and return a typed result
   */
  protected handleError<T>(
    error: unknown,
    defaultMessage = 'An operation failed',
    defaultCategory = ErrorCategory.Other,
    defaultValue?: T
  ): T | null {
    // Categorize and log the error
    const appError = this.createAppError(error, defaultMessage, defaultCategory);
    logError(appError);

    // Return the default value or null
    return defaultValue !== undefined ? defaultValue : null;
  }

  /**
   * Create an AppError from any error
   */
  protected createAppError(
    error: unknown,
    defaultMessage = 'An operation failed',
    defaultCategory = ErrorCategory.Other
  ): AppError {
    // If it's already an AppError, return it
    if (typeof error === 'object' && error !== null && 'category' in error && 'message' in error) {
      return error as AppError;
    }

    // Try to categorize the error
    const categorizedError = categorizeError(error);

    // If categorization worked, return it
    if (categorizedError.category !== ErrorCategory.Other) {
      return categorizedError;
    }

    // Create a new error with the default category and message
    return createError(
      defaultCategory,
      typeof error === 'string' ? error : defaultMessage,
      typeof error === 'object' ? JSON.stringify(error) : undefined,
      error
    );
  }

  /**
   * Wrap an async function with standardized error handling
   */
  protected async withErrorHandling<T>(
    fn: () => Promise<T>,
    errorMessage: string,
    errorCategory: ErrorCategory,
    defaultValue?: T
  ): Promise<T | null> {
    try {
      return await fn();
    } catch (error) {
      return this.handleError(error, errorMessage, errorCategory, defaultValue);
    }
  }
}
