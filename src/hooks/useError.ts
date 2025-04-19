import { useState } from 'react';
import { AppError, ErrorCategory } from '../utils';
import { addErrorNotification } from '../store/notification-store';

/**
 * Custom hook for managing error state
 */
const useError = () => {
  const [error, setError] = useState<AppError | null>(null);

  /**
   * Clear the current error
   */
  const clearError = () => {
    setError(null);
  };

  /**
   * Set an error with the given message and category
   */
  const setErrorWithMessage = (
    message: string,
    category: ErrorCategory = ErrorCategory.Other,
    details?: string
  ) => {
    const errorObj = {
      message,
      category,
      details,
      timestamp: new Date()
    };

    // Set local error state
    setError(errorObj);

    // Also add to notification system
    addErrorNotification(errorObj);
  };

  /**
   * Set an error object and add it to notifications
   */
  const setErrorObject = (errorObj: AppError) => {
    // Set local error state
    setError(errorObj);

    // Also add to notification system
    addErrorNotification(errorObj);
  };

  return {
    error,
    setError: setErrorObject,
    clearError,
    setErrorWithMessage
  };
};

export default useError;
