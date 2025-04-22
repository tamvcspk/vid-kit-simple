import { create } from 'zustand';
import { Store } from '@tauri-apps/plugin-store';
import { CONFIG_STORE_PATH, CONFIG_STORE_KEYS } from '../constants/stores';
import { ConfigStore } from '../types/store.types';

interface ConfigState {
  outputFolder: string;
  maxParallelJobs: number;
  retryLimit: number;
  selectedGpu: number;
  theme: 'light' | 'dark';
  defaultFormat: string;
  useGpu: boolean;
  isLoading: boolean;
  error: string | null;

  // Actions
  loadConfig: () => Promise<void>;
  saveConfig: (config: Partial<ConfigState>) => Promise<void>;
  resetConfig: () => Promise<void>;
  setTheme: (theme: 'light' | 'dark') => Promise<void>;
  setOutputFolder: (folder: string) => Promise<void>;
  setMaxParallelJobs: (jobs: number) => Promise<void>;
  setRetryLimit: (limit: number) => Promise<void>;
  setSelectedGpu: (gpu: number) => Promise<void>;
  setDefaultFormat: (format: string) => Promise<void>;
  setUseGpu: (use: boolean) => Promise<void>;
}

// Default configuration values
const DEFAULT_CONFIG: Omit<ConfigState, 'isLoading' | 'error' | 'loadConfig' | 'saveConfig'> = {
  outputFolder: '',
  maxParallelJobs: 2,
  retryLimit: 3,
  selectedGpu: -1,
  theme: 'light',
  defaultFormat: 'mp4',
  useGpu: false,
};

export const useConfigStore = create<ConfigState>((set, get) => ({
  ...DEFAULT_CONFIG,
  isLoading: false,
  error: null,

  loadConfig: async () => {
    set({ isLoading: true, error: null });
    try {
      const store = new Store(CONFIG_STORE_PATH);
      const config = await store.get(CONFIG_STORE_KEYS.CONFIG) as Partial<ConfigState> || {};

      // Merge with default values for any missing properties
      set({
        ...DEFAULT_CONFIG,
        ...config,
        isLoading: false
      });
    } catch (error) {
      console.error('Failed to load config:', error);
      set({ error: String(error), isLoading: false });
    }
  },

  saveConfig: async (config) => {
    set({ isLoading: true, error: null });
    try {
      const store = new Store(CONFIG_STORE_PATH);
      const currentState = get();

      // Create a new config object with the current state and the new values
      const newConfig = {
        outputFolder: currentState.outputFolder,
        maxParallelJobs: currentState.maxParallelJobs,
        retryLimit: currentState.retryLimit,
        selectedGpu: currentState.selectedGpu,
        theme: currentState.theme,
        defaultFormat: currentState.defaultFormat,
        useGpu: currentState.useGpu,
        ...config
      };

      // Save to store
      await store.set(CONFIG_STORE_KEYS.CONFIG, newConfig);
      await store.save();

      // Update state
      set({ ...newConfig, isLoading: false });
    } catch (error) {
      console.error('Failed to save config:', error);
      set({ error: String(error), isLoading: false });
    }
  },

  resetConfig: async () => {
    set({ isLoading: true, error: null });
    try {
      const store = new Store(CONFIG_STORE_PATH);

      // Save default config to store
      await store.set(CONFIG_STORE_KEYS.CONFIG, DEFAULT_CONFIG);
      await store.save();

      // Update state
      set({ ...DEFAULT_CONFIG, isLoading: false });
    } catch (error) {
      console.error('Failed to reset config:', error);
      set({ error: String(error), isLoading: false });
    }
  },

  setTheme: async (theme) => {
    return get().saveConfig({ theme });
  },

  setOutputFolder: async (outputFolder) => {
    return get().saveConfig({ outputFolder });
  },

  setMaxParallelJobs: async (maxParallelJobs) => {
    return get().saveConfig({ maxParallelJobs });
  },

  setRetryLimit: async (retryLimit) => {
    return get().saveConfig({ retryLimit });
  },

  setSelectedGpu: async (selectedGpu) => {
    return get().saveConfig({ selectedGpu });
  },

  setDefaultFormat: async (defaultFormat) => {
    return get().saveConfig({ defaultFormat });
  },

  setUseGpu: async (useGpu) => {
    return get().saveConfig({ useGpu });
  }
}));
