import { Store } from '@tauri-apps/plugin-store';

/**
 * Helper function to create and get a store instance
 * @param path The path to the store file
 * @returns A store instance
 */
export async function getStore(path: string): Promise<Store> {
  const store = new Store(path);
  return store;
}

/**
 * Helper function to get a value from a store
 * @param path The path to the store file
 * @param key The key to get
 * @returns The value or undefined if not found
 */
export async function getValue<T>(path: string, key: string): Promise<T | undefined> {
  const store = await getStore(path);
  return store.get(key) as T | undefined;
}

/**
 * Helper function to set a value in a store
 * @param path The path to the store file
 * @param key The key to set
 * @param value The value to set
 */
export async function setValue<T>(path: string, key: string, value: T): Promise<void> {
  const store = await getStore(path);
  await store.set(key, value);
  await store.save();
}

/**
 * Helper function to delete a value from a store
 * @param path The path to the store file
 * @param key The key to delete
 */
export async function deleteValue(path: string, key: string): Promise<void> {
  const store = await getStore(path);
  await store.delete(key);
  await store.save();
}

/**
 * Helper function to clear a store
 * @param path The path to the store file
 */
export async function clearStore(path: string): Promise<void> {
  const store = await getStore(path);
  await store.clear();
  await store.save();
}
