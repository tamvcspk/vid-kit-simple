import { useEffect } from "react";
import useAppStore from "../store/app-state";

/**
 * Custom hook to use AppStore
 * Automatically fetches app state when component mounts
 */
export function useAppState() {
  const {
    data: appState,
    isLoading,
    error,
    fetchAppState,
    setSelectedGpu
  } = useAppStore();

  // Fetch app state when component mounts
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
