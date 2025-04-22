import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { LazyStore } from '@tauri-apps/plugin-store';
import { ConversionState, FileInfo } from '../types/state.types';

// Create a store instance
const store = new LazyStore('conversion-state.json');

// Define interface for ConversionStore
interface ConversionStore {
  // State
  data: ConversionState | null;
  isLoading: boolean;
  error: string | null;

  // Actions
  setConversionState: (conversionState: ConversionState) => void;
  fetchConversionState: () => Promise<void>;

  // File management
  addFileToList: (file: Omit<FileInfo, 'id'>) => Promise<void>;
  removeFileFromList: (fileId: string) => Promise<void>;
  selectFile: (fileId: string | null) => Promise<void>;
  clearFileList: () => Promise<void>;

  // Helper methods
  getDefaultConversionState: () => ConversionState;
  saveToStore: () => Promise<void>;
  loadFromStore: () => Promise<ConversionState | null>;

  // Task management has been moved to tasks.store.ts
}

// Create store with devtools middleware
const useConversionStore = create<ConversionStore>()(
  devtools(
    (set) => ({
      // Initial state
      data: null,
      isLoading: true,
      error: null,

      // Actions
      setConversionState: (conversionState) => set({ data: conversionState }),

      getDefaultConversionState: () => ({
        files: [],
        selected_file_id: null
      }),

      saveToStore: async () => {
        const state = get().data;
        if (!state) return;

        await store.set('conversion-state', state);
        await store.save();
      },

      loadFromStore: async () => {
        return await store.get<ConversionState>('conversion-state');
      },

      fetchConversionState: async () => {
        try {
          set({ isLoading: true });

          // Try to load from store
          let conversionState = await get().loadFromStore();

          // If not found, try to get from backend for backward compatibility
          if (!conversionState) {
            try {
              conversionState = await invoke<ConversionState>('get_conversion_state');
              // Save to store for future use
              set({ data: conversionState });
              await get().saveToStore();
            } catch (e) {
              // If backend call fails, use default state
              conversionState = get().getDefaultConversionState();
              set({ data: conversionState });
              await get().saveToStore();
            }
          } else {
            set({ data: conversionState });
          }

          // Emit event for backward compatibility
          await invoke('emit_conversion_state_changed', { conversionState });

          set({ error: null });
        } catch (error) {
          set({ error: `Failed to fetch conversion state: ${error}` });
          // Use default state if all else fails
          const defaultState = get().getDefaultConversionState();
          set({ data: defaultState });
        } finally {
          set({ isLoading: false });
        }
      },

      // File management
      addFileToList: async (file) => {
        try {
          const id = crypto.randomUUID();
          const state = get().data;

          if (!state) {
            // Initialize state if it doesn't exist
            const newState = get().getDefaultConversionState();
            set({ data: newState });
          }

          // Create new file info
          const fileInfo: FileInfo = {
            id,
            name: file.name,
            path: file.path,
            size: file.size,
            type: file.type,
            duration: file.duration,
            resolution: file.resolution,
            thumbnail: file.thumbnail
          };

          // Update state
          const updatedState = { ...get().data! };

          // Check if file already exists by path
          if (!updatedState.files.some(f => f.path === file.path)) {
            updatedState.files.push(fileInfo);

            // If no file is selected, select the first one
            if (updatedState.selected_file_id === null && updatedState.files.length === 1) {
              updatedState.selected_file_id = id;
            }

            // Update state
            set({ data: updatedState });

            // Save to store
            await get().saveToStore();

            // Emit event for backward compatibility
            await invoke('emit_conversion_state_changed', { conversionState: updatedState });
          }
        } catch (error) {
          set({ error: `Failed to add file to list: ${error}` });
        }
      },

      removeFileFromList: async (fileId) => {
        try {
          const state = get().data;
          if (!state) return;

          // Find file index
          const fileIndex = state.files.findIndex(f => f.id === fileId);
          if (fileIndex === -1) {
            set({ error: `File not found: ${fileId}` });
            return;
          }

          // Create updated state
          const updatedState = { ...state };
          updatedState.files.splice(fileIndex, 1);

          // If the deleted file is the currently selected file
          if (state.selected_file_id === fileId) {
            // Select the first file in the list if any files remain
            updatedState.selected_file_id = updatedState.files.length > 0 ? updatedState.files[0].id : null;
          }

          // Update state
          set({ data: updatedState });

          // Save to store
          await get().saveToStore();

          // Emit event for backward compatibility
          await invoke('emit_conversion_state_changed', { conversionState: updatedState });
        } catch (error) {
          set({ error: `Failed to remove file from list: ${error}` });
        }
      },

      selectFile: async (fileId) => {
        try {
          const state = get().data;
          if (!state) return;

          // Create updated state
          const updatedState = { ...state };

          if (fileId === null) {
            // Deselect file
            updatedState.selected_file_id = null;
          } else {
            // Check if file exists
            const fileExists = state.files.some(f => f.id === fileId);
            if (!fileExists) {
              set({ error: `File not found: ${fileId}` });
              return;
            }

            // Select file
            updatedState.selected_file_id = fileId;
          }

          // Update state
          set({ data: updatedState });

          // Save to store
          await get().saveToStore();

          // Emit event for backward compatibility
          await invoke('emit_conversion_state_changed', { conversionState: updatedState });
        } catch (error) {
          set({ error: `Failed to select file: ${error}` });
        }
      },

      clearFileList: async () => {
        try {
          const state = get().data;
          if (!state) return;

          // Create updated state
          const updatedState = { ...state, files: [], selected_file_id: null };

          // Update state
          set({ data: updatedState });

          // Save to store
          await get().saveToStore();

          // Emit event for backward compatibility
          await invoke('emit_conversion_state_changed', { conversionState: updatedState });
        } catch (error) {
          set({ error: `Failed to clear file list: ${error}` });
        }
      },

      // Task management has been moved to tasks.store.ts
    }),
    { name: 'conversion-store' }
  )
);

// Set up listener for conversion-state-changed event
listen<ConversionState>('conversion-state-changed', (event) => {
  useConversionStore.setState({ data: event.payload });
}).catch(console.error);

// Set up listener for conversion-progress event
listen<{ task_id: string; progress: number }>('conversion-progress', () => {
  // No need to update state here because conversion-state-changed will be emitted
}).catch(console.error);

export default useConversionStore;
