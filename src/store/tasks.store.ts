import { create } from 'zustand';
import { Store } from '@tauri-apps/plugin-store';
import { Task, TaskStatus } from '../types/store.types';
import { TASKS_STORE_PATH, TASKS_STORE_KEYS } from '../constants/stores';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { v4 as uuidv4 } from 'uuid';

// Create a store instance
let storePromise: Promise<Store> | null = null;

// Function to get the store instance
const getStore = async (): Promise<Store> => {
  if (!storePromise) {
    storePromise = Store.load(TASKS_STORE_PATH);
  }
  return storePromise;
};

interface TasksState {
  tasks: Task[];
  queue: string[]; // Array of task IDs in queue order
  isLoading: boolean;
  error: string | null;

  // Actions
  loadTasks: () => Promise<void>;
  refreshTasks: () => Promise<void>; // Added refreshTasks method
  addTask: (task: Omit<Task, 'id' | 'status' | 'progress' | 'attempts' | 'created_at'>) => Promise<string>;
  updateTask: (id: string, updates: Partial<Task>) => Promise<void>;
  removeTask: (id: string) => Promise<void>;
  clearCompletedTasks: () => Promise<void>;
  reorderTasks: (newOrder: string[]) => Promise<void>;

  // Task operations
  startTask: (id: string) => Promise<void>;
  pauseTask: (id: string) => Promise<void>;
  resumeTask: (id: string) => Promise<void>;
  cancelTask: (id: string) => Promise<void>;
  retryTask: (id: string) => Promise<void>;
  startQueue: () => Promise<void>;
  pauseQueue: () => Promise<void>;
  cancelQueue: () => Promise<void>;

  // Getters
  getTaskById: (id: string) => Task | undefined;
  getPendingTasks: () => Task[];
  getRunningTasks: () => Task[];
  getCompletedTasks: () => Task[];
  getFailedTasks: () => Task[];
  getCanceledTasks: () => Task[];
  getPausedTasks: () => Task[];
  is_queue_paused: boolean;
}

export const useTasksStore = create<TasksState>((set, get) => ({
  // State
  tasks: [],
  queue: [],
  isLoading: false,
  error: null,
  is_queue_paused: false,

  // Actions
  refreshTasks: async () => {
    set({ isLoading: true, error: null });
    try {
      // Fetch latest tasks from backend
      await invoke('get_tasks')
        .then((backendTasks: any) => {
          if (backendTasks && Array.isArray(backendTasks)) {
            // Update local state with backend tasks
            set({ tasks: backendTasks, isLoading: false });
            console.log('Tasks refreshed from backend');

            // Save to store
            const store = getStore();
            store.then(s => {
              s.set(TASKS_STORE_KEYS.TASKS, backendTasks);
              s.save();
            });

            // Also get the queue
            invoke<string[]>('get_queue')
              .then((queue) => {
                set({ queue });

                // Save to store
                store.then(s => {
                  s.set(TASKS_STORE_KEYS.QUEUE, queue);
                  s.save();
                });
              })
              .catch(error => {
                console.error('Failed to refresh queue from backend:', error);
              });
          }
        })
        .catch(error => {
          console.error('Failed to refresh tasks from backend:', error);
          // Fall back to loading from store
          get().loadTasks();
        });
    } catch (error) {
      console.error('Failed to refresh tasks:', error);
      set({ error: String(error), isLoading: false });

      // Fall back to loading from store
      get().loadTasks();
    }
  },

  loadTasks: async () => {
    set({ isLoading: true, error: null });
    try {
      const store = await getStore();
      const tasks = await store.get(TASKS_STORE_KEYS.TASKS) as Task[] || [];
      const queue = await store.get(TASKS_STORE_KEYS.QUEUE) as string[] || [];

      // Try to get queue paused state from backend
      try {
        const is_queue_paused = await invoke<boolean>('is_queue_paused');
        set({ tasks, queue, is_queue_paused, isLoading: false });
      } catch (error) {
        // If backend call fails, use stored value or default to false
        set({ tasks, queue, isLoading: false });
      }

      console.log('Tasks loaded');

      // Set up event listeners for task updates
      const unlistenTaskProgress = await listen('task-progress', (event) => {
        const { taskId, progress } = event.payload as { taskId: string; progress: number };
        get().updateTask(taskId, { progress });
      });

      const unlistenTaskCompleted = await listen('task-completed', (event) => {
        const taskId = event.payload as string;
        get().updateTask(taskId, {
          status: TaskStatus.Completed,
          progress: 100,
          completed_at: new Date().toISOString()
        });
      });

      const unlistenTaskFailed = await listen('task-failed', (event) => {
        const { taskId, error } = event.payload as { taskId: string; error: string };
        get().updateTask(taskId, {
          status: TaskStatus.Failed,
          error,
          completed_at: new Date().toISOString()
        });
      });

      // Listen for queue state changes
      const unlistenQueueStateChanged = await listen('queue-state-changed', (event) => {
        const { is_paused } = event.payload as { is_paused: boolean };
        set({ is_queue_paused: is_paused });
      });

      // Clean up listeners when component unmounts
      window.addEventListener('beforeunload', () => {
        unlistenTaskProgress();
        unlistenTaskCompleted();
        unlistenTaskFailed();
        unlistenQueueStateChanged();
      });

    } catch (error) {
      console.error('Failed to load tasks:', error);
      set({ error: String(error), isLoading: false });
    }
  },

  addTask: async (taskData) => {
    set({ isLoading: true, error: null });
    try {
      const store = await getStore();

      const task: Task = {
        id: uuidv4(),
        status: TaskStatus.Pending,
        progress: 0,
        attempts: 0,
        created_at: new Date().toISOString(),
        ...taskData
      };

      // Add to tasks array
      const tasks = [...get().tasks, task];

      // Add to queue
      const queue = [...get().queue, task.id];

      // Save to store
      await store.set(TASKS_STORE_KEYS.TASKS, tasks);
      await store.set(TASKS_STORE_KEYS.QUEUE, queue);
      await store.save();

      // Update state
      set({ tasks, queue, isLoading: false });

      // Create task in backend
      await invoke('create_task', {
        inputPath: task.input_path,
        outputPath: task.output_path,
        config: task.config,
        taskId: task.id
      });

      return task.id;
    } catch (error) {
      console.error('Failed to add task:', error);
      set({ error: String(error), isLoading: false });
      throw error;
    }
  },

  updateTask: async (id, updates) => {
    try {
      const tasks = [...get().tasks];
      const index = tasks.findIndex(t => t.id === id);

      if (index === -1) {
        throw new Error(`Task with ID ${id} not found`);
      }

      // Update task
      tasks[index] = { ...tasks[index], ...updates };

      // Save to store
      const store = await getStore();
      await store.set(TASKS_STORE_KEYS.TASKS, tasks);
      await store.save();

      // Update state
      set({ tasks });
    } catch (error) {
      console.error(`Failed to update task ${id}:`, error);
      set({ error: String(error) });
    }
  },

  removeTask: async (id) => {
    set({ isLoading: true, error: null });
    try {
      // Get current state
      const tasks = get().tasks.filter(t => t.id !== id);
      const queue = get().queue.filter(taskId => taskId !== id);

      // Save to store
      const store = await getStore();
      await store.set(TASKS_STORE_KEYS.TASKS, tasks);
      await store.set(TASKS_STORE_KEYS.QUEUE, queue);
      await store.save();

      // Update state
      set({ tasks, queue, isLoading: false });
    } catch (error) {
      console.error('Failed to remove task:', error);
      set({ error: String(error), isLoading: false });
    }
  },

  clearCompletedTasks: async () => {
    set({ isLoading: true, error: null });
    try {
      // Filter out completed tasks
      const tasks = get().tasks.filter(
        t => t.status !== TaskStatus.Completed && t.status !== TaskStatus.Canceled
      );

      // Queue should already not contain completed tasks, but filter just in case
      const queue = get().queue.filter(
        id => tasks.some(t => t.id === id && t.status === TaskStatus.Pending)
      );

      // Save to store
      const store = await getStore();
      await store.set(TASKS_STORE_KEYS.TASKS, tasks);
      await store.set(TASKS_STORE_KEYS.QUEUE, queue);
      await store.save();

      // Update state
      set({ tasks, queue, isLoading: false });
    } catch (error) {
      console.error('Failed to clear completed tasks:', error);
      set({ error: String(error), isLoading: false });
    }
  },

  reorderTasks: async (newOrder) => {
    set({ isLoading: true, error: null });
    try {
      // Validate that all IDs in newOrder exist in the queue
      const currentQueue = get().queue;
      const isValid = newOrder.every(id => currentQueue.includes(id));

      if (!isValid) {
        throw new Error('Invalid task order: contains IDs not in the current queue');
      }

      // Save to store
      const store = await getStore();
      await store.set(TASKS_STORE_KEYS.QUEUE, newOrder);
      await store.save();

      // Update state
      set({ queue: newOrder, isLoading: false });

      // Update backend queue order
      await invoke('reorder_tasks', { newOrder });
    } catch (error) {
      console.error('Failed to reorder tasks:', error);
      set({ error: String(error), isLoading: false });
    }
  },

  // Task operations
  startTask: async (id) => {
    try {
      await invoke('run_task', { taskId: id });

      // Update local state
      get().updateTask(id, { status: TaskStatus.Running });
    } catch (error) {
      console.error(`Failed to start task ${id}:`, error);
      set({ error: String(error) });
    }
  },

  pauseTask: async (id) => {
    try {
      await invoke('pause_task', { taskId: id });

      // Update local state
      get().updateTask(id, { status: TaskStatus.Paused });
    } catch (error) {
      console.error(`Failed to pause task ${id}:`, error);
      set({ error: String(error) });
    }
  },

  resumeTask: async (id) => {
    try {
      await invoke('resume_task', { taskId: id });

      // Update local state
      get().updateTask(id, { status: TaskStatus.Running });
    } catch (error) {
      console.error(`Failed to resume task ${id}:`, error);
      set({ error: String(error) });
    }
  },

  cancelTask: async (id) => {
    try {
      await invoke('cancel_task', { taskId: id });

      // Update local state
      get().updateTask(id, {
        status: TaskStatus.Canceled,
        completed_at: new Date().toISOString()
      });

      // Remove from queue if present
      const queue = get().queue.filter(taskId => taskId !== id);
      set({ queue });

      // Update store
      const store = await getStore();
      await store.set(TASKS_STORE_KEYS.QUEUE, queue);
      await store.save();
    } catch (error) {
      console.error(`Failed to cancel task ${id}:`, error);
      set({ error: String(error) });
    }
  },

  retryTask: async (id) => {
    try {
      const task = get().getTaskById(id);
      if (!task) {
        throw new Error(`Task with ID ${id} not found`);
      }

      // Reset task status
      await get().updateTask(id, {
        status: TaskStatus.Pending,
        progress: 0,
        error: undefined,
        attempts: task.attempts + 1
      });

      // Add back to queue if not already there
      if (!get().queue.includes(id)) {
        const queue = [...get().queue, id];
        set({ queue });

        // Update store
        const store = await getStore();
        await store.set(TASKS_STORE_KEYS.QUEUE, queue);
        await store.save();
      }

      // Invoke backend retry
      await invoke('retry_task', { taskId: id });
    } catch (error) {
      console.error(`Failed to retry task ${id}:`, error);
      set({ error: String(error) });
    }
  },

  startQueue: async () => {
    try {
      await invoke('start_queue');

      // Update status of all pending tasks in queue to running
      const { tasks, queue } = get();
      const updatedTasks = [...tasks];

      for (const taskId of queue) {
        const index = updatedTasks.findIndex(t => t.id === taskId);
        if (index !== -1 && updatedTasks[index].status === TaskStatus.Pending) {
          updatedTasks[index] = {
            ...updatedTasks[index],
            status: TaskStatus.Running
          };
        }
      }

      set({ tasks: updatedTasks });
    } catch (error) {
      console.error('Failed to start queue:', error);
      set({ error: String(error) });
    }
  },

  pauseQueue: async () => {
    try {
      await invoke('pause_queue');

      // Update status of all running tasks to paused
      const updatedTasks = get().tasks.map(task =>
        task.status === TaskStatus.Running
          ? { ...task, status: TaskStatus.Paused }
          : task
      );

      set({ tasks: updatedTasks });
    } catch (error) {
      console.error('Failed to pause queue:', error);
      set({ error: String(error) });
    }
  },

  cancelQueue: async () => {
    try {
      await invoke('cancel_queue');

      // Update status of all pending and running tasks to canceled
      const updatedTasks = get().tasks.map(task =>
        (task.status === TaskStatus.Pending || task.status === TaskStatus.Running || task.status === TaskStatus.Paused)
          ? {
              ...task,
              status: TaskStatus.Canceled,
              completed_at: new Date().toISOString()
            }
          : task
      );

      // Clear the queue
      set({ tasks: updatedTasks, queue: [] });

      // Update store
      const store = await getStore();
      await store.set(TASKS_STORE_KEYS.TASKS, updatedTasks);
      await store.set(TASKS_STORE_KEYS.QUEUE, []);
      await store.save();
    } catch (error) {
      console.error('Failed to cancel queue:', error);
      set({ error: String(error) });
    }
  },

  // Getters
  getTaskById: (id) => {
    return get().tasks.find(task => task.id === id);
  },

  getPendingTasks: () => {
    return get().tasks.filter(task => task.status === TaskStatus.Pending);
  },

  getRunningTasks: () => {
    return get().tasks.filter(task => task.status === TaskStatus.Running);
  },

  getCompletedTasks: () => {
    return get().tasks.filter(task => task.status === TaskStatus.Completed);
  },

  getFailedTasks: () => {
    return get().tasks.filter(task => task.status === TaskStatus.Failed);
  },

  getCanceledTasks: () => {
    return get().tasks.filter(task => task.status === TaskStatus.Canceled);
  },

  getPausedTasks: () => {
    return get().tasks.filter(task => task.status === TaskStatus.Paused);
  }
}));
