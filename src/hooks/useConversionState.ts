import { useEffect } from 'react';
import useConversionStore from '../store/conversion-state';

/**
 * Custom hook to use ConversionStore
 * Automatically fetches conversion state when component mounts
 */
export function useConversionState() {
  const {
    data: conversionState,
    isLoading,
    error,
    fetchConversionState,
    addFileToList,
    removeFileFromList,
    selectFile,
    clearFileList,
    addTask,
    markTaskFailed
  } = useConversionStore();

  // Fetch conversion state when component mounts
  useEffect(() => {
    fetchConversionState();
  }, [fetchConversionState]);

  return {
    conversionState,
    loading: isLoading,
    error,
    refreshConversionState: fetchConversionState,
    addFileToList,
    removeFileFromList,
    selectFile,
    clearFileList,
    addTask,
    markTaskFailed
  };
}
