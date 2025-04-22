import { useEffect } from 'react';
import { listen, UnlistenFn } from '@tauri-apps/api/event';

/**
 * Hook to listen to Tauri events
 * Automatically cleans up the listener when the component unmounts
 * 
 * @param event The event name to listen to
 * @param handler The handler function to call when the event is emitted
 */
export function useTauriEvent<T>(
  event: string,
  handler: (data: T) => void
) {
  useEffect(() => {
    let unlistenFn: UnlistenFn | undefined;
    
    // Set up the event listener
    const setupListener = async () => {
      unlistenFn = await listen<T>(event, (e) => {
        handler(e.payload);
      });
    };
    
    setupListener();
    
    // Clean up the event listener when the component unmounts
    return () => {
      if (unlistenFn) {
        unlistenFn();
      }
    };
  }, [event, handler]);
}
