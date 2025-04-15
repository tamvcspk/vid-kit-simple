import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { ConversionState, FileInfo } from '../types/state.types';

// Định nghĩa interface cho ConversionStore
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
  selectFile: (fileId: string) => Promise<void>;
  clearFileList: () => Promise<void>;
  
  // Task management
  addTask: (taskId: string) => Promise<void>;
  markTaskFailed: (taskId: string) => Promise<void>;
}

// Tạo store với devtools middleware
const useConversionStore = create<ConversionStore>()(
  devtools(
    (set) => ({
      // State ban đầu
      data: null,
      isLoading: true,
      error: null,

      // Actions
      setConversionState: (conversionState) => set({ data: conversionState }),
      
      fetchConversionState: async () => {
        try {
          set({ isLoading: true });
          const conversionState = await invoke<ConversionState>('get_conversion_state');
          set({ data: conversionState, error: null });
        } catch (error) {
          set({ error: `Failed to fetch conversion state: ${error}` });
        } finally {
          set({ isLoading: false });
        }
      },
      
      // File management
      addFileToList: async (file) => {
        try {
          const id = crypto.randomUUID();
          await invoke('add_file_to_list', {
            id,
            name: file.name,
            path: file.path,
            size: file.size,
            fileType: file.type,
            duration: file.duration || null,
            resolution: file.resolution ? [file.resolution.width, file.resolution.height] : null,
            thumbnail: file.thumbnail || null
          });
          // State sẽ được cập nhật thông qua event listener
        } catch (error) {
          set({ error: `Failed to add file to list: ${error}` });
        }
      },
      
      removeFileFromList: async (fileId) => {
        try {
          await invoke('remove_file_from_list', { fileId });
          // State sẽ được cập nhật thông qua event listener
        } catch (error) {
          set({ error: `Failed to remove file from list: ${error}` });
        }
      },
      
      selectFile: async (fileId) => {
        try {
          await invoke('select_file', { fileId });
          // State sẽ được cập nhật thông qua event listener
        } catch (error) {
          set({ error: `Failed to select file: ${error}` });
        }
      },
      
      clearFileList: async () => {
        try {
          await invoke('clear_file_list');
          // State sẽ được cập nhật thông qua event listener
        } catch (error) {
          set({ error: `Failed to clear file list: ${error}` });
        }
      },
      
      // Task management
      addTask: async (taskId) => {
        try {
          await invoke('add_conversion_task_wrapper', { taskId });
          // State sẽ được cập nhật thông qua event listener
        } catch (error) {
          set({ error: `Failed to add task: ${error}` });
        }
      },
      
      markTaskFailed: async (taskId) => {
        try {
          await invoke('mark_task_failed_wrapper', { taskId });
          // State sẽ được cập nhật thông qua event listener
        } catch (error) {
          set({ error: `Failed to mark task as failed: ${error}` });
        }
      },
    }),
    { name: 'conversion-store' }
  )
);

// Thiết lập listener cho sự kiện conversion-state-changed
listen<ConversionState>('conversion-state-changed', (event) => {
  useConversionStore.setState({ data: event.payload });
}).catch(console.error);

// Thiết lập listener cho sự kiện conversion-progress
listen<{ task_id: string; progress: number }>('conversion-progress', () => {
  // Không cần cập nhật state ở đây vì conversion-state-changed sẽ được emit
}).catch(console.error);

export default useConversionStore;
