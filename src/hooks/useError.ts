import { useState } from 'react';
import { AppError, ErrorCategory } from '../utils';

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
    setError({
      message,
      category,
      details,
      timestamp: new Date()
    });
  };

  return {
    error,
    setError,
    clearError,
    setErrorWithMessage
  };
};

export default useError;
