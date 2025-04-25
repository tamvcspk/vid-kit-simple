import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { Store } from '@tauri-apps/plugin-store';

// Import constants for store paths and keys
import { APP_INFO_STORE_PATH, APP_INFO_STORE_KEYS } from '../constants/stores';

// Lazy initialization of the store
let appInfoStore: Store | null = null;

const getAppInfoStore = async (): Promise<Store> => {
  if (!appInfoStore) {
    // Use the new API: Store.load() instead of new Store()
    appInfoStore = await Store.load(APP_INFO_STORE_PATH);
  }
  return appInfoStore;
};

// Backend GPU info structure
interface BackendGpuInfo {
  name: string;
  is_available: boolean;
}

// Backend AppInfo structure
interface AppInfo {
  app_version: string;
  ffmpeg_version: string | null;
  gpu_available: boolean;
  gpus: BackendGpuInfo[];
  selected_gpu_index: number; // -1 for CPU, 0+ for GPU
}

interface AppState {
  activeTab: 'convert' | 'split' | 'edit' | 'sanitize';
  appInfo: AppInfo | null; // Store full AppInfo
  isLoading: boolean;
  error: string | null;
  backendAlive: boolean;
  lastHeartbeat: number;

  // Actions
  setActiveTab: (tab: 'convert' | 'split' | 'edit' | 'sanitize') => void;
  loadGpuInfo: () => Promise<void>; // Load and validate GPU info from backend
  updateAppInfo: (appInfo: Partial<AppInfo>) => void; // Update app info locally
  setGpuEnabled: (enabled: boolean) => Promise<void>; // Enable/disable GPU
  setSelectedGpu: (index: number) => Promise<void>; // Select specific GPU
  checkBackendStatus: () => void;
  saveAppInfoToStore: () => Promise<void>; // Save app info to local store
  loadAppInfoFromStore: () => Promise<void>; // Load app info from local store
}

export const useAppStore = create<AppState>((set, get) => ({
  // State
  activeTab: 'convert',
  appInfo: null,
  isLoading: false,
  error: null,
  backendAlive: true,
  lastHeartbeat: Date.now(),

  // Actions
  setActiveTab: (tab) => {
    set({ activeTab: tab });
  },

  loadGpuInfo: async () => {
    set({ isLoading: true, error: null });
    try {
      // First try to load from local store
      await get().loadAppInfoFromStore();

      // Then get app info from backend to validate/update
      const backendAppInfo = await invoke<AppInfo>('get_app_info');

      // If we have local app info, merge with backend info
      // Otherwise just use backend info
      const currentAppInfo = get().appInfo;
      if (currentAppInfo) {
        // Update with backend info but keep any local customizations
        set({
          appInfo: {
            ...currentAppInfo,
            // Always take these from backend as they're hardware/system dependent
            app_version: backendAppInfo.app_version,
            ffmpeg_version: backendAppInfo.ffmpeg_version,
            gpu_available: backendAppInfo.gpu_available,
            gpus: backendAppInfo.gpus,
            // Keep selected_gpu_index from local state if it's valid
            selected_gpu_index: currentAppInfo.selected_gpu_index >= -1 &&
                               (currentAppInfo.selected_gpu_index < backendAppInfo.gpus.length) ?
                               currentAppInfo.selected_gpu_index : backendAppInfo.selected_gpu_index
          },
          isLoading: false
        });
      } else {
        // No local state, use backend info
        set({ appInfo: backendAppInfo, isLoading: false });
      }

      console.log('GPU info loaded and validated with backend');

      // Save to store
      await get().saveAppInfoToStore();

      // Set up listener for app info changes from backend
      const unlistenAppInfoChanged = await listen('app-info-changed', (event) => {
        const newBackendAppInfo = event.payload as AppInfo;

        // Update only the hardware/system dependent parts
        const currentAppInfo = get().appInfo;
        if (currentAppInfo) {
          get().updateAppInfo({
            app_version: newBackendAppInfo.app_version,
            ffmpeg_version: newBackendAppInfo.ffmpeg_version,
            gpu_available: newBackendAppInfo.gpu_available,
            gpus: newBackendAppInfo.gpus
          });
        } else {
          set({ appInfo: newBackendAppInfo });
        }

        // Save to store
        get().saveAppInfoToStore();
      });

      // Clean up listener when component unmounts
      window.addEventListener('beforeunload', () => {
        unlistenAppInfoChanged();
      });
    } catch (error) {
      console.error('Failed to load GPU info:', error);
      set({ error: String(error), isLoading: false });

      // Try to use local store as fallback
      try {
        await get().loadAppInfoFromStore();
        if (!get().appInfo) {
          // Still no app info, use default values
          set({
            appInfo: {
              app_version: 'Unknown',
              ffmpeg_version: null,
              gpu_available: false,
              gpus: [],
              selected_gpu_index: -1
            },
            isLoading: false
          });
        }
      } catch (storeError) {
        console.error('Failed to load from store as fallback:', storeError);
        // Use default values
        set({
          appInfo: {
            app_version: 'Unknown',
            ffmpeg_version: null,
            gpu_available: false,
            gpus: [],
            selected_gpu_index: -1
          },
          isLoading: false
        });
      }
    }
  },

  setGpuEnabled: async (enabled) => {
    set({ isLoading: true, error: null });
    try {
      // Get current app info
      const { appInfo } = get();
      if (!appInfo) {
        throw new Error('App info not loaded');
      }

      // Use set_gpu command with -1 for CPU, 0 for first GPU
      const gpuIndex = enabled ? 0 : -1;

      // Update local state first
      get().updateAppInfo({ selected_gpu_index: gpuIndex });

      // Then notify backend
      await invoke('set_gpu', { gpu_index: gpuIndex });

      // Save to store
      await get().saveAppInfoToStore();

      set({ isLoading: false });
    } catch (error) {
      console.error('Failed to set GPU enabled:', error);
      set({ error: String(error), isLoading: false });
    }
  },

  setSelectedGpu: async (index) => {
    set({ isLoading: true, error: null });
    try {
      // Get current app info
      const { appInfo } = get();
      if (!appInfo) {
        throw new Error('App info not loaded');
      }

      // Validate index
      if (index < -1 || (index >= 0 && index >= appInfo.gpus.length)) {
        throw new Error(`Invalid GPU index: ${index}`);
      }

      // Update local state first
      get().updateAppInfo({ selected_gpu_index: index });

      // Then notify backend
      await invoke('set_gpu', { gpu_index: index });

      // Save to store
      await get().saveAppInfoToStore();

      set({ isLoading: false });
    } catch (error) {
      console.error('Failed to set selected GPU:', error);
      set({ error: String(error), isLoading: false });
    }
  },

  updateAppInfo: (partialAppInfo) => {
    const currentAppInfo = get().appInfo;
    if (currentAppInfo) {
      // Merge the partial app info with the current app info
      set({
        appInfo: {
          ...currentAppInfo,
          ...partialAppInfo
        }
      });
    } else {
      console.error('Cannot update app info: current app info is null');
    }
  },

  saveAppInfoToStore: async () => {
    try {
      const { appInfo } = get();
      if (appInfo) {
        // Get the store
        const store = await getAppInfoStore();

        // Save app info to store
        // With the new API, set() automatically saves, no need to call save()
        await store.set(APP_INFO_STORE_KEYS.APP_INFO, appInfo);

        console.log('App info saved to store');
      }
    } catch (error) {
      console.error('Failed to save app info to store:', error);
    }
  },

  loadAppInfoFromStore: async () => {
    try {
      // Get the store
      const store = await getAppInfoStore();

      // Load app info from store
      const appInfo = await store.get(APP_INFO_STORE_KEYS.APP_INFO) as AppInfo | null;

      if (appInfo) {
        set({ appInfo });
        console.log('App info loaded from store');
      } else {
        console.log('No app info found in store');
      }
    } catch (error) {
      console.error('Failed to load app info from store:', error);
    }
  },

  checkBackendStatus: () => {
    const lastHeartbeat = get().lastHeartbeat;
    const now = Date.now();

    // If no heartbeat for more than 60 seconds, consider backend dead
    if (now - lastHeartbeat > 60000) {
      set({ backendAlive: false });
    }
  }
}));

// Set up heartbeat listener
listen('heartbeat', () => {
  useAppStore.setState({
    lastHeartbeat: Date.now(),
    backendAlive: true
  });
});

// Set up watchdog timer to check backend status
setInterval(() => {
  useAppStore.getState().checkBackendStatus();
}, 10000); // Check every 10 seconds
