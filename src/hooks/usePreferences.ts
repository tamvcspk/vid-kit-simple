import { usePreferencesStore } from './usePreferencesStore';

export function usePreferences() {
  // Sử dụng hook mới
  return usePreferencesStore();
}
