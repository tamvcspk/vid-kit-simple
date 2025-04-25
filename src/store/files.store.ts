import { create } from 'zustand';
import { Store } from '@tauri-apps/plugin-store';
import { v4 as uuidv4 } from 'uuid';
import { FILES_STORE_PATH, FILES_STORE_KEYS } from '../constants/stores';

// Create a store instance
let storePromise: Promise<Store> | null = null;

// Function to get the store instance
const getStore = async (): Promise<Store> => {
  if (!storePromise) {
    storePromise = Store.load(FILES_STORE_PATH);
  }
  return storePromise;
};

export interface FileInfo {
  id: string;
  name: string;
  path: string;
  size: number;
  type: string;
  duration?: number;
  resolution?: {
    width: number;
    height: number;
  };
  thumbnail?: string;
  selected?: boolean;
}

interface FilesState {
  files: FileInfo[];
  selectedFileId: string | null;
  isLoading: boolean;
  error: string | null;

  // Actions
  loadFiles: () => Promise<void>;
  addFile: (file: Omit<FileInfo, 'id' | 'selected'>) => Promise<string>;
  updateFile: (id: string, updates: Partial<FileInfo>) => Promise<void>;
  removeFile: (id: string) => Promise<void>;
  clearFiles: () => Promise<void>;
  selectFile: (id: string | null) => Promise<void>;

  // Getters
  getFileById: (id: string) => FileInfo | undefined;
  getSelectedFile: () => FileInfo | undefined;
}

export const useFilesStore = create<FilesState>((set, get) => ({
  // State
  files: [],
  selectedFileId: null,
  isLoading: false,
  error: null,

  // Actions
  loadFiles: async () => {
    set({ isLoading: true, error: null });
    try {
      const store = await getStore();
      const files = await store.get(FILES_STORE_KEYS.FILES) as FileInfo[] || [];
      const selectedFileId = await store.get(FILES_STORE_KEYS.SELECTED_FILE) as string || null;

      set({ files, selectedFileId, isLoading: false });
    } catch (error) {
      console.error('Failed to load files:', error);
      set({ error: String(error), isLoading: false });
    }
  },

  addFile: async (fileData) => {
    set({ isLoading: true, error: null });
    try {
      const store = await getStore();

      const file: FileInfo = {
        id: uuidv4(),
        selected: false,
        ...fileData
      };

      // Add to files array
      const files = [...get().files, file];

      // Save to store
      await store.set(FILES_STORE_KEYS.FILES, files);
      await store.save();

      // Update state
      set({ files, isLoading: false });

      return file.id;
    } catch (error) {
      console.error('Failed to add file:', error);
      set({ error: String(error), isLoading: false });
      throw error;
    }
  },

  updateFile: async (id, updates) => {
    try {
      const files = [...get().files];
      const index = files.findIndex(f => f.id === id);

      if (index === -1) {
        throw new Error(`File with ID ${id} not found`);
      }

      // Update file
      files[index] = { ...files[index], ...updates };

      // Save to store
      const store = await getStore();
      await store.set(FILES_STORE_KEYS.FILES, files);
      await store.save();

      // Update state
      set({ files });
    } catch (error) {
      console.error(`Failed to update file ${id}:`, error);
      set({ error: String(error) });
    }
  },

  removeFile: async (id) => {
    set({ isLoading: true, error: null });
    try {
      // Get current state
      const files = get().files.filter(f => f.id !== id);
      let selectedFileId = get().selectedFileId;

      // If the removed file was selected, clear selection
      if (selectedFileId === id) {
        selectedFileId = null;
      }

      // Save to store
      const store = await getStore();
      await store.set(FILES_STORE_KEYS.FILES, files);
      await store.set(FILES_STORE_KEYS.SELECTED_FILE, selectedFileId);
      await store.save();

      // Update state
      set({ files, selectedFileId, isLoading: false });
    } catch (error) {
      console.error('Failed to remove file:', error);
      set({ error: String(error), isLoading: false });
    }
  },

  clearFiles: async () => {
    set({ isLoading: true, error: null });
    try {
      // Save to store
      const store = await getStore();
      await store.set(FILES_STORE_KEYS.FILES, []);
      await store.set(FILES_STORE_KEYS.SELECTED_FILE, null);
      await store.save();

      // Update state
      set({ files: [], selectedFileId: null, isLoading: false });
    } catch (error) {
      console.error('Failed to clear files:', error);
      set({ error: String(error), isLoading: false });
    }
  },

  selectFile: async (id) => {
    try {
      // If id is null, just clear selection
      if (id === null) {
        // Update files to clear any selected flag
        const files = get().files.map(f => ({ ...f, selected: false }));

        // Save to store
        const store = await getStore();
        await store.set(FILES_STORE_KEYS.FILES, files);
        await store.set(FILES_STORE_KEYS.SELECTED_FILE, null);
        await store.save();

        // Update state
        set({ files, selectedFileId: null });
        return;
      }

      // Find the file
      const files = [...get().files];
      const index = files.findIndex(f => f.id === id);

      if (index === -1) {
        throw new Error(`File with ID ${id} not found`);
      }

      // Update selected status for all files
      for (let i = 0; i < files.length; i++) {
        files[i].selected = (i === index);
      }

      // Save to store
      const store = await getStore();
      await store.set(FILES_STORE_KEYS.FILES, files);
      await store.set(FILES_STORE_KEYS.SELECTED_FILE, id);
      await store.save();

      // Update state
      set({ files, selectedFileId: id });
    } catch (error) {
      console.error(`Failed to select file ${id}:`, error);
      set({ error: String(error) });
    }
  },

  // Getters
  getFileById: (id) => {
    return get().files.find(file => file.id === id);
  },

  getSelectedFile: () => {
    const { files, selectedFileId } = get();
    return files.find(file => file.id === selectedFileId);
  }
}));
