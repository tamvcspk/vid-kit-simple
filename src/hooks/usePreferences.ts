import { useEffect } from 'react';
import useActualPreferencesStore from '../store/preferences-state';

/**
 * Hook tùy chỉnh để sử dụng PreferencesStore
 * Tự động fetch preferences state khi component mount
 */
export function usePreferences() {
  const {
    data: preferences,
    isLoading,
    error,
    fetchPreferencesState,
    updatePreferences,
    savePreferencesToFile,
    loadPreferencesFromFile
  } = useActualPreferencesStore();

  // Fetch preferences state khi component mount
  useEffect(() => {
    fetchPreferencesState();
  }, [fetchPreferencesState]);

  return {
    preferences,
    loading: isLoading,
    error,
    refreshPreferences: fetchPreferencesState,
    updatePreferences,
    savePreferencesToFile,
    loadPreferencesFromFile
  };
}
