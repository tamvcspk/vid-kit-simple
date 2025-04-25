import { useEffect } from 'react';
import { useAppStore } from '../store';

/**
 * Hook to access and manage application state
 * Automatically loads GPU info when component mounts
 */
export function useAppState() {
  const {
    activeTab,
    appInfo,
    isLoading,
    error,
    backendAlive,
    setActiveTab,
    loadGpuInfo,
    setGpuEnabled,
    setSelectedGpu
  } = useAppStore();

  // Load GPU info on mount
  useEffect(() => {
    loadGpuInfo();
  }, [loadGpuInfo]);

  // Create a compatible appState object for legacy components
  const appState = appInfo ? {
    is_initialized: true,
    app_version: appInfo.app_version,
    ffmpeg_version: appInfo.ffmpeg_version,
    gpu_available: appInfo.gpu_available,
    gpus: appInfo.gpus,
    selected_gpu_index: appInfo.selected_gpu_index
  } : null;

  // Create a simplified gpuInfo object for backward compatibility
  const gpuInfo = appInfo ? {
    available: appInfo.gpu_available,
    enabled: appInfo.selected_gpu_index >= 0,
    name: appInfo.selected_gpu_index >= 0 && appInfo.selected_gpu_index < appInfo.gpus.length
      ? appInfo.gpus[appInfo.selected_gpu_index].name
      : 'CPU'
  } : {
    available: false,
    enabled: false,
    name: 'GPU not available'
  };

  return {
    // State
    activeTab,
    gpuInfo, // For backward compatibility
    appInfo,
    appState, // For legacy components
    isLoading,
    error,
    backendAlive,

    // Actions
    setActiveTab,
    setGpuEnabled,
    setSelectedGpu,
    refreshGpuInfo: loadGpuInfo
  };
}
