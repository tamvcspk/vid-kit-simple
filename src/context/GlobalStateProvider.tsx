import { ReactNode, useEffect } from 'react';
import { useAppStore, useConfigStore, useFilesStore, usePresetsStore, useTasksStore } from '../store';

interface GlobalStateProviderProps {
  children: ReactNode;
}

export function GlobalStateProvider({ children }: GlobalStateProviderProps) {
  // Get fetch state functions from stores
  const { loadGpuInfo } = useAppStore();
  const { loadConfig } = useConfigStore();
  const { loadFiles } = useFilesStore();
  const { loadPresets } = usePresetsStore();
  const { loadTasks } = useTasksStore();

  // Initialize state when component mounts
  useEffect(() => {
    // Fetch all states
    Promise.all([
      loadGpuInfo(),
      loadConfig(),
      loadFiles(),
      loadPresets(),
      loadTasks()
    ]).catch(error => {
      console.error('Failed to initialize state:', error);
    });
  }, [loadGpuInfo, loadConfig, loadFiles, loadPresets, loadTasks]);

  return <>{children}</>;
}
