import { ReactNode, useEffect } from 'react';
import { useAppStore, useConfigStore, useConversionStore, usePresetsStore, useTasksStore } from '../store';

interface GlobalStateProviderProps {
  children: ReactNode;
}

export function GlobalStateProvider({ children }: GlobalStateProviderProps) {
  // Get fetch state functions from stores
  const { loadGpuInfo } = useAppStore();
  const { loadConfig } = useConfigStore();
  const { fetchConversionState } = useConversionStore();
  const { loadPresets } = usePresetsStore();
  const { loadTasks } = useTasksStore();

  // Initialize state when component mounts
  useEffect(() => {
    // Fetch all states
    Promise.all([
      loadGpuInfo(),
      loadConfig(),
      fetchConversionState(),
      loadPresets(),
      loadTasks()
    ]).catch(error => {
      console.error('Failed to initialize state:', error);
    });
  }, [loadGpuInfo, loadConfig, fetchConversionState, loadPresets, loadTasks]);

  return <>{children}</>;
}
