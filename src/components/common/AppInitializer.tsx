import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { useAppStore, useConfigStore, usePresetsStore, useTasksStore } from '../../store';
import { useNotifications } from '../../hooks';

/**
 * Component to initialize application state and set up event listeners
 * This component doesn't render anything, it just handles initialization logic
 */
export function AppInitializer() {
  const { loadGpuInfo } = useAppStore();
  const { loadConfig } = useConfigStore();
  const { loadPresets, createDefaultPresets } = usePresetsStore();
  const { loadTasks } = useTasksStore();
  const { addNotification } = useNotifications();

  // Initialize state on mount
  useEffect(() => {
    const initializeApp = async () => {
      try {
        // Load configuration
        await loadConfig();
        console.log('Configuration loaded');

        // Load presets and create defaults if needed
        await loadPresets();
        await createDefaultPresets();
        console.log('Presets loaded');

        // Load tasks
        await loadTasks();
        console.log('Tasks loaded');

        // Load GPU info
        await loadGpuInfo();
        console.log('GPU info loaded');

        // Add success notification
        addNotification({
          id: crypto.randomUUID(),
          type: 'success',
          source: 'System',
          message: 'Application initialized successfully',
          timestamp: new Date().toISOString(),
          read: false
        });
      } catch (error) {
        console.error('Error initializing application:', error);
        
        // Add error notification
        addNotification({
          id: crypto.randomUUID(),
          type: 'error',
          source: 'System',
          message: `Error initializing application: ${error}`,
          timestamp: new Date().toISOString(),
          read: false
        });
      }
    };

    initializeApp();
  }, [loadConfig, loadPresets, createDefaultPresets, loadTasks, loadGpuInfo, addNotification]);

  // Set up event listeners for backend events
  useEffect(() => {
    const setupEventListeners = async () => {
      // Listen for task events
      const unlistenTaskCompleted = await listen('task-completed', (event) => {
        const taskId = event.payload as string;
        addNotification({
          id: crypto.randomUUID(),
          type: 'success',
          source: 'Task Manager',
          message: `Task ${taskId} completed successfully`,
          timestamp: new Date().toISOString(),
          read: false
        });
      });

      const unlistenTaskFailed = await listen('task-failed', (event) => {
        const { taskId, error } = event.payload as { taskId: string; error: string };
        addNotification({
          id: crypto.randomUUID(),
          type: 'error',
          source: 'Task Manager',
          message: `Task ${taskId} failed: ${error}`,
          timestamp: new Date().toISOString(),
          read: false
        });
      });

      // Listen for GPU status changes
      const unlistenGpuStatus = await listen('gpu-status-changed', () => {
        loadGpuInfo();
      });

      // Listen for heartbeat events
      const unlistenHeartbeat = await listen('heartbeat', () => {
        // Update last heartbeat time in app store
        useAppStore.setState({ lastHeartbeat: Date.now() });
      });

      // Clean up listeners when component unmounts
      return () => {
        unlistenTaskCompleted();
        unlistenTaskFailed();
        unlistenGpuStatus();
        unlistenHeartbeat();
      };
    };

    const unlisten = setupEventListeners();
    
    // Clean up function
    return () => {
      unlisten.then(cleanup => cleanup());
    };
  }, [loadGpuInfo, addNotification]);

  // This component doesn't render anything
  return null;
}
