import { ReactNode, useEffect } from 'react';
import useAppStore from '../store/app-state';
import useConversionStore from '../store/conversion-state';
import usePreferencesStore from '../store/preferences-state';

interface GlobalStateProviderProps {
  children: ReactNode;
}

export function GlobalStateProvider({ children }: GlobalStateProviderProps) {
  // Get fetch state functions from stores
  const fetchAppState = useAppStore(state => state.fetchAppState);
  const fetchConversionState = useConversionStore(state => state.fetchConversionState);
  const fetchPreferencesState = usePreferencesStore(state => state.fetchPreferencesState);

  // Initialize state when component mounts
  useEffect(() => {
    // Fetch all states
    Promise.all([
      fetchAppState(),
      fetchConversionState(),
      fetchPreferencesState()
    ]).catch(error => {
      console.error('Failed to initialize state:', error);
    });
  }, [fetchAppState, fetchConversionState, fetchPreferencesState]);

  return <>{children}</>;
}
