import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { AppState } from '../types/state.types';

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
          await invoke('set_selected_gpu', { gpuIndex });
          // State will be updated through event listener
        } catch (error) {
          set({ error: `Failed to set selected GPU: ${error}` });
        }
      },

      fetchAppState: async () => {
        try {
          set({ isLoading: true });
          const appState = await invoke<AppState>('get_app_state');
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

// Set up listener for app-state-changed event
listen<AppState>('app-state-changed', (event) => {
  useAppStore.setState({ data: event.payload });
}).catch(console.error);

export default useAppStore;
