import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { UserPreferencesState } from '../types/state.types';

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

      fetchPreferencesState: async () => {
        try {
          set({ isLoading: true });
          const preferencesState = await invoke<UserPreferencesState>('get_preferences');
          set({ data: preferencesState, error: null });
        } catch (error) {
          set({ error: `Failed to fetch preferences state: ${error}` });
        } finally {
          set({ isLoading: false });
        }
      },

      updatePreferences: async (preferences) => {
        try {
          await invoke('update_preferences', { newPreferences: preferences });
          // State will be updated through event listener
        } catch (error) {
          set({ error: `Failed to update preferences: ${error}` });
        }
      },

      savePreferencesToFile: async () => {
        try {
          await invoke('save_preferences_to_file');
        } catch (error) {
          set({ error: `Failed to save preferences to file: ${error}` });
        }
      },

      loadPreferencesFromFile: async () => {
        try {
          await invoke('load_preferences_from_file');
          // State will be updated through event listener
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
