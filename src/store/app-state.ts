import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { AppState, GpuInfo } from '../types/state.types';

// Define AppInfo interface to match backend
interface AppInfo {
  app_version: string;
  ffmpeg_version: string | null;
  gpu_available: boolean;
  gpus: GpuInfo[];
  selected_gpu_index: number; // -1 for CPU, 0+ for GPU
}

// Define interface for AppStore
interface AppStore {
  // State
  data: AppState | null;
  isLoading: boolean;
  error: string | null;

  // Actions
  setAppState: (appState: AppState) => void;
  setSelectedGpu: (gpuIndex: number) => Promise<void>;
  fetchAppState: () => Promise<void>;
}

// Create store with devtools middleware
const useAppStore = create<AppStore>()(
  devtools(
    (set) => ({
      // Initial state
      data: null,
      isLoading: true,
      error: null,

      // Actions
      setAppState: (appState) => set({ data: appState }),

      setSelectedGpu: async (gpuIndex) => {
        try {
          await invoke('set_gpu', { gpuIndex });
          // State will be updated through event listener
        } catch (error) {
          set({ error: `Failed to set selected GPU: ${error}` });
        }
      },

      fetchAppState: async () => {
        try {
          set({ isLoading: true });
          // Use new get_app_info command
          const appInfo = await invoke<AppInfo>('get_app_info');

          // Convert AppInfo to AppState
          const appState: AppState = {
            is_initialized: true,
            app_version: appInfo.app_version,
            ffmpeg_version: appInfo.ffmpeg_version,
            gpu_available: appInfo.gpu_available,
            gpus: appInfo.gpus,
            selected_gpu_index: appInfo.selected_gpu_index
          };

          set({ data: appState, error: null });
        } catch (error) {
          set({ error: `Failed to fetch app state: ${error}` });
        } finally {
          set({ isLoading: false });
        }
      },
    }),
    { name: 'app-store' }
  )
);

// Set up listener for app-info-changed event
listen<AppInfo>('app-info-changed', (event) => {
  // Convert AppInfo to AppState
  const appState: AppState = {
    is_initialized: true,
    app_version: event.payload.app_version,
    ffmpeg_version: event.payload.ffmpeg_version,
    gpu_available: event.payload.gpu_available,
    gpus: event.payload.gpus,
    selected_gpu_index: event.payload.selected_gpu_index
  };

  useAppStore.setState({ data: appState });
}).catch(console.error);

// Also keep the old listener for backward compatibility
listen<AppState>('app-state-changed', (event) => {
  useAppStore.setState({ data: event.payload });
}).catch(console.error);

export default useAppStore;
