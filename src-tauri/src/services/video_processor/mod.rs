use std::fs;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use serde::{Deserialize, Serialize};
use ffmpeg_next as ffmpeg;
use ffmpeg::format::{input, output};
use ffmpeg::codec::{self, encoder};
use ffmpeg::media::Type as MediaType;
use ffmpeg::software::scaling::{context::Context as ScalingContext, flag::Flags as ScalingFlags};
use ffmpeg::util::frame::video::Video as VideoFrame;
use ffmpeg::util::rational::Rational;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessingStatus {
    Pending,
    Running(f32), // Progress (0.0 - 100.0)
    Complete,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingTask {
    pub id: String,
    pub input_file: String,
    pub options: ProcessingOptions,
    pub status: ProcessingStatus,
}

pub struct VideoProcessor {
    tasks: Vec<ProcessingTask>,
    progress_channel: Option<(Sender<(String, f32)>, Receiver<(String, f32)>)>,
}

impl VideoProcessor {
    pub fn new() -> Self {
        // Initialize FFmpeg if not already initialized
        if let Err(e) = ffmpeg::init() {
            eprintln!("Failed to initialize FFmpeg: {}", e);
        }

        VideoProcessor {
            tasks: Vec::new(),
            progress_channel: None,
        }
    }

    /// Get video file information
    pub fn get_video_info(&self, file_path: &str) -> Result<VideoInfo, String> {
        // Normalize path to ensure it works on all operating systems
        let normalized_path = file_path.replace("\\", "/");

        let input_ctx = input(&normalized_path)
            .map_err(|e| format!("Cannot open video file: {}", e))?;

        let stream = input_ctx
            .streams()
            .best(MediaType::Video)
            .ok_or_else(|| "Video stream not found".to_string())?;

        let codec_ctx = ffmpeg::codec::context::Context::from_parameters(stream.parameters())
            .map_err(|e| format!("Cannot create context from parameters: {}", e))?;

        let decoder = codec_ctx.decoder()
            .video()
            .map_err(|e| format!("Cannot create decoder: {}", e))?;

        // Get format information
        let format_name = input_ctx.format().name().to_string();

        // Get duration information
        let duration = if input_ctx.duration() > 0 {
            input_ctx.duration() as f64 / f64::from(ffmpeg::ffi::AV_TIME_BASE)
        } else {
            0.0
        };

        // Get bitrate information
        let bitrate = if input_ctx.bit_rate() > 0 {
            input_ctx.bit_rate() as u64
        } else {
            0
        };

        // Get framerate information
        let framerate = if stream.avg_frame_rate().numerator() != 0 {
            stream.avg_frame_rate().numerator() as f32 / stream.avg_frame_rate().denominator() as f32
        } else {
            0.0
        };

        // Get codec information
        let codec_name = match decoder.codec() {
            Some(codec) => codec.name().to_string(),
            None => "unknown".to_string(),
        };

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
    pub fn create_task(&mut self, input_file: &str, options: ProcessingOptions) -> String {
        let task_id = format!("task_{}", self.tasks.len());

        let task = ProcessingTask {
            id: task_id.clone(),
            input_file: input_file.to_string(),
            options,
            status: ProcessingStatus::Pending,
        };

        self.tasks.push(task);
        task_id
    }

    /// Run processing task by ID
    pub fn run_task(&mut self, task_id: &str) -> Result<(), String> {
        let task_index = self
            .tasks
            .iter()
            .position(|t| t.id == task_id)
            .ok_or_else(|| format!("Task with ID {} not found", task_id))?;

        // Update status
        self.tasks[task_index].status = ProcessingStatus::Running(0.0);

        // Create channel for progress updates
        let (tx, rx) = channel();
        self.progress_channel = Some((tx.clone(), rx));

        // Clone task to use in thread
        let task = self.tasks[task_index].clone();
        let task_id_clone = task_id.to_string();

        // Spawn a thread to process video
        thread::spawn(move || {
            // Create output directory if it doesn't exist
            if let Some(parent) = Path::new(&task.options.output_path).parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    let _ = tx.send((task_id_clone.clone(), -1.0)); // Signal error
                    eprintln!("Cannot create output directory: {}", e);
                    return;
                }
            }

            // Normalize input path
            let normalized_input_path = task.input_file.replace("\\", "/");
            eprintln!("Input path: {}", normalized_input_path);

            // Open input file
            let mut input_ctx = match input(&normalized_input_path) {
                Ok(ctx) => ctx,
                Err(e) => {
                    let _ = tx.send((task_id_clone.clone(), -1.0)); // Signal error
                    eprintln!("Cannot open input file: {}", e);
                    return;
                }
            };

            // Normalize output path
            let normalized_output_path = task.options.output_path.replace("\\", "/");
            eprintln!("Output path: {}", normalized_output_path);

            // Create output context
            let mut output_ctx = match output(&normalized_output_path) {
                Ok(ctx) => ctx,
                Err(e) => {
                    let _ = tx.send((task_id_clone.clone(), -1.0)); // Signal error
                    eprintln!("Cannot create output context: {}", e);
                    return;
                }
            };

            // Find video stream
            let input_stream = match input_ctx.streams().best(MediaType::Video) {
                Some(stream) => stream,
                None => {
                    let _ = tx.send((task_id_clone, -1.0)); // Signal error
                    eprintln!("Video stream not found");
                    return;
                }
            };

            let input_stream_index = input_stream.index();
            let input_time_base = input_stream.time_base();

            // Create decoder
            let decoder_ctx = match ffmpeg::codec::context::Context::from_parameters(input_stream.parameters()) {
                Ok(ctx) => ctx,
                Err(e) => {
                    let _ = tx.send((task_id_clone, -1.0)); // Signal error
                    eprintln!("Cannot create decoder context: {}", e);
                    return;
                }
            };

            let mut decoder = match decoder_ctx.decoder().video() {
                Ok(dec) => dec,
                Err(e) => {
                    let _ = tx.send((task_id_clone, -1.0)); // Signal error
                    eprintln!("Cannot create video decoder: {}", e);
                    return;
                }
            };

            // Use H264 codec instead of MPEG4 to avoid B-frames issues
            let codec_id = codec::Id::H264; // H264 has more options and is widely supported

            let encoder_codec = match encoder::find(codec_id) {
                Some(codec) => codec,
                None => {
                    let _ = tx.send((task_id_clone, -1.0)); // Signal error
                    eprintln!("Encoder codec not found");
                    return;
                }
            };

            // Create output stream
            let mut output_stream = match output_ctx.add_stream(encoder_codec) {
                Ok(stream) => stream,
                Err(e) => {
                    let _ = tx.send((task_id_clone, -1.0)); // Signal error
                    eprintln!("Cannot add output stream: {}", e);
                    return;
                }
            };

            // Create encoder context
            let encoder_ctx = codec::context::Context::new_with_codec(encoder_codec);

            // Create encoder
            let mut enc = match encoder_ctx.encoder().video() {
                Ok(enc) => enc,
                Err(e) => {
                    let _ = tx.send((task_id_clone.clone(), -1.0)); // Signal error
                    eprintln!("Cannot create video encoder: {}", e);
                    return;
                }
            };

            // Set encoder parameters
            let width = if let Some((w, _)) = task.options.resolution { w } else { decoder.width() };
            let height = if let Some((_, h)) = task.options.resolution { h } else { decoder.height() };
            let framerate = if let Some(fps) = task.options.framerate {
                Rational::new(fps as i32, 1)
            } else {
                decoder.frame_rate().unwrap_or(Rational::new(30, 1)) // Default to 30fps if not available
            };

            // Ensure dimensions are even (required by some encoders)
            let width = width - (width % 2);
            let height = height - (height % 2);

            enc.set_width(width);
            enc.set_height(height);
            // Use pixel format compatible with encoder
            enc.set_format(ffmpeg::format::pixel::Pixel::YUV420P); // Most common pixel format
            enc.set_time_base(Rational::new(1, 25));
            enc.set_frame_rate(Some(Rational::new(25, 1))); // Use stable framerate of 25fps

            // Set codec parameters
            enc.set_max_b_frames(0); // Don't use B-frames to avoid errors
            enc.set_gop(25); // Set GOP size (1 second with 25fps)

            // Add parameters to avoid timestamp errors
            enc.set_flags(ffmpeg::codec::flag::Flags::GLOBAL_HEADER);
            output_stream.set_time_base(Rational::new(1, 25000)); // More detailed timebase

            if let Some(bitrate) = task.options.bitrate {
                // Convert bitrate from u64 to usize
                let bitrate_usize = bitrate.try_into().unwrap_or(0);
                enc.set_bit_rate(bitrate_usize);
            }

            // Set parameters for output stream
            output_stream.set_parameters(&enc);

            // Open encoder
            let mut enc = match enc.open_as(encoder_codec) {
                Ok(enc) => enc,
                Err(e) => {
                    let _ = tx.send((task_id_clone.clone(), -1.0)); // Signal error
                    eprintln!("Cannot open encoder: {}", e);
                    return;
                }
            };

            // Set time base for output stream
            let output_time_base = output_stream.time_base();

            // Create scaler to convert frame format if needed
            let mut scaler = match ScalingContext::get(
                decoder.format(),
                decoder.width(),
                decoder.height(),
                ffmpeg::format::pixel::Pixel::YUV420P, // Ensure output pixel format is YUV420P
                width,
                height,
                ScalingFlags::BILINEAR,
            ) {
                Ok(s) => s,
                Err(e) => {
                    let _ = tx.send((task_id_clone.clone(), -1.0)); // Signal error
                    eprintln!("Cannot create scaler: {}", e);
                    return;
                }
            };

            // Write header for output file
            if let Err(e) = output_ctx.write_header() {
                let _ = tx.send((task_id_clone.clone(), -1.0)); // Signal error
                eprintln!("Cannot write header: {}", e);
                return;
            }

            let mut decoded_frame = VideoFrame::empty();
            let mut scaled_frame = VideoFrame::empty();
            let mut frame_count = 0;
            let mut total_frames = 0;

            // Estimate total frames
            if input_ctx.duration() > 0 && framerate.numerator() > 0 {
                total_frames = (input_ctx.duration() as f64 / f64::from(ffmpeg::ffi::AV_TIME_BASE) * framerate.numerator() as f64 / framerate.denominator() as f64) as i32;
            }

            // Process each packet
            for (stream, packet) in input_ctx.packets() {
                if stream.index() == input_stream_index {
                    // Send packet to decoder
                    if let Err(e) = decoder.send_packet(&packet) {
                        eprintln!("Error sending packet to decoder: {}", e);
                        continue;
                    }

                    // Receive decoded frames
                    while decoder.receive_frame(&mut decoded_frame).is_ok() {
                        frame_count += 1;

                        // Scale frame if needed
                        if let Err(e) = scaler.run(&decoded_frame, &mut scaled_frame) {
                            eprintln!("Error scaling frame: {}", e);
                            continue;
                        }

                        // Set timestamp for frame with incremental value
                        // Use frame_count * 1000 to ensure no duplicate timestamps
                        scaled_frame.set_pts(Some(frame_count * 1000));
                        scaled_frame.set_kind(decoded_frame.kind());

                        // Send frame to encoder
                        if let Err(e) = enc.send_frame(&scaled_frame) {
                            eprintln!("Error sending frame to encoder: {}", e);
                            continue;
                        }

                        // Receive encoded packets
                        let mut encoded_packet = ffmpeg::Packet::empty();
                        while enc.receive_packet(&mut encoded_packet).is_ok() {
                            // Set stream index and rescale timestamp
                            encoded_packet.set_stream(0);
                            encoded_packet.rescale_ts(input_time_base, output_time_base);

                            // Write packet to output file
                            if let Err(e) = encoded_packet.write_interleaved(&mut output_ctx) {
                                eprintln!("Error writing packet: {}", e);
                            }
                        }

                        // Update progress
                        if total_frames > 0 {
                            let progress = (frame_count as f32 / total_frames as f32) * 100.0;

                            // Send progress through channel
                            let _ = tx.send((task_id_clone.clone(), progress));
                        }
                    }
                }
            }

            // Flush decoder
            if let Err(e) = decoder.send_eof() {
                eprintln!("Error sending EOF to decoder: {}", e);
            }

            // Receive remaining frames from decoder
            while decoder.receive_frame(&mut decoded_frame).is_ok() {
                // Scale frame if needed
                if let Err(e) = scaler.run(&decoded_frame, &mut scaled_frame) {
                    eprintln!("Error scaling frame: {}", e);
                    continue;
                }

                // Set timestamp for frame with incremental value
                frame_count += 1; // Increase frame_count to avoid duplicate timestamps
                scaled_frame.set_pts(Some(frame_count * 1000));
                scaled_frame.set_kind(decoded_frame.kind());

                // Send frame to encoder
                if let Err(e) = enc.send_frame(&scaled_frame) {
                    eprintln!("Error sending frame to encoder: {}", e);
                    continue;
                }

                // Receive encoded packets
                let mut encoded_packet = ffmpeg::Packet::empty();
                while enc.receive_packet(&mut encoded_packet).is_ok() {
                    // Set stream index and rescale timestamp
                    encoded_packet.set_stream(0);
                    encoded_packet.rescale_ts(input_time_base, output_time_base);

                    // Write packet to output file
                    if let Err(e) = encoded_packet.write_interleaved(&mut output_ctx) {
                        eprintln!("Error writing packet: {}", e);
                    }
                }
            }

            // Flush encoder
            if let Err(e) = enc.send_eof() {
                eprintln!("Error sending EOF to encoder: {}", e);
            }

            // Receive remaining packets from encoder
            let mut encoded_packet = ffmpeg::Packet::empty();
            while enc.receive_packet(&mut encoded_packet).is_ok() {
                // Set stream index and rescale timestamp
                encoded_packet.set_stream(0);
                encoded_packet.rescale_ts(input_time_base, output_time_base);

                // Write packet to output file
                if let Err(e) = encoded_packet.write_interleaved(&mut output_ctx) {
                    eprintln!("Error writing packet: {}", e);
                }
            }

            // Write trailer for output file
            if let Err(e) = output_ctx.write_trailer() {
                let _ = tx.send((task_id_clone.clone(), -1.0)); // Signal error
                eprintln!("Cannot write trailer: {}", e);
                return;
            }

            // Signal completion
            let _ = tx.send((task_id_clone.clone(), 100.0));
        });

        Ok(())
    }

    /// Check progress of tasks
    pub fn check_progress(&mut self) -> Vec<(String, ProcessingStatus)> {
        if let Some((_, ref rx)) = self.progress_channel {
            // Receive all available progress updates
            while let Ok((task_id, progress)) = rx.try_recv() {
                if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
                    if progress < 0.0 {
                        task.status =
                            ProcessingStatus::Failed("Error during processing".to_string());
                    } else if progress >= 100.0 {
                        task.status = ProcessingStatus::Complete;
                    } else {
                        task.status = ProcessingStatus::Running(progress);
                    }
                }
            }
        }

        // Return the current status of all tasks
        self.tasks
            .iter()
            .map(|task| (task.id.clone(), task.status.clone()))
            .collect()
    }

    /// Get information about a task
    pub fn get_task(&self, task_id: &str) -> Option<&ProcessingTask> {
        self.tasks.iter().find(|t| t.id == task_id)
    }

    /// Get list of all tasks
    pub fn get_tasks(&self) -> &[ProcessingTask] {
        &self.tasks
    }
}
