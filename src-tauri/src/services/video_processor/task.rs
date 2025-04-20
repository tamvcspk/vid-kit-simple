use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use uuid::Uuid;

use super::error::VideoError;
use super::ProcessingOptions;

/// Video processing status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Canceled,
}

/// Video processing task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingTask {
    pub id: String,
    pub input_file: PathBuf,
    pub options: ProcessingOptions,
    pub progress: f32,
    pub status: TaskStatus,
    pub error_message: Option<String>,
    pub output_file: Option<PathBuf>,
    pub completion_time: Option<std::time::SystemTime>,
}

/// Communication channels with worker thread
pub struct TaskChannels {
    pub progress_tx: Sender<f32>,
    pub progress_rx: Receiver<f32>,
    pub error_tx: Sender<VideoError>,
    pub error_rx: Receiver<VideoError>,
}

impl TaskChannels {
    /// Create a new set of channels
    pub fn new() -> Self {
        let (progress_tx, progress_rx) = channel();
        let (error_tx, error_rx) = channel();

        Self {
            progress_tx,
            progress_rx,
            error_tx,
            error_rx,
        }
    }
}

impl ProcessingTask {
    /// Create a new task with a unique ID
    pub fn new(input_file: PathBuf, options: ProcessingOptions) -> Self {
        // Save output path before options is moved
        let output_path = PathBuf::from(&options.output_path);

        Self {
            id: Uuid::new_v4().to_string(),
            input_file,
            options,
            progress: 0.0,
            status: TaskStatus::Pending,
            error_message: None,
            output_file: Some(output_path),
            completion_time: None,
        }
    }

    /// Update progress
    pub fn update_progress(&mut self, progress: f32) {
        self.progress = progress;
        if progress >= 100.0 {
            self.status = TaskStatus::Completed;
            self.completion_time = Some(std::time::SystemTime::now());
        }
    }

    /// Mark task as running
    pub fn mark_running(&mut self) {
        self.status = TaskStatus::Running;
    }

    /// Mark task as completed
    pub fn mark_completed(&mut self) {
        self.status = TaskStatus::Completed;
        self.progress = 100.0;
        self.completion_time = Some(std::time::SystemTime::now());
    }

    /// Mark task as failed
    pub fn mark_failed(&mut self, error: Option<String>) {
        self.status = TaskStatus::Failed;
        self.error_message = error;
        self.completion_time = Some(std::time::SystemTime::now());
    }

    /// Mark task as canceled
    pub fn mark_canceled(&mut self) {
        self.status = TaskStatus::Canceled;
        self.completion_time = Some(std::time::SystemTime::now());
    }
}
