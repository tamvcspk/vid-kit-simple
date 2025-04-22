import { useEffect } from 'react';
import { useTasksStore, usePresetsStore } from '../store';

/**
 * Hook to access and manage conversion state
 * Automatically loads tasks and presets when component mounts
 */
export function useConversionState() {
  const {
    tasks,
    queue,
    isLoading: tasksLoading,
    error: tasksError,
    loadTasks,
    addTask,
    updateTask,
    removeTask,
    clearCompletedTasks,
    reorderTasks,
    startTask,
    pauseTask,
    resumeTask,
    cancelTask,
    retryTask,
    startQueue,
    pauseQueue,
    cancelQueue,
    getTaskById,
    getPendingTasks,
    getRunningTasks,
    getCompletedTasks,
    getFailedTasks
  } = useTasksStore();

  const {
    presets,
    isLoading: presetsLoading,
    error: presetsError,
    loadPresets,
    savePreset,
    deletePreset,
    createDefaultPresets,
    selectPreset,
    createPreset,
    duplicatePreset,
    getPresetById,
    selectedPresetId
  } = usePresetsStore();

  // Load tasks and presets on mount
  useEffect(() => {
    loadTasks();
    loadPresets();
    createDefaultPresets();
  }, [loadTasks, loadPresets, createDefaultPresets]);

  return {
    // Tasks state and actions
    tasks,
    queue,
    tasksLoading,
    tasksError,
    addTask,
    updateTask,
    removeTask,
    clearCompletedTasks,
    reorderTasks,
    startTask,
    pauseTask,
    resumeTask,
    cancelTask,
    retryTask,
    startQueue,
    pauseQueue,
    cancelQueue,
    getTaskById,
    getPendingTasks,
    getRunningTasks,
    getCompletedTasks,
    getFailedTasks,
    refreshTasks: loadTasks,

    // Presets state and actions
    presets,
    presetsLoading,
    presetsError,
    savePreset,
    deletePreset,
    selectPreset,
    createPreset,
    duplicatePreset,
    getPresetById,
    selectedPresetId,
    refreshPresets: loadPresets
  };
}
