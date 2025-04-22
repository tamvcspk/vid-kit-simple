import { useEffect } from 'react';
import { useAppStore } from '../store';

/**
 * Hook to access and manage application state
 * Automatically loads GPU info when component mounts
 */
export function useAppState() {
  const {
    activeTab,
    gpuInfo,
    isLoading,
    error,
    backendAlive,
    setActiveTab,
    loadGpuInfo,
    setGpuEnabled
  } = useAppStore();

  // Load GPU info on mount
  useEffect(() => {
    loadGpuInfo();
  }, [loadGpuInfo]);

  return {
    // State
    activeTab,
    gpuInfo,
    isLoading,
    error,
    backendAlive,

    // Actions
    setActiveTab,
    setGpuEnabled,
    refreshGpuInfo: loadGpuInfo
  };
}
