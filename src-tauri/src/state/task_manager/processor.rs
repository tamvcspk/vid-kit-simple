use std::collections::HashMap;
use std::time::Duration;
use tauri::{AppHandle, Manager, Emitter};

use crate::services::video_processor::{VideoProcessor, ProcessingOptions};
use super::errors::TaskError;
use super::{Task, TaskStatus};

/// Emit event
fn emit_event(app_handle: &AppHandle, event: &str, payload: Option<serde_json::Value>) {
    if let Some(payload) = payload {
        let _ = app_handle.emit(event, payload);
    } else {
        let _ = app_handle.emit(event, ());
    }
}

/// Task processor that acts as a proxy between tasks and video processor
#[derive(Clone)]
pub struct TaskProcessor {
    video_processor: VideoProcessor,
}

impl TaskProcessor {
    /// Create a new TaskProcessor
    pub fn new() -> Self {
        Self {
            video_processor: VideoProcessor::new(),
        }
    }

    /// Process a task based on its type
    pub async fn process_task(&self, task: &Task, app_handle: &AppHandle) -> Result<(), TaskError> {
        // Create progress callback
        let app_handle_clone = app_handle.clone();
        let task_id_clone = task.id.clone();
        let progress_callback = Box::new(move |progress: f32| -> bool {
            // Update task progress
            let _ = emit_event(&app_handle_clone, "task-progress", Some(serde_json::json!({
                "task_id": task_id_clone,
                "progress": progress
            })));

            // Check if task is paused or canceled
            let task_manager = app_handle_clone.state::<super::TaskManager>();
            let task_status = {
                let manager = task_manager.inner();
                match manager.get_task(&task_id_clone) {
                    Ok(task) => task.status,
                    Err(_) => return false, // Task not found, stop processing
                }
            };

            if task_status == TaskStatus::Canceled {
                return false; // Stop processing
            }

            if task_status == TaskStatus::Paused {
                // In a real implementation, we would wait for the task to be resumed
                // For now, we'll just return false to stop processing
                return false;
            }

            true // Continue processing
        });

        // Get task information
        let input_path = &task.input_path;
        let output_path = &task.output_path;
        let config = &task.config;

        // Create processing options from config
        let options = create_processing_options(config)?;

        // Process task based on type
        match task.task_type.as_str() {
            "convert" => {
                // Call convert_video from VideoProcessor
                self.video_processor.convert_video(
                    input_path,
                    output_path,
                    options,
                    progress_callback,
                ).map_err(|e| TaskError::ProcessingFailed(e.to_string()))?;
            },
            "split" => {
                // Get start and end time from config
                let start_time = config.get("start_time")
                    .and_then(|s| s.parse::<f64>().ok())
                    .unwrap_or(0.0);
                let end_time = config.get("end_time")
                    .and_then(|s| s.parse::<f64>().ok())
                    .unwrap_or(0.0);

                // Call split_video from VideoProcessor
                self.video_processor.split_video(
                    input_path,
                    output_path,
                    start_time,
                    end_time,
                    options,
                    progress_callback,
                ).map_err(|e| TaskError::ProcessingFailed(e.to_string()))?;
            },
            "edit" => {
                // Create edit operations from config
                let edit_operations = config.clone();

                // Call edit_video from VideoProcessor
                self.video_processor.edit_video(
                    input_path,
                    output_path,
                    edit_operations,
                    options,
                    progress_callback,
                ).map_err(|e| TaskError::ProcessingFailed(e.to_string()))?;
            },
            "sanitize" => {
                // Create sanitize options from config
                let sanitize_options = config.clone();

                // Call sanitize_video from VideoProcessor
                self.video_processor.sanitize_video(
                    input_path,
                    output_path,
                    sanitize_options,
                    options,
                    progress_callback,
                ).map_err(|e| TaskError::ProcessingFailed(e.to_string()))?;
            },
            _ => {
                return Err(TaskError::UnsupportedTaskType(task.task_type.clone()));
            }
        }

        Ok(())
    }
}

/// Create ProcessingOptions from config
fn create_processing_options(config: &HashMap<String, String>) -> Result<ProcessingOptions, TaskError> {
    let mut options = ProcessingOptions {
        output_format: config.get("output_format").cloned().unwrap_or_else(|| "mp4".to_string()),
        output_path: config.get("output_path").cloned().unwrap_or_default(),
        resolution: None,
        bitrate: None,
        framerate: None,
        use_gpu: config.get("use_gpu").map_or(false, |v| v == "true"),
        gpu_codec: config.get("gpu_codec").cloned(),
        cpu_codec: config.get("cpu_codec").cloned(),

        // Time options
        start_time: None,
        end_time: None,

        // Edit options
        crop: None,
        rotate: None,
        flip: None,
        flop: None,

        // Sanitize options
        remove_metadata: None,
        blur_regions: None,
        audio_volume: None,
        denoise: None,
    };

    // Parse resolution if provided
    if let (Some(width), Some(height)) = (config.get("width"), config.get("height")) {
        if let (Ok(w), Ok(h)) = (width.parse::<u32>(), height.parse::<u32>()) {
            options.resolution = Some((w, h));
        }
    }

    // Parse bitrate if provided
    if let Some(bitrate) = config.get("bitrate") {
        if let Ok(b) = bitrate.parse::<u64>() {
            options.bitrate = Some(b);
        }
    }

    // Parse framerate if provided
    if let Some(framerate) = config.get("framerate") {
        if let Ok(f) = framerate.parse::<f32>() {
            options.framerate = Some(f);
        }
    }

    // Parse time options
    if let Some(start_time) = config.get("start_time") {
        if let Ok(t) = start_time.parse::<f64>() {
            options.start_time = Some(t);
        }
    }

    if let Some(end_time) = config.get("end_time") {
        if let Ok(t) = end_time.parse::<f64>() {
            options.end_time = Some(t);
        }
    }

    // Parse edit options
    if let Some(crop) = config.get("crop") {
        // Format: "x,y,width,height"
        let parts: Vec<&str> = crop.split(',').collect();
        if parts.len() == 4 {
            if let (Ok(x), Ok(y), Ok(w), Ok(h)) = (
                parts[0].trim().parse::<u32>(),
                parts[1].trim().parse::<u32>(),
                parts[2].trim().parse::<u32>(),
                parts[3].trim().parse::<u32>(),
            ) {
                options.crop = Some((x, y, w, h));
            }
        }
    }

    if let Some(rotate) = config.get("rotate") {
        if let Ok(r) = rotate.parse::<i32>() {
            // Only allow 90, 180, 270 degrees
            if r == 90 || r == 180 || r == 270 {
                options.rotate = Some(r);
            }
        }
    }

    options.flip = config.get("flip").map(|v| v == "true");
    options.flop = config.get("flop").map(|v| v == "true");

    // Parse sanitize options
    options.remove_metadata = config.get("remove_metadata").map(|v| v == "true");
    options.denoise = config.get("denoise").map(|v| v == "true");

    if let Some(volume) = config.get("audio_volume") {
        if let Ok(v) = volume.parse::<f32>() {
            options.audio_volume = Some(v);
        }
    }

    // Parse blur regions
    if let Some(blur_regions) = config.get("blur_regions") {
        // Format: "x1,y1,w1,h1;x2,y2,w2,h2;..."
        let regions: Vec<(u32, u32, u32, u32)> = blur_regions
            .split(';')
            .filter_map(|region| {
                let parts: Vec<&str> = region.split(',').collect();
                if parts.len() == 4 {
                    if let (Ok(x), Ok(y), Ok(w), Ok(h)) = (
                        parts[0].trim().parse::<u32>(),
                        parts[1].trim().parse::<u32>(),
                        parts[2].trim().parse::<u32>(),
                        parts[3].trim().parse::<u32>(),
                    ) {
                        return Some((x, y, w, h));
                    }
                }
                None
            })
            .collect();

        if !regions.is_empty() {
            options.blur_regions = Some(regions);
        }
    }

    Ok(options)
}

/// Update task progress
async fn update_task_progress(app_handle: &AppHandle, task_id: &str, progress: f32) -> Result<(), TaskError> {
    // Just emit the progress event, we don't need to update the task here
    // The task will be updated by the task manager when it receives the event

    // Emit progress event
    emit_event(app_handle, "task-progress", Some(serde_json::json!({
        "taskId": task_id,
        "progress": progress
    })));

    Ok(())
}

/// Wait for task to be resumed
async fn wait_for_resume(task_id: &str, app_handle: &AppHandle) -> Result<(), TaskError> {
    // Get task manager
    let task_manager = app_handle.state::<super::TaskManager>();

    // Wait for task to be resumed
    loop {
        // Check task status
        let task_status = {
            let manager = task_manager.inner();
            let task = manager.get_task(task_id)?;
            task.status
        };

        if task_status == TaskStatus::Running {
            break;
        }

        if task_status == TaskStatus::Canceled {
            return Err(TaskError::Canceled);
        }

        // Sleep for a short time
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}

