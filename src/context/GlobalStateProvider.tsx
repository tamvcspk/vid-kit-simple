import { ReactNode, useEffect } from 'react';
import useAppStore from '../states/app-state';
import useConversionStore from '../states/conversion-state';
import usePreferencesStore from '../states/preferences-state';

interface GlobalStateProviderProps {
  children: ReactNode;
}

export function GlobalStateProvider({ children }: GlobalStateProviderProps) {
  // Lấy các hàm fetch state từ các store
  const fetchAppState = useAppStore(state => state.fetchAppState);
  const fetchConversionState = useConversionStore(state => state.fetchConversionState);
  const fetchPreferencesState = usePreferencesStore(state => state.fetchPreferencesState);

  // Khởi tạo state khi component mount
  useEffect(() => {
    // Fetch tất cả các state
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
