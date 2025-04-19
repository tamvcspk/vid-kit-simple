use std::sync::{Mutex, MutexGuard};
use std::fmt::Display;
use log;
use parking_lot::Mutex as ParkingMutex;
use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::state::errors::{StateError, StateResult};

/// Helper function to safely handle mutex locks and recover from poisoned mutexes
///
/// Instead of using `.unwrap()` which would panic on a poisoned mutex,
/// this function recovers the inner value and logs a warning.
///
/// # Arguments
/// * `lock_result` - The result of calling `.lock()` on a mutex
///
/// # Returns
/// * `MutexGuard<T>` - The mutex guard, either from a successful lock or recovered from a poisoned mutex
///
/// # Example
/// ```
/// let mutex = Mutex::new(42);
/// let guard = lock_or_recover(mutex.lock());
/// ```
pub fn lock_or_recover<T>(lock_result: std::sync::LockResult<MutexGuard<'_, T>>)
    -> MutexGuard<'_, T> {
    match lock_result {
        Ok(guard) => guard,
        Err(poisoned) => {
            log::warn!("Mutex was poisoned, recovering state");
            poisoned.into_inner()
        }
    }
}

/// Extension trait to add safe locking methods to Mutex
pub trait SafeMutex<T> {
    /// Safely lock the mutex, recovering from poison if needed
    fn safe_lock(&self) -> MutexGuard<'_, T>;
}

impl<T> SafeMutex<T> for Mutex<T> {
    fn safe_lock(&self) -> MutexGuard<'_, T> {
        lock_or_recover(self.lock())
    }
}

/// Helper function to log an error and return it as a String
///
/// This function logs the error with context and returns it as a String
/// for consistent error handling throughout the application.
///
/// # Arguments
/// * `error` - The error to log and return
/// * `context` - Additional context for the error
///
/// # Returns
/// * `String` - The formatted error message
///
/// # Example
/// ```
/// let result = some_operation().map_err(|e| log_error(e, "Failed to perform operation"));
/// ```
pub fn log_error<E: Display>(error: E, context: &str) -> String {
    let error_msg = format!("{}: {}", context, error);
    log::error!("{}", error_msg);
    error_msg
}

/// Helper function to log an error and return it as a Result
///
/// This function logs the error with context and returns it as a Result<T, String>
/// for consistent error handling throughout the application.
///
/// # Arguments
/// * `error` - The error to log and return
/// * `context` - Additional context for the error
///
/// # Returns
/// * `Result<T, String>` - Err containing the formatted error message
///
/// # Example
/// ```
/// let result = some_operation().map_err(|e| log_and_return_error::<(), _>(e, "Failed to perform operation"));
/// ```
pub fn log_and_return_error<T, E: Display>(error: E, context: &str) -> Result<T, String> {
    Err(log_error(error, context))
}

/// Helper function to update state, clone it, and emit an event
///
/// This function handles the common pattern:
/// 1. Lock the mutex
/// 2. Update the state
/// 3. Clone the state for emitting
/// 4. Drop the lock
/// 5. Emit an event with the updated state
///
/// # Arguments
/// * `mutex` - The mutex containing the state
/// * `app` - The AppHandle for emitting events
/// * `event` - The event name to emit
/// * `update` - A function that updates the state and returns a Result
///
/// # Returns
/// * `StateResult<()>` - Success or error
///
/// # Example
/// ```
/// with_state(&conversion.state, &app_handle, "conversion-state-changed", |state| {
///     state.files.push(file_info);
///     Ok(())
/// })
/// ```
pub fn with_state<S, F>(
    mutex: &ParkingMutex<S>,
    app: &AppHandle,
    event: &str,
    update: F
) -> StateResult<()>
where 
    S: Clone + Serialize,
    F: FnOnce(&mut S) -> StateResult<()>
{
    // Get lock (parking_lot::Mutex doesn't return a Result and can't be poisoned)
    let mut state = mutex.lock();
    
    // Update the state
    update(&mut state)?;
    
    // Clone state for emit
    let state_snapshot = state.clone();
    
    // Lock is automatically dropped at the end of scope
    
    // Emit event
    app.emit(event, state_snapshot)
        .map_err(StateError::from)?;
    
    Ok(())
}

/// Helper function to log a state error
///
/// This function logs the state error with context
///
/// # Arguments
/// * `error` - The error to log
/// * `context` - Additional context for the error
///
/// # Example
/// ```
/// log_state_error(&error, "Failed to update conversion state");
/// ```
pub fn log_state_error<E: std::fmt::Display>(error: &E, context: &str) {
    log::error!("{}: {}", context, error);
}
