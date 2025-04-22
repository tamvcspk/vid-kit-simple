// New stores
export { useAppStore } from './app.store';
export { useConfigStore } from './config.store';
export { usePresetsStore } from './presets.store';
export { useTasksStore } from './tasks.store';
export { useFilesStore } from './files.store';

// Legacy stores - will be removed in future versions
import useAppStore from './app-state';
export { useAppStore as useAppStateStore };

import useConversionStore from './conversion-state';
export { useConversionStore };

import usePreferencesStore from './preferences-state';
export { usePreferencesStore };
