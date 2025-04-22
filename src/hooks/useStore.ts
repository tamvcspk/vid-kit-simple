import { useEffect, useState } from 'react';
import { Store } from '@tauri-apps/plugin-store';

/**
 * Hook to use a Tauri store
 * @param path The path to the store file
 * @param key The key to get/set
 * @param defaultValue The default value to use if the key doesn't exist
 * @returns [value, setValue, loading, error]
 */
export function useStore<T>(
  path: string,
  key: string,
  defaultValue: T
): [T, (value: T) => Promise<void>, boolean, string | null] {
  const [value, setValue] = useState<T>(defaultValue);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  // Load the value from the store
  useEffect(() => {
    const loadValue = async () => {
      try {
        setLoading(true);
        setError(null);
        
        const store = new Store(path);
        const storedValue = await store.get(key);
        
        if (storedValue !== null && storedValue !== undefined) {
          setValue(storedValue as T);
        } else {
          setValue(defaultValue);
        }
      } catch (err) {
        console.error(`Error loading value from store ${path}:${key}`, err);
        setError(String(err));
      } finally {
        setLoading(false);
      }
    };

    loadValue();
  }, [path, key, defaultValue]);

  // Function to update the value in the store
  const updateValue = async (newValue: T) => {
    try {
      setLoading(true);
      setError(null);
      
      const store = new Store(path);
      await store.set(key, newValue);
      await store.save();
      
      setValue(newValue);
    } catch (err) {
      console.error(`Error saving value to store ${path}:${key}`, err);
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  return [value, updateValue, loading, error];
}
