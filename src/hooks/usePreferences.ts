import { useEffect } from 'react';
import useActualPreferencesStore from '../store/preferences-state';

/**
 * Custom hook to use PreferencesStore
 * Automatically fetches preferences state when component mounts
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

  // Fetch preferences state when component mounts
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
