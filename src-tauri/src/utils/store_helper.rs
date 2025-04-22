use log::{error, info};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt;
use crate::utils::error::{AppError, AppResult, ErrorCode};

/// Constants for store file paths
pub const TASKS_STORE_PATH: &str = "tasks.json";
pub const CONFIG_STORE_PATH: &str = "config.json";
pub const PRESETS_STORE_PATH: &str = "presets.json";

/// Helper function to get a store
pub fn get_store<R: Runtime>(
    app_handle: &AppHandle<R>,
    path: &str,
) -> AppResult<Arc<tauri_plugin_store::Store<R>>> {
    app_handle.store(path).map_err(|e| {
        error!("Failed to get store {}: {}", path, e);
        AppError::state_error(
            format!("Failed to get store: {}", e),
            ErrorCode::StateAccessError,
            Some(format!("Error accessing store file: {}", path))
        )
    })
}

/// Helper function to get a value from a store
pub fn get_value<R: Runtime, T: DeserializeOwned>(
    app_handle: &AppHandle<R>,
    path: &str,
    key: &str,
) -> AppResult<Option<T>> {
    let store = get_store(app_handle, path)?;

    match store.get(key) {
        Some(value) => {
            serde_json::from_value::<T>(value.clone()).map_err(|e| {
                error!("Failed to deserialize value for key {}: {}", key, e);
                AppError::state_error(
                    format!("Failed to deserialize value: {}", e),
                    ErrorCode::StateSerializationError,
                    Some(format!("Error deserializing data for key '{}' in store '{}'", key, path))
                )
            }).map(Some)
        },
        None => Ok(None),
    }
}

/// Helper function to set a value in a store
pub fn set_value<R: Runtime, T: Serialize>(
    app_handle: &AppHandle<R>,
    path: &str,
    key: &str,
    value: &T,
) -> AppResult<()> {
    let store = get_store(app_handle, path)?;

    // Serialize the value to JSON
    let json_value = serde_json::to_value(value).map_err(|e| {
        error!("Failed to serialize value for key {}: {}", key, e);
        AppError::state_error(
            format!("Failed to serialize value: {}", e),
            ErrorCode::StateSerializationError,
            Some(format!("Error serializing data for key '{}' in store '{}'", key, path))
        )
    })?;

    // Set the value in the store
    store.set(key, json_value);

    // Save the store
    store.save().map_err(|e| {
        error!("Failed to save store {}: {}", path, e);
        AppError::state_error(
            format!("Failed to save store: {}", e),
            ErrorCode::StateSerializationError,
            Some(format!("Error saving store file: {}", path))
        )
    })?;

    info!("Successfully saved value for key {} in store {}", key, path);
    Ok(())
}

/// Helper function to delete a value from a store
pub fn delete_value<R: Runtime>(
    app_handle: &AppHandle<R>,
    path: &str,
    key: &str,
) -> AppResult<()> {
    let store = get_store(app_handle, path)?;

    // Delete the key from the store
    if !store.delete(key) {
        error!("Failed to delete key {}: Key not found", key);
        return Err(AppError::state_error(
            format!("Failed to delete key: Key not found"),
            ErrorCode::StateMutationError,
            Some(format!("Key '{}' not found in store '{}'", key, path))
        ));
    }

    // Save the store
    store.save().map_err(|e| {
        error!("Failed to save store {}: {}", path, e);
        AppError::state_error(
            format!("Failed to save store: {}", e),
            ErrorCode::StateSerializationError,
            Some(format!("Error saving store file: {}", path))
        )
    })?;

    info!("Successfully deleted key {} from store {}", key, path);
    Ok(())
}

/// Helper function to clear a store
pub fn clear_store<R: Runtime>(
    app_handle: &AppHandle<R>,
    path: &str,
) -> AppResult<()> {
    let store = get_store(app_handle, path)?;

    // Clear the store
    store.clear();

    // Save the store
    store.save().map_err(|e| {
        error!("Failed to save store {}: {}", path, e);
        AppError::state_error(
            format!("Failed to save store: {}", e),
            ErrorCode::StateSerializationError,
            Some(format!("Error saving store file: {}", path))
        )
    })?;

    info!("Successfully cleared store {}", path);
    Ok(())
}

/// Helper function to check if a store exists
pub fn store_exists<R: Runtime>(
    app_handle: &AppHandle<R>,
    path: &str,
) -> AppResult<bool> {
    match app_handle.store(path) {
        Ok(_) => Ok(true),
        Err(e) => {
            // If the error is because the file doesn't exist, return false
            // Otherwise, propagate the error
            if e.to_string().contains("No such file") {
                Ok(false)
            } else {
                error!("Failed to check if store exists {}: {}", path, e);
                Err(AppError::state_error(
                    format!("Failed to check if store exists: {}", e),
                    ErrorCode::StateAccessError,
                    Some(format!("Error checking store file: {}", path))
                ))
            }
        }
    }
}
