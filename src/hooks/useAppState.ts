import { useEffect } from "react";
import useAppStore from "../store/app-state";

/**
 * Hook tùy chỉnh để sử dụng AppStore
 * Tự động fetch app state khi component mount
 */
export function useAppState() {
  const {
    data: appState,
    isLoading,
    error,
    fetchAppState,
    setSelectedGpu
  } = useAppStore();

  // Fetch app state khi component mount
  useEffect(() => {
    fetchAppState();
  }, [fetchAppState]);

  return {
    appState,
    loading: isLoading,
    error,
    refreshAppState: fetchAppState,
    setSelectedGpu
  };
}
