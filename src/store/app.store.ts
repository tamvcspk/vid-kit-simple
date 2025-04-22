import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

interface GpuInfo {
  available: boolean;
  enabled: boolean;
  name: string;
}

interface AppState {
  activeTab: 'convert' | 'split' | 'edit' | 'sanitize';
  gpuInfo: GpuInfo;
  isLoading: boolean;
  error: string | null;
  backendAlive: boolean;
  lastHeartbeat: number;
  
  // Actions
  setActiveTab: (tab: 'convert' | 'split' | 'edit' | 'sanitize') => void;
  loadGpuInfo: () => Promise<void>;
  setGpuEnabled: (enabled: boolean) => Promise<void>;
  checkBackendStatus: () => void;
}

export const useAppStore = create<AppState>((set, get) => ({
  // State
  activeTab: 'convert',
  gpuInfo: {
    available: false,
    enabled: false,
    name: ''
  },
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
      const gpuInfo = await invoke<GpuInfo>('get_gpu_info');
      set({ gpuInfo, isLoading: false });
      
      // Set up listener for GPU status changes
      const unlistenGpuStatus = await listen('gpu-status-changed', (event) => {
        const newGpuInfo = event.payload as GpuInfo;
        set({ gpuInfo: newGpuInfo });
      });
      
      // Clean up listener when component unmounts
      window.addEventListener('beforeunload', () => {
        unlistenGpuStatus();
      });
      
    } catch (error) {
      console.error('Failed to load GPU info:', error);
      set({ error: String(error), isLoading: false });
    }
  },
  
  setGpuEnabled: async (enabled) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('set_gpu_enabled', { enabled });
      
      // Update will come through the event listener
      set({ isLoading: false });
    } catch (error) {
      console.error('Failed to set GPU enabled:', error);
      set({ error: String(error), isLoading: false });
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
