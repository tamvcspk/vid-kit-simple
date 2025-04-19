import { invoke } from '@tauri-apps/api/core';

// Interval in milliseconds for task cleanup (e.g., every 5 minutes)
const CLEANUP_INTERVAL = 5 * 60 * 1000;

/**
 * Start a periodic task cleanup process
 * This will run in the background and clean up completed/failed tasks
 * to prevent memory leaks
 */
export function startTaskCleanupScheduler(): void {
  // Run cleanup immediately
  cleanupTasks();
  
  // Set up interval for periodic cleanup
  setInterval(cleanupTasks, CLEANUP_INTERVAL);
}

/**
 * Call the backend to clean up old tasks
 */
async function cleanupTasks(): Promise<void> {
  try {
    await invoke('cleanup_video_tasks');
    console.log('Task cleanup completed');
  } catch (error) {
    console.error('Failed to clean up tasks:', error);
  }
}
