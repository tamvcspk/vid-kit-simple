mod error;
mod task;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use log;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use threadpool::ThreadPool;

use crate::state::conversion_state;
use crate::state::conversion_state::get_task_id_from_string;

// Re-export from FFmpeg
use ffmpeg_next as ffmpeg;
use ffmpeg::format::{input, output};
use ffmpeg::codec::{self, encoder};
use ffmpeg::media::Type as MediaType;
use ffmpeg::software::scaling::{context::Context as ScalingContext, flag::Flags as ScalingFlags};
use ffmpeg::util::frame::video::Video as VideoFrame;
use ffmpeg::util::rational::Rational;

pub use error::{VideoError, VideoResult};
pub use task::{ProcessingTask, TaskStatus, TaskChannels};

/// Video information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub path: String,
    pub format: String,
    pub duration: f64,
    pub width: u32,
    pub height: u32,
    pub bitrate: u64,
    pub codec: String,
    pub framerate: f32,
}

/// Video processing options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingOptions {
    pub output_format: String,
    pub output_path: String,
    pub resolution: Option<(u32, u32)>,
    pub bitrate: Option<u64>,
    pub framerate: Option<f32>,
    pub use_gpu: bool,
    pub gpu_codec: Option<String>,
    pub cpu_codec: Option<String>,
}

/// Process video with threadpool and state management
pub struct VideoProcessor {
    // Manage tasks with HashMap instead of Vec
    tasks: HashMap<String, ProcessingTask>,
    // Pool worker threads, limited by number
    thread_pool: ThreadPool,
    // AppHandle to emit events
    app_handle: Option<AppHandle>,
    // Channel for progress updates
    progress_tx: Option<std::sync::mpsc::Sender<(String, f32)>>,
    // Flag to indicate if progress monitor is running
    progress_monitor_running: bool,
}

impl VideoProcessor {
    /// Create a new VideoProcessor
    pub fn new() -> Self {
        // Initialize FFmpeg if not already initialized
        if let Err(e) = ffmpeg::init() {
            log::error!("Failed to initialize FFmpeg: {}", e);
        }

        // Use the number of CPU cores as the thread pool size
        let num_workers = num_cpus::get();
        log::info!("Creating video processor with {} worker threads", num_workers);

        VideoProcessor {
            tasks: HashMap::new(),
            thread_pool: ThreadPool::new(num_workers),
            app_handle: None,
            progress_tx: None,
            progress_monitor_running: false,
        }
    }

    /// Set AppHandle to emit events
    pub fn set_app_handle(&mut self, app_handle: AppHandle) {
        self.app_handle = Some(app_handle.clone());

        // Start progress monitor if not already running
        if !self.progress_monitor_running {
            self.start_progress_monitor(app_handle);
        }
    }

    /// Start a thread to monitor progress updates
    fn start_progress_monitor(&mut self, app_handle: AppHandle) {
        // Create channel for progress updates
        let (tx, rx) = std::sync::mpsc::channel::<(String, f32)>();
        self.progress_tx = Some(tx);

        // Clone app_handle for use in thread
        let app_handle_clone = app_handle.clone();

        // Start thread to monitor progress
        std::thread::spawn(move || {
            log::info!("Starting progress monitor thread");

            // Continuously receive progress updates
            while let Ok((task_id, progress)) = rx.recv() {
                // Convert task_id to Uuid
                if let Ok(task_uuid) = get_task_id_from_string(&task_id) {
                    // Update progress in state manager
                    if let Err(e) = conversion_state::update_conversion_progress(task_uuid, progress, app_handle_clone.clone()) {
                        log::error!("Failed to update progress: {}", e);
                    }
                } else {
                    log::error!("Invalid task ID format: {}", task_id);
                }
            }

            log::info!("Progress monitor thread stopped");
        });

        self.progress_monitor_running = true;
    }

    /// Get video information
    pub fn get_video_info(&self, file_path: &str) -> VideoResult<VideoInfo> {
        // Convert path to PathBuf
        let path = PathBuf::from(file_path);

        // Open input file
        let input_ctx = input(&path)
            .map_err(|e| VideoError::ffmpeg(format!("Cannot open video file '{}': {}", path.display(), e)))?;

        // Find video stream
        let stream = input_ctx
            .streams()
            .best(MediaType::Video)
            .ok_or_else(|| VideoError::NoVideoStream(path.clone()))?;

        // Create context from parameters
        let codec_ctx = ffmpeg::codec::context::Context::from_parameters(stream.parameters())
            .map_err(|e| VideoError::decoder(format!("Cannot create decoder context for file '{}': {}", path.display(), e)))?;

        // Create decoder
        let decoder = codec_ctx.decoder()
            .video()
            .map_err(|e| VideoError::decoder(format!("Cannot create decoder for file '{}': {}", path.display(), e)))?;

        // Get format information
        let format_name = input_ctx.format().name().to_string();

        // Get duration information
        let duration = if input_ctx.duration() > 0 {
            input_ctx.duration() as f64 / f64::from(ffmpeg::ffi::AV_TIME_BASE)
        } else {
            0.0
        };

        // Get bitrate information
        let bitrate = input_ctx.bit_rate() as u64;

        // Get framerate information
        let framerate = if stream.avg_frame_rate().numerator() != 0 {
            stream.avg_frame_rate().numerator() as f32 / stream.avg_frame_rate().denominator() as f32
        } else {
            0.0
        };

        // Get codec information
        let codec_name = decoder.codec()
            .map(|c| c.name().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        Ok(VideoInfo {
            path: file_path.to_string(),
            format: format_name,
            duration,
            width: decoder.width(),
            height: decoder.height(),
            bitrate,
            codec: codec_name,
            framerate,
        })
    }

    /// Create a new processing task
    pub fn create_task(&mut self, input_file: String, output_file: String, options: ProcessingOptions) -> VideoResult<String> {
        // Convert input_file to PathBuf
        let input_path = PathBuf::from(input_file);

        // Update options with output path
        let mut updated_options = options;
        updated_options.output_path = output_file;

        // Create new task with UUID
        let task = ProcessingTask::new(input_path, updated_options);
        let task_id = task.id.clone();

        // Add to HashMap
        self.tasks.insert(task_id.clone(), task);

        Ok(task_id)
    }

    /// Run task by ID
    pub fn run_task(&mut self, task_id: &str) -> VideoResult<()> {
        // Check if task exists
        if !self.tasks.contains_key(task_id) {
            return Err(VideoError::task_not_found(task_id));
        }

        // Set up channels to monitor progress
        let channels = TaskChannels::new();
        let task_progress_tx = channels.progress_tx.clone();
        let error_tx = channels.error_tx.clone();

        // Get global progress channel
        let global_progress_tx = match &self.progress_tx {
            Some(tx) => tx.clone(),
            None => {
                // If progress monitor not started, start it now
                if let Some(app_handle) = &self.app_handle {
                    self.start_progress_monitor(app_handle.clone());
                    self.progress_tx.as_ref().unwrap().clone()
                } else {
                    return Err(VideoError::other("AppHandle not set"));
                }
            }
        };

        // Update task status to Running
        if let Some(task) = self.tasks.get_mut(task_id) {
            task.mark_running();
        }

        // Clone task and AppHandle for use in thread
        let task = self.tasks.get(task_id).unwrap().clone();
        let app_handle = match &self.app_handle {
            Some(handle) => handle.clone(),
            None => return Err(VideoError::other("AppHandle not set")),
        };

        // Clone task_id for use in thread
        let task_id = task_id.to_string();

        // No need to register task with ConversionStateManager here
        // May have been handled by command on the frontend

        // Create a wrapper for task_progress_tx that forwards updates to global_progress_tx
        let task_id_clone = task_id.to_string();
        let wrapped_progress_tx = move |progress: f32| {
            // Forward progress to global channel
            if let Err(e) = global_progress_tx.send((task_id_clone.clone(), progress)) {
                log::error!("Failed to send progress to global channel: {}", e);
            }

            // Also send to task-specific channel
            if let Err(e) = task_progress_tx.send(progress) {
                log::error!("Failed to send progress to task channel: {}", e);
            }
        };

        // Add task to thread pool
        self.thread_pool.execute(move || {
            // Process video in worker thread
            match process_video_with_callback(&task, wrapped_progress_tx, error_tx.clone()) {
                Ok(_) => {
                    // Success, mark task as completed
                    if let Ok(task_uuid) = get_task_id_from_string(&task_id) {
                        if let Err(e) = conversion_state::mark_task_completed(task_uuid, task.output_file.clone().unwrap_or_default(), app_handle.clone()) {
                            log::error!("Failed to mark task as completed: {}", e);
                        }
                    } else {
                        log::error!("Invalid task ID format: {}", task_id);
                    }
                }
                Err(e) => {
                    // Failure, mark task as failed
                    log::error!("Task {} failed: {}", task_id, e);
                    if let Ok(task_uuid) = get_task_id_from_string(&task_id) {
                        if let Err(se) = conversion_state::mark_task_failed(task_uuid, Some(e.to_string()), app_handle.clone()) {
                            log::error!("Failed to mark task as failed: {}", se);
                        }
                    } else {
                        log::error!("Invalid task ID format: {}", task_id);
                    }
                }
            }
        });

        Ok(())
    }

    /// Get task information
    pub fn get_task(&self, task_id: &str) -> Option<&ProcessingTask> {
        self.tasks.get(task_id)
    }

    /// Get list of all tasks
    pub fn get_tasks(&self) -> Vec<&ProcessingTask> {
        self.tasks.values().collect()
    }

    /// Check progress of all tasks (now only handles cleanup)
    pub fn check_progress(&mut self) {
        // Progress updates are now handled by the progress monitor thread
        // This method is kept for backward compatibility

        // Cleanup completed and failed tasks
        self.cleanup_tasks();
    }

    /// Clean up completed and failed tasks that are older than a certain threshold
    pub fn cleanup_tasks(&mut self) {
        // Get current time
        let now = std::time::SystemTime::now();

        // Define threshold for cleanup (e.g., 1 hour for completed tasks, 24 hours for failed tasks)
        let completed_threshold = std::time::Duration::from_secs(60 * 60); // 1 hour
        let failed_threshold = std::time::Duration::from_secs(24 * 60 * 60); // 24 hours

        // Collect task IDs to remove
        let tasks_to_remove: Vec<String> = self.tasks.iter()
            .filter(|(_, task)| {
                match task.status {
                    // Keep running and pending tasks
                    TaskStatus::Running | TaskStatus::Pending => false,

                    // Remove completed tasks after 1 hour
                    TaskStatus::Completed => {
                        if let Some(completion_time) = &task.completion_time {
                            if let Ok(elapsed) = now.duration_since(*completion_time) {
                                return elapsed > completed_threshold;
                            }
                        }
                        false
                    },

                    // Remove failed and canceled tasks after 24 hours
                    TaskStatus::Failed | TaskStatus::Canceled => {
                        if let Some(completion_time) = &task.completion_time {
                            if let Ok(elapsed) = now.duration_since(*completion_time) {
                                return elapsed > failed_threshold;
                            }
                        }
                        false
                    },
                }
            })
            .map(|(id, _)| id.clone())
            .collect();

        // Remove tasks
        for task_id in tasks_to_remove {
            log::info!("Cleaning up task {}", task_id);
            self.tasks.remove(&task_id);
        }
    }
}

impl Clone for VideoProcessor {
    fn clone(&self) -> Self {
        // Clone tasks
        let tasks = self.tasks.clone();

        // Tạo thread pool mới với cùng kích thước
        let thread_pool = ThreadPool::new(self.thread_pool.max_count());

        // Clone AppHandle nếu có
        let app_handle = self.app_handle.clone();

        VideoProcessor {
            tasks,
            thread_pool,
            app_handle,
            progress_tx: self.progress_tx.clone(),
            progress_monitor_running: self.progress_monitor_running,
        }
    }
}

/// Legacy function for backward compatibility
#[allow(dead_code)]
fn process_video(
    task: &ProcessingTask,
    progress_tx: std::sync::mpsc::Sender<f32>,
    error_tx: std::sync::mpsc::Sender<VideoError>,
) -> VideoResult<()> {
    // Wrap the progress_tx in a callback
    let progress_callback = move |progress: f32| {
        if let Err(e) = progress_tx.send(progress) {
            log::error!("Failed to send progress update: {}", e);
        }
    };

    // Call the new function
    process_video_with_callback(task, progress_callback, error_tx)
}

/// Process video in worker thread with callback for progress updates
fn process_video_with_callback<F>(
    task: &ProcessingTask,
    progress_callback: F,
    _error_tx: std::sync::mpsc::Sender<VideoError>,
) -> VideoResult<()>
where
    F: Fn(f32) + Send + 'static,
{
    // Ensure output directory exists
    if let Some(parent) = Path::new(&task.options.output_path).parent() {
        fs::create_dir_all(parent)?;
    }

    // Open input file
    log::info!("Opening input file: {:?}", task.input_file);
    let mut input_ctx = input(&task.input_file)
        .map_err(|e| VideoError::ffmpeg(format!("Cannot open input file '{}': {}", task.input_file.display(), e)))?;

    // Create output context
    log::info!("Creating output context: {}", task.options.output_path);
    let mut output_ctx = output(&task.options.output_path)
        .map_err(|e| VideoError::ffmpeg(format!("Cannot create output context for '{}': {}", task.options.output_path, e)))?;

    // Find video stream
    let input_stream = input_ctx.streams().best(MediaType::Video)
        .ok_or_else(|| VideoError::NoVideoStream(task.input_file.clone()))?;

    let input_stream_index = input_stream.index();
    let input_time_base = input_stream.time_base();

    // Create decoder
    let decoder_ctx = ffmpeg::codec::context::Context::from_parameters(input_stream.parameters())?;
    let mut decoder = decoder_ctx.decoder().video()?;

    // Choose codec based on options
    let codec_id = choose_codec(&task.options);
    let encoder_codec = encoder::find(codec_id)
        .ok_or_else(|| VideoError::codec("Encoder codec not found"))?;

    // Create output stream
    let mut output_stream = output_ctx.add_stream(encoder_codec)?;

    // Create encoder context
    let encoder_ctx = codec::context::Context::new_with_codec(encoder_codec);
    let mut enc = encoder_ctx.encoder().video()?;

    // Set encoder parameters
    let width = if let Some((w, _)) = task.options.resolution { w } else { decoder.width() };
    let height = if let Some((_, h)) = task.options.resolution { h } else { decoder.height() };

    // Ensure dimensions are divisible by 2 (required by many encoders)
    let width = width - (width % 2);
    let height = height - (height % 2);

    // Set framerate
    let framerate = if let Some(fps) = task.options.framerate {
        Rational::new(fps as i32, 1)
    } else {
        decoder.frame_rate().unwrap_or(Rational::new(30, 1))
    };

    configure_encoder(&mut enc, width, height, framerate, task.options.bitrate)?;

    // Set parameters for output stream
    output_stream.set_parameters(&enc);
    let mut enc = enc.open_as(encoder_codec)?;
    let output_time_base = output_stream.time_base();

    // Create scaler to convert frame format if needed
    let mut scaler = ScalingContext::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        ffmpeg::format::pixel::Pixel::YUV420P,
        width,
        height,
        ScalingFlags::BILINEAR,
    )?;

    // Write header to output file
    output_ctx.write_header()?;

    // Variables needed for processing loop
    let mut decoded_frame = VideoFrame::empty();
    let mut scaled_frame = VideoFrame::empty();
    let mut frame_count = 0;
    let mut total_frames = 0;

    // Estimate total frames
    if input_ctx.duration() > 0 && framerate.numerator() > 0 {
        total_frames = (input_ctx.duration() as f64 / f64::from(ffmpeg::ffi::AV_TIME_BASE) * framerate.numerator() as f64 / framerate.denominator() as f64) as i32;
    }

    // Process each packet
    let exit_loop = false;
    for (stream, packet) in input_ctx.packets() {
        // Only process video stream
        if stream.index() != input_stream_index {
            continue;
        }

        // Send packet to decoder
        if let Err(e) = decoder.send_packet(&packet) {
            log::error!("Error sending packet to decoder: {}", e);
            continue;
        }

        // Receive decoded frame
        while let Ok(_) = decoder.receive_frame(&mut decoded_frame) {
            // Convert frame if needed
            if let Err(e) = scaler.run(&decoded_frame, &mut scaled_frame) {
                log::error!("Error scaling frame: {}", e);
                continue;
            }

            // Set pts for scaled frame
            scaled_frame.set_pts(Some(frame_count as i64));
            scaled_frame.set_kind(decoded_frame.kind());

            // Send frame to encoder
            if let Err(e) = enc.send_frame(&scaled_frame) {
                log::error!("Error sending frame to encoder: {}", e);
                continue;
            }

            // Process encoded packet
            let mut encoded_packet = ffmpeg::packet::Packet::empty();
            while let Ok(_) = enc.receive_packet(&mut encoded_packet) {
                // Set stream index and rescale timestamp
                encoded_packet.set_stream(0);
                encoded_packet.rescale_ts(input_time_base, output_time_base);

                // Write packet to output file
                if let Err(e) = encoded_packet.write_interleaved(&mut output_ctx) {
                    log::error!("Error writing packet: {}", e);
                }
            }

            // Update processed frame count
            frame_count += 1;

            // Update progress
            if total_frames > 0 {
                let progress = (frame_count as f32 / total_frames as f32) * 100.0;
                progress_callback(progress);
            }
        }

        // Check for stop signal
        if exit_loop {
            break;
        }
    }

    // Flush encoder
    if let Err(e) = enc.send_eof() {
        log::error!("Error sending EOF to encoder: {}", e);
    }

    // Process remaining packets
    let mut encoded_packet = ffmpeg::packet::Packet::empty();
    while let Ok(_) = enc.receive_packet(&mut encoded_packet) {
        encoded_packet.set_stream(0);
        encoded_packet.rescale_ts(input_time_base, output_time_base);

        if let Err(e) = encoded_packet.write_interleaved(&mut output_ctx) {
            log::error!("Error writing final packet: {}", e);
        }
    }

    // Write trailer to output file
    output_ctx.write_trailer()?;

    // Report completion progress
    progress_callback(100.0);

    Ok(())
}

/// Choose codec based on options
fn choose_codec(options: &ProcessingOptions) -> codec::Id {
    if options.use_gpu {
        if let Some(gpu_codec) = &options.gpu_codec {
            match gpu_codec.as_str() {
                "nvenc" => return codec::Id::H264,
                "qsv" => return codec::Id::H264,
                "vaapi" => return codec::Id::H264,
                "videotoolbox" => return codec::Id::H264,
                _ => log::warn!("Unknown GPU codec: {}, falling back to software H264", gpu_codec),
            }
        }
    }

    if let Some(cpu_codec) = &options.cpu_codec {
        match cpu_codec.as_str() {
            "h264" => return codec::Id::H264,
            "h265" => return codec::Id::HEVC,
            "vp9" => return codec::Id::VP9,
            "av1" => return codec::Id::AV1,
            _ => log::warn!("Unknown CPU codec: {}, falling back to H264", cpu_codec),
        }
    }

    // Default: H264
    codec::Id::H264
}

/// Configure encoder
fn configure_encoder(
    enc: &mut ffmpeg::encoder::video::Video,
    width: u32,
    height: u32,
    framerate: Rational,
    bitrate: Option<u64>,
) -> VideoResult<()> {
    // Set size and format
    enc.set_width(width);
    enc.set_height(height);
    enc.set_format(ffmpeg::format::pixel::Pixel::YUV420P);

    // Set timebase and framerate
    enc.set_time_base(Rational::new(1, 25));
    enc.set_frame_rate(Some(framerate));

    // Set codec parameters
    enc.set_max_b_frames(2);
    enc.set_gop(25); // Set GOP size (1 second with 25fps)

    // Add parameter to avoid timestamp errors
    enc.set_flags(ffmpeg::codec::flag::Flags::GLOBAL_HEADER);

    // Set bitrate if available
    if let Some(br) = bitrate {
        // Convert from u64 to usize
        if let Ok(br_usize) = br.try_into() {
            enc.set_bit_rate(br_usize);
        }
    }

    Ok(())
}


