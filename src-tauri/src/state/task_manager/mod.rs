mod errors;
mod processor;

use std::collections::{HashMap, VecDeque, HashSet};
use std::sync::{Arc, Mutex, Condvar};
use parking_lot::RwLock;
use tauri::{AppHandle, Manager, Emitter};
use uuid::Uuid;
use chrono::Utc;
use log::info;
use serde::{Serialize, Deserialize};
use tokio::sync::Semaphore;
use serde_json::json;

use crate::utils::error::{AppError, ErrorCode};
use crate::utils::store_helper::{self, TASKS_STORE_PATH};

pub use errors::{TaskError, TaskResult};
pub use processor::TaskProcessor;

/// Status of a task
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
    Canceled,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "pending"),
            TaskStatus::Running => write!(f, "running"),
            TaskStatus::Paused => write!(f, "paused"),
            TaskStatus::Completed => write!(f, "completed"),
            TaskStatus::Failed => write!(f, "failed"),
            TaskStatus::Canceled => write!(f, "canceled"),
        }
    }
}

/// Represents a processing task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub input_path: String,
    pub output_path: String,
    pub status: TaskStatus,
    pub progress: f32,
    pub error: Option<String>,
    pub attempts: usize,
    pub created_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub task_type: String,
    pub config: HashMap<String, String>,
}

/// Manages tasks and their execution
pub struct TaskManager {
    tasks: RwLock<Vec<Task>>,
    queue: RwLock<VecDeque<String>>,
    max_concurrent_tasks: RwLock<usize>,
    is_queue_paused: RwLock<bool>,
    semaphore: RwLock<Arc<Semaphore>>,
    pause_condvar: Arc<(Mutex<HashSet<String>>, Condvar)>,
    task_processor: TaskProcessor,
}

impl TaskManager {
    /// Create a new TaskManager
    pub fn new(max_concurrent_tasks: usize) -> Self {
        Self {
            tasks: RwLock::new(Vec::new()),
            queue: RwLock::new(VecDeque::new()),
            max_concurrent_tasks: RwLock::new(max_concurrent_tasks),
            is_queue_paused: RwLock::new(false),
            semaphore: RwLock::new(Arc::new(Semaphore::new(max_concurrent_tasks))),
            pause_condvar: Arc::new((Mutex::new(HashSet::new()), Condvar::new())),
            task_processor: TaskProcessor::new(),
        }
    }

    /// Create a new task
    pub fn create_task(
        &self,
        input_path: String,
        output_path: String,
        task_type: String,
        config: HashMap<String, String>,
    ) -> TaskResult<String> {
        // Generate a unique ID for the task
        let task_id = Uuid::new_v4().to_string();

        // Create the task
        let task = Task {
            id: task_id.clone(),
            input_path,
            output_path,
            status: TaskStatus::Pending,
            progress: 0.0,
            error: None,
            attempts: 0,
            created_at: Utc::now().to_rfc3339(),
            started_at: None,
            completed_at: None,
            task_type,
            config,
        };

        // Add task to the tasks list
        {
            let mut tasks = self.tasks.write();
            tasks.push(task);
        }

        // Add task to the queue
        {
            let mut queue = self.queue.write();
            queue.push_back(task_id.clone());
        }

        Ok(task_id)
    }

    /// Get a task by ID
    pub fn get_task(&self, task_id: &str) -> TaskResult<Task> {
        let tasks = self.tasks.read();

        tasks.iter()
            .find(|task| task.id == task_id)
            .cloned()
            .ok_or_else(|| TaskError::TaskNotFound(task_id.to_string()))
    }

    /// Get a task by ID and update it
    pub fn update_task<F>(&self, task_id: &str, update_fn: F) -> TaskResult<()>
    where
        F: FnOnce(&mut Task)
    {
        let mut tasks = self.tasks.write();

        let task = tasks.iter_mut()
            .find(|task| task.id == task_id)
            .ok_or_else(|| TaskError::TaskNotFound(task_id.to_string()))?;

        update_fn(task);

        Ok(())
    }

    /// Get all tasks
    pub fn get_all_tasks(&self) -> Vec<Task> {
        let tasks = self.tasks.read();
        tasks.clone()
    }

    /// Get the task queue
    pub fn get_queue(&self) -> Vec<String> {
        let queue = self.queue.read();
        queue.iter().cloned().collect()
    }

    /// Start a task
    pub fn start_task(&self, task_id: &str, app_handle: &AppHandle) -> TaskResult<()> {
        // Create a clone of the task to process
        let task_clone = {
            // Get the task
            let task = self.get_task(task_id)?;

            // Check task status
            if task.status != TaskStatus::Pending {
                return Err(TaskError::InvalidStatus(format!(
                    "Task {} is not in pending state", task_id
                )));
            }

            // Clone the task
            task.clone()
        };

        // Update the task
        self.update_task(task_id, |task| {
            task.status = TaskStatus::Running;
            task.started_at = Some(Utc::now().to_rfc3339());
            task.attempts += 1;
        })?;

        // Save state
        self.save_state(app_handle)?;

        // Emit task-started event
        emit_event(app_handle, "task-started", Some(json!({
            "taskId": task_id
        })));

        // Create a clone of app_handle to use in thread
        let app_handle_clone = app_handle.clone();

        // Create a clone of task_processor to use in thread
        let task_processor = self.task_processor.clone();

        // Create a clone of semaphore to use in thread
        let semaphore = self.semaphore.read().clone();

        // Create a thread to process the task
        tokio::spawn(async move {
            // Acquire a permit from the semaphore
            let _permit = semaphore.acquire().await.unwrap();

            // Process the task
            let result = task_processor.process_task(&task_clone, &app_handle_clone).await;

            // Update task status after processing
            match result {
                Ok(_) => {
                    // Update task status to completed
                    update_task_status(
                        &app_handle_clone,
                        &task_clone.id,
                        TaskStatus::Completed,
                        100.0,
                        None,
                    ).await;

                    // Emit task-completed event
                    emit_event(&app_handle_clone, "task-completed", Some(json!({
                        "taskId": task_clone.id
                    })));
                },
                Err(e) => {
                    // Update task status to failed
                    update_task_status(
                        &app_handle_clone,
                        &task_clone.id,
                        TaskStatus::Failed,
                        task_clone.progress,
                        Some(e.to_string()),
                    ).await;

                    // Emit task-failed event
                    emit_event(&app_handle_clone, "task-failed", Some(json!({
                        "taskId": task_clone.id,
                        "error": e.to_string()
                    })));
                }
            }

            // Process next tasks in queue
            let task_manager = app_handle_clone.state::<TaskManager>();
            let _ = task_manager.inner().process_next_tasks(&app_handle_clone);
        });

        Ok(())
    }

    /// Process next tasks in queue
    /// Note: This method now takes &self instead of &mut self to allow calling it from a thread
    pub fn process_next_tasks(&self, app_handle: &AppHandle) -> TaskResult<()> {
        // If queue is paused, do nothing
        if *self.is_queue_paused.read() {
            return Ok(());
        }

        // Count running tasks
        let running_count = {
            let tasks = self.tasks.read();
            tasks.iter()
                .filter(|task| task.status == TaskStatus::Running)
                .count()
        };

        // Calculate available slots
        let available_slots = self.max_concurrent_tasks.read().saturating_sub(running_count);

        if available_slots == 0 {
            return Ok(());
        }

        // Get pending tasks
        let pending_tasks = {
            let tasks = self.tasks.read();
            let queue = self.queue.read();

            // Get tasks in queue order
            let mut pending = Vec::new();
            for task_id in queue.iter() {
                if let Some(task) = tasks.iter().find(|t| t.id == *task_id) {
                    if task.status == TaskStatus::Pending {
                        pending.push(task.id.clone());
                    }
                }
            }
            pending
        };

        // Since we can't call start_task directly (it requires &mut self),
        // we'll emit an event for each task that should be started
        let mut started = 0;
        for task_id in pending_tasks {
            if started >= available_slots {
                break;
            }

            // Emit an event to start the task
            emit_event(app_handle, "start-task", Some(json!({
                "taskId": task_id
            })));

            started += 1;
        }

        Ok(())
    }

    /// Start the queue
    pub fn start_queue(&self, app_handle: &AppHandle) -> TaskResult<()> {
        // If queue is paused, resume it
        if *self.is_queue_paused.read() {
            self.resume_queue(app_handle)?;
            return Ok(());
        }

        // Process next tasks
        self.process_next_tasks(app_handle)?;

        // Emit queue-started event
        emit_event(app_handle, "queue-started", None);

        Ok(())
    }

    /// Pause the queue
    pub fn pause_queue(&self, app_handle: &AppHandle) -> TaskResult<()> {
        // Update is_queue_paused
        *self.is_queue_paused.write() = true;

        // Collect IDs of running tasks
        let running_task_ids = {
            let tasks = self.tasks.read();
            tasks.iter()
                .filter(|task| task.status == TaskStatus::Running)
                .map(|task| task.id.clone())
                .collect::<Vec<_>>()
        };

        // Pause each running task
        for id in running_task_ids {
            let _ = self.pause_task(&id, app_handle);
        }

        // Emit queue-paused event
        emit_event(app_handle, "queue-paused", None);

        // Save state
        self.save_state(app_handle)?;

        Ok(())
    }

    /// Resume the queue
    pub fn resume_queue(&self, app_handle: &AppHandle) -> TaskResult<()> {
        // Update is_queue_paused
        *self.is_queue_paused.write() = false;

        // Collect IDs of paused tasks
        let paused_task_ids = {
            let tasks = self.tasks.read();
            tasks.iter()
                .filter(|task| task.status == TaskStatus::Paused)
                .map(|task| task.id.clone())
                .collect::<Vec<_>>()
        };

        // Resume each paused task
        for id in paused_task_ids {
            let _ = self.resume_task(&id, app_handle);
        }

        // Process next tasks
        self.process_next_tasks(app_handle)?;

        // Emit queue-resumed event
        emit_event(app_handle, "queue-resumed", None);

        // Save state
        self.save_state(app_handle)?;

        Ok(())
    }

    /// Cancel the queue
    pub fn cancel_queue(&self, app_handle: &AppHandle) -> TaskResult<()> {
        // Collect IDs of pending, running, and paused tasks
        let tasks_to_cancel = {
            let tasks = self.tasks.read();
            tasks.iter()
                .filter(|task| {
                    task.status == TaskStatus::Pending ||
                    task.status == TaskStatus::Running ||
                    task.status == TaskStatus::Paused
                })
                .map(|task| task.id.clone())
                .collect::<Vec<_>>()
        };

        // Cancel each task
        for id in tasks_to_cancel {
            let _ = self.cancel_task(&id, app_handle);
        }

        // Clear the queue
        {
            let mut queue = self.queue.write();
            queue.clear();
        }

        // Emit queue-canceled event
        emit_event(app_handle, "queue-canceled", None);

        // Save state
        self.save_state(app_handle)?;

        Ok(())
    }

    /// Pause a task
    pub fn pause_task(&self, task_id: &str, app_handle: &AppHandle) -> TaskResult<()> {
        // Get the task
        let task = self.get_task(task_id)?;

        // Check task status
        if task.status != TaskStatus::Running {
            return Err(TaskError::InvalidStatus(format!(
                "Task {} is not in running state", task_id
            )));
        }

        // Update task status
        self.update_task(task_id, |task| {
            task.status = TaskStatus::Paused;
        })?;

        // Save state
        self.save_state(app_handle)?;

        // Emit task-paused event
        emit_event(app_handle, "task-paused", Some(json!({
            "taskId": task_id
        })));

        Ok(())
    }

    /// Update task progress
    pub fn update_task_progress(&self, task_id: &str, progress: f32, app_handle: &AppHandle) -> TaskResult<()> {
        // Update the task
        self.update_task(task_id, |task| {
            task.progress = progress;
        })?;

        // Save state
        self.save_state(app_handle)?;

        Ok(())
    }

    /// Resume a task
    pub fn resume_task(&self, task_id: &str, app_handle: &AppHandle) -> TaskResult<()> {
        // Get the task
        let task = self.get_task(task_id)?;

        // Check task status
        if task.status != TaskStatus::Paused {
            return Err(TaskError::InvalidStatus(format!(
                "Task {} is not in paused state", task_id
            )));
        }

        // Update task status
        self.update_task(task_id, |task| {
            task.status = TaskStatus::Running;
        })?;

        // Save state
        self.save_state(app_handle)?;

        // Emit task-resumed event
        emit_event(app_handle, "task-resumed", Some(json!({
            "taskId": task_id
        })));

        // Wake up the task
        let (lock, cvar) = &*self.pause_condvar;
        let mut paused_tasks = lock.lock().unwrap();
        paused_tasks.remove(task_id);
        cvar.notify_all();

        Ok(())
    }

    /// Cancel a task
    pub fn cancel_task(&self, task_id: &str, app_handle: &AppHandle) -> TaskResult<()> {
        // Get the task
        let task = self.get_task(task_id)?;

        // Check task status
        if task.status != TaskStatus::Running && task.status != TaskStatus::Paused && task.status != TaskStatus::Pending {
            return Err(TaskError::InvalidStatus(format!(
                "Task {} cannot be canceled in its current state", task_id
            )));
        }

        // Update task status
        self.update_task(task_id, |task| {
            task.status = TaskStatus::Canceled;
            task.completed_at = Some(Utc::now().to_rfc3339());
        })?;

        // Save state
        self.save_state(app_handle)?;

        // Emit task-canceled event
        emit_event(app_handle, "task-canceled", Some(json!({
            "taskId": task_id
        })));

        // Wake up the task if it's paused
        let (lock, cvar) = &*self.pause_condvar;
        let mut paused_tasks = lock.lock().unwrap();
        paused_tasks.remove(task_id);
        cvar.notify_all();

        // Remove from queue if present
        {
            let mut queue = self.queue.write();
            let position = queue.iter().position(|id| id == task_id);
            if let Some(pos) = position {
                queue.remove(pos);
            }
        }

        Ok(())
    }

    /// Retry a task
    pub fn retry_task(&self, task_id: &str, app_handle: &AppHandle) -> TaskResult<()> {
        // Get the task
        let task = self.get_task(task_id)?;

        // Check task status
        if task.status != TaskStatus::Failed && task.status != TaskStatus::Canceled {
            return Err(TaskError::InvalidStatus(format!(
                "Task {} cannot be retried in its current state", task_id
            )));
        }

        // Update task status
        self.update_task(task_id, |task| {
            task.status = TaskStatus::Pending;
            task.progress = 0.0;
            task.error = None;
            task.completed_at = None;
        })?;

        // Save state
        self.save_state(app_handle)?;

        // Emit task-retried event
        emit_event(app_handle, "task-retried", Some(json!({
            "taskId": task_id
        })));

        // Add to queue if not already there
        {
            let mut queue = self.queue.write();
            if !queue.iter().any(|id| id == task_id) {
                queue.push_back(task_id.to_string());
            }
        }

        // If queue is not paused and there are available slots, start the task
        if !*self.is_queue_paused.read() {
            let running_count = {
                let tasks = self.tasks.read();
                tasks.iter()
                    .filter(|task| task.status == TaskStatus::Running)
                    .count()
            };

            if running_count < *self.max_concurrent_tasks.read() {
                self.start_task(task_id, app_handle)?;
            }
        }

        Ok(())
    }

    /// Remove a task
    pub fn remove_task(&self, task_id: &str, app_handle: &AppHandle) -> TaskResult<()> {
        // Find task in the list
        let task = self.get_task(task_id)?;

        // Check task status
        if task.status == TaskStatus::Running || task.status == TaskStatus::Paused {
            return Err(TaskError::InvalidStatus(format!(
                "Task {} cannot be removed while running or paused", task_id
            )));
        }

        // Remove from tasks list
        {
            let mut tasks = self.tasks.write();
            tasks.retain(|t| t.id != task_id);
        }

        // Remove from queue if present
        {
            let mut queue = self.queue.write();
            queue.retain(|id| id != task_id);
        }

        // Save state
        self.save_state(app_handle)?;

        // Emit task-removed event
        emit_event(app_handle, "task-removed", Some(json!({
            "taskId": task_id
        })));

        Ok(())
    }

    /// Clear completed tasks
    pub fn clear_completed_tasks(&self, app_handle: &AppHandle) -> TaskResult<()> {
        // Collect IDs of completed and canceled tasks
        let tasks_to_remove = {
            let tasks = self.tasks.read();
            tasks.iter()
                .filter(|task| task.status == TaskStatus::Completed || task.status == TaskStatus::Canceled)
                .map(|task| task.id.clone())
                .collect::<Vec<_>>()
        };

        // Remove each task
        for id in tasks_to_remove {
            let _ = self.remove_task(&id, app_handle);
        }

        Ok(())
    }

    /// Reorder tasks in the queue
    pub fn reorder_tasks(&self, new_order: Vec<String>, app_handle: &AppHandle) -> TaskResult<()> {
        // Validate that all IDs exist
        {
            let tasks = self.tasks.read();
            for id in &new_order {
                if !tasks.iter().any(|task| task.id == *id) {
                    return Err(TaskError::TaskNotFound(id.clone()));
                }
            }
        }

        // Update queue
        {
            let mut queue = self.queue.write();
            queue.clear();
            for id in new_order {
                queue.push_back(id);
            }
        }

        // Save state
        self.save_state(app_handle)?;

        // Emit queue-reordered event
        emit_event(app_handle, "queue-reordered", None);

        Ok(())
    }

    /// Set the maximum number of concurrent tasks
    pub fn set_max_concurrent_tasks(&self, max: usize, app_handle: &AppHandle) -> TaskResult<()> {
        // Update max_concurrent_tasks
        *self.max_concurrent_tasks.write() = max;
        *self.semaphore.write() = Arc::new(Semaphore::new(max));

        // Save state
        self.save_state(app_handle)?;

        // Emit max-concurrent-tasks-changed event
        emit_event(app_handle, "max-concurrent-tasks-changed", Some(json!({
            "max": max
        })));

        Ok(())
    }

    /// Get the maximum number of concurrent tasks
    pub fn get_max_concurrent_tasks(&self) -> usize {
        *self.max_concurrent_tasks.read()
    }

    /// Check if the queue is paused
    pub fn is_queue_paused(&self) -> bool {
        *self.is_queue_paused.read()
    }

    /// Save the task state to a file
    pub fn save_state(&self, app_handle: &AppHandle) -> TaskResult<()> {
        // Get tasks and queue
        let tasks = self.tasks.read();
        let queue = self.queue.read();

        // Save tasks
        store_helper::set_value(app_handle, TASKS_STORE_PATH, "tasks", &*tasks)
            .map_err(|e| TaskError::StoreSaveError(e.to_string()))?;

        // Save queue
        store_helper::set_value(app_handle, TASKS_STORE_PATH, "queue", &*queue)
            .map_err(|e| TaskError::StoreSaveError(e.to_string()))?;

        // Save max_concurrent_tasks
        store_helper::set_value(app_handle, TASKS_STORE_PATH, "max_concurrent_tasks", &*self.max_concurrent_tasks.read())
            .map_err(|e| TaskError::StoreSaveError(e.to_string()))?;

        // Save is_queue_paused
        store_helper::set_value(app_handle, TASKS_STORE_PATH, "is_queue_paused", &*self.is_queue_paused.read())
            .map_err(|e| TaskError::StoreSaveError(e.to_string()))?;

        info!("Task state saved successfully");
        Ok(())
    }

    /// Load the task state from a file
    pub fn load_state(&self, app_handle: &AppHandle) -> TaskResult<()> {
        // Check if the store exists
        let store_exists = store_helper::store_exists(app_handle, TASKS_STORE_PATH)
            .map_err(|e| TaskError::StoreLoadError(e.to_string()))?;

        if !store_exists {
            info!("Task store does not exist, skipping load");
            return Ok(());
        }

        // Load tasks
        let tasks_opt: Option<Vec<Task>> = store_helper::get_value(app_handle, TASKS_STORE_PATH, "tasks")
            .map_err(|e| TaskError::StoreLoadError(e.to_string()))?;

        if let Some(tasks_vec) = tasks_opt {
            let mut tasks = self.tasks.write();
            *tasks = tasks_vec;
            info!("Loaded {} tasks from store", tasks.len());
        }

        // Load queue
        let queue_opt: Option<VecDeque<String>> = store_helper::get_value(app_handle, TASKS_STORE_PATH, "queue")
            .map_err(|e| TaskError::StoreLoadError(e.to_string()))?;

        if let Some(queue_vec) = queue_opt {
            let mut queue = self.queue.write();
            *queue = queue_vec;
            info!("Loaded {} tasks in queue from store", queue.len());
        }

        // Load max_concurrent_tasks
        let max_concurrent_tasks_opt: Option<usize> = store_helper::get_value(app_handle, TASKS_STORE_PATH, "max_concurrent_tasks")
            .map_err(|e| TaskError::StoreLoadError(e.to_string()))?;

        if let Some(max) = max_concurrent_tasks_opt {
            *self.max_concurrent_tasks.write() = max;
            *self.semaphore.write() = Arc::new(Semaphore::new(max));
            info!("Loaded max_concurrent_tasks: {}", max);
        }

        // Load is_queue_paused
        let is_queue_paused_opt: Option<bool> = store_helper::get_value(app_handle, TASKS_STORE_PATH, "is_queue_paused")
            .map_err(|e| TaskError::StoreLoadError(e.to_string()))?;

        if let Some(paused) = is_queue_paused_opt {
            *self.is_queue_paused.write() = paused;
            info!("Loaded is_queue_paused: {}", paused);
        }

        Ok(())
    }
}

/// Update task status
async fn update_task_status(
    app_handle: &AppHandle,
    task_id: &str,
    status: TaskStatus,
    progress: f32,
    error: Option<String>,
) {
    // Get task manager
    let task_manager = app_handle.state::<TaskManager>();

    // We need to update the task status in a way that doesn't require mutable access
    // First, get the current task
    let current_task = {
        let manager = task_manager.inner();
        match manager.get_task(task_id) {
            Ok(task) => task.clone(),
            Err(_) => return, // Task not found
        }
    };

    // Create an updated task
    let mut updated_task = current_task;
    updated_task.status = status;
    updated_task.progress = progress;
    if let Some(err) = error {
        updated_task.error = Some(err);
    }
    if status == TaskStatus::Completed || status == TaskStatus::Failed || status == TaskStatus::Canceled {
        updated_task.completed_at = Some(Utc::now().to_rfc3339());
    }

    // Emit an event to update the task
    emit_event(app_handle, "task-updated", Some(serde_json::json!({
        "task": updated_task
    })));
}

/// Emit event
fn emit_event(app_handle: &AppHandle, event: &str, payload: Option<serde_json::Value>) {
    if let Some(payload) = payload {
        let _ = app_handle.emit(event, payload);
    } else {
        let _ = app_handle.emit(event, ());
    }
}

// TaskProcessor already implements Clone via #[derive(Clone)]

// Implement From<TaskError> for AppError
impl From<TaskError> for AppError {
    fn from(error: TaskError) -> Self {
        match error {
            TaskError::TaskNotFound(id) => AppError::new(
                format!("Task {} not found", id),
                ErrorCode::TaskNotFound,
                Some("Task not found in task manager".to_string()),
            ),
            TaskError::InvalidStatus(msg) => AppError::new(
                format!("Invalid task status: {}", msg),
                ErrorCode::InvalidArgument,
                Some("Cannot change task to the requested state".to_string()),
            ),
            TaskError::ProcessingFailed(msg) => AppError::new(
                format!("Task processing failed: {}", msg),
                ErrorCode::VideoProcessingFailed,
                Some("Error processing video".to_string()),
            ),
            TaskError::UnsupportedTaskType(task_type) => AppError::new(
                format!("Unsupported task type: {}", task_type),
                ErrorCode::InvalidArgument,
                Some("The specified task type is not supported".to_string()),
            ),
            TaskError::StoreSaveError(msg) => AppError::new(
                format!("Failed to save task state: {}", msg),
                ErrorCode::FileWriteError,
                Some("Error saving task state to store".to_string()),
            ),
            TaskError::StoreLoadError(msg) => AppError::new(
                format!("Failed to load task state: {}", msg),
                ErrorCode::FileReadError,
                Some("Error loading task state from store".to_string()),
            ),
            TaskError::Canceled => AppError::new(
                "Task was canceled".to_string(),
                ErrorCode::TaskCanceled,
                Some("The task was canceled by the user".to_string()),
            ),
            TaskError::Other(msg) => AppError::new(
                msg,
                ErrorCode::UnknownError,
                Some("Unexpected error in task manager".to_string()),
            ),
        }
    }
}
