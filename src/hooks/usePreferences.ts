import { useEffect } from 'react';
import { useConfigStore } from '../store';

/**
 * Hook to access and manage application preferences
 * Automatically loads configuration when component mounts
 */
export function usePreferences() {
  const {
    outputFolder,
    maxParallelJobs,
    retryLimit,
    selectedGpu,
    theme,
    defaultFormat,
    useGpu,
    isLoading,
    error,
    loadConfig,
    saveConfig,
    resetConfig,
    setTheme,
    setOutputFolder,
    setMaxParallelJobs,
    setRetryLimit,
    setSelectedGpu,
    setDefaultFormat,
    setUseGpu
  } = useConfigStore();

  // Load config on mount
  useEffect(() => {
    loadConfig();
  }, [loadConfig]);

  return {
    // State
    preferences: {
      outputFolder,
      maxParallelJobs,
      retryLimit,
      selectedGpu,
      theme,
      defaultFormat,
      useGpu
    },
    isLoading,
    error,

    // Actions
    updatePreferences: saveConfig,
    resetPreferences: resetConfig,
    setTheme,
    setOutputFolder,
    setMaxParallelJobs,
    setRetryLimit,
    setSelectedGpu,
    setDefaultFormat,
    setUseGpu,
    refreshPreferences: loadConfig
  };
}
