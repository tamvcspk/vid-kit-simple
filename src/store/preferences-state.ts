import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { UserPreferencesState } from '../types/state.types';
import { useConfigStore } from './config.store';

// This store is now a wrapper around config.store.ts for backward compatibility
// New code should use config.store.ts directly

// Define interface for PreferencesStore
interface PreferencesStore {
  // State
  data: UserPreferencesState | null;
  isLoading: boolean;
  error: string | null;

  // Actions
  setPreferencesState: (preferencesState: UserPreferencesState) => void;
  fetchPreferencesState: () => Promise<void>;
  updatePreferences: (preferences: UserPreferencesState) => Promise<void>;
  savePreferencesToFile: () => Promise<void>;
  loadPreferencesFromFile: () => Promise<void>;

  // Helper methods
  getDefaultPreferences: () => UserPreferencesState;
  convertConfigToPreferences: () => UserPreferencesState | null;
}

// Create store with devtools middleware
const usePreferencesStore = create<PreferencesStore>()(
  devtools(
    (set) => ({
      // Initial state
      data: null,
      isLoading: true,
      error: null,

      // Actions
      setPreferencesState: (preferencesState) => set({ data: preferencesState }),

      getDefaultPreferences: () => ({
        default_output_dir: null,
        default_format: 'mp4',
        use_gpu: false,
        theme: 'light'
      }),

      // Convert config store data to preferences format
      convertConfigToPreferences: () => {
        const config = useConfigStore.getState();
        if (!config) return null;

        return {
          default_output_dir: config.outputFolder || null,
          default_format: config.defaultFormat,
          use_gpu: config.useGpu,
          theme: config.theme
        };
      },

      fetchPreferencesState: async () => {
        try {
          set({ isLoading: true });

          // Load from config store
          await useConfigStore.getState().loadConfig();

          // Convert to preferences format
          const preferences = usePreferencesStore.getState().convertConfigToPreferences();

          // Set state
          set({ data: preferences, error: null });

          // Emit event for backward compatibility
          if (preferences) {
            await invoke('emit_preferences_changed', { preferences });
          }
        } catch (error) {
          set({ error: `Failed to fetch preferences state: ${error}` });
          // Use default preferences if all else fails
          const defaultPrefs = usePreferencesStore.getState().getDefaultPreferences();
          set({ data: defaultPrefs });
        } finally {
          set({ isLoading: false });
        }
      },

      updatePreferences: async (preferences) => {
        try {
          // Update local state
          set({ data: preferences });

          // Convert to config format
          const config = {
            outputFolder: preferences.default_output_dir || '',
            defaultFormat: preferences.default_format,
            useGpu: preferences.use_gpu,
            theme: preferences.theme as 'light' | 'dark'
          };

          // Save to config store
          await useConfigStore.getState().saveConfig(config);

          // Emit event for backward compatibility
          await invoke('emit_preferences_changed', { preferences });
        } catch (error) {
          set({ error: `Failed to update preferences: ${error}` });
        }
      },

      savePreferencesToFile: async () => {
        try {
          // This is now handled by config store
          const preferences = usePreferencesStore.getState().data;
          if (!preferences) return;

          // Convert to config format and save
          const config = {
            outputFolder: preferences.default_output_dir || '',
            defaultFormat: preferences.default_format,
            useGpu: preferences.use_gpu,
            theme: preferences.theme as 'light' | 'dark'
          };

          await useConfigStore.getState().saveConfig(config);
        } catch (error) {
          set({ error: `Failed to save preferences to file: ${error}` });
        }
      },

      loadPreferencesFromFile: async () => {
        try {
          // This is now handled by config store
          await useConfigStore.getState().loadConfig();

          // Convert to preferences format
          const preferences = usePreferencesStore.getState().convertConfigToPreferences();

          // Set state
          if (preferences) {
            set({ data: preferences });

            // Emit event for backward compatibility
            await invoke('emit_preferences_changed', { preferences });
          }
        } catch (error) {
          set({ error: `Failed to load preferences from file: ${error}` });
        }
      },
    }),
    { name: 'preferences-store' }
  )
);

// Set up listener for preferences-changed event
listen<UserPreferencesState>('preferences-changed', (event) => {
  usePreferencesStore.setState({ data: event.payload });
}).catch(console.error);

export default usePreferencesStore;
