import { useEffect } from 'react';
import useConversionStore from '../states/conversion-state';

/**
 * Hook tùy chỉnh để sử dụng ConversionStore
 * Tự động fetch conversion state khi component mount
 */
export function useConversionStateStore() {
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

  // Fetch conversion state khi component mount
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
