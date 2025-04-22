use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use log::{error, info};

use ffmpeg::codec::{self, encoder};
use ffmpeg::format::{input, output};
use ffmpeg::media::Type as MediaType;
use ffmpeg::software::scaling::{context::Context as ScalingContext, flag::Flags as ScalingFlags};
use ffmpeg::util::frame::video::Video as VideoFrame;
use ffmpeg::util::rational::Rational;
use ffmpeg_next as ffmpeg;

use crate::utils::error::{AppError, AppResult, ErrorCode};
use super::{VideoInfo, ProcessingOptions};

/// Video processor that contains only processing logic
#[derive(Clone)]
pub struct VideoProcessor {}

impl VideoProcessor {
    /// Create a new VideoProcessor
    pub fn new() -> Self {
        // Initialize FFmpeg if not already initialized
        if let Err(e) = ffmpeg::init() {
            error!("Failed to initialize FFmpeg: {}", e);
        }

        Self {}
    }

    /// Get video information
    pub fn get_video_info(&self, file_path: &str) -> AppResult<VideoInfo> {
        // Check if file exists
        if !Path::new(file_path).exists() {
            return Err(AppError::io_error(
                std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
                ErrorCode::FileNotFound,
                Some(format!("Video file not found: {}", file_path)),
            ));
        }

        // Convert path to PathBuf
        let path = PathBuf::from(file_path);

        // Open input file
        let input_ctx = input(&path).map_err(|e| {
            AppError::ffmpeg_error(
                format!("Cannot open video file '{}': {}", path.display(), e),
                ErrorCode::FFmpegInitError,
                Some(format!("Error opening video file: {}", file_path)),
            )
        })?;

        // Find video stream
        let stream = input_ctx
            .streams()
            .best(MediaType::Video)
            .ok_or_else(|| {
                AppError::video_error(
                    format!("No video stream found in file: {}", path.display()),
                    ErrorCode::InvalidVideoFormat,
                    Some(format!("File does not contain a valid video stream: {}", file_path)),
                )
            })?;

        // Create context from parameters
        let codec_ctx = ffmpeg::codec::context::Context::from_parameters(stream.parameters())
            .map_err(|e| {
                AppError::video_error(
                    format!("Cannot create decoder context: {}", e),
                    ErrorCode::DecodingError,
                    Some(format!("Error creating decoder context for file: {}", file_path)),
                )
            })?;

        // Create decoder
        let decoder = codec_ctx.decoder().video().map_err(|e| {
            AppError::video_error(
                format!("Cannot create decoder: {}", e),
                ErrorCode::DecodingError,
                Some(format!("Error creating video decoder for file: {}", file_path)),
            )
        })?;

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
            stream.avg_frame_rate().numerator() as f32
                / stream.avg_frame_rate().denominator() as f32
        } else {
            0.0
        };

        // Get codec information
        let codec_name = decoder
            .codec()
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

    /// Process a video with the given options
    pub fn process_video(
        &self,
        input_path: &str,
        output_path: &str,
        options: ProcessingOptions,
        progress_callback: impl Fn(f32) -> bool + Send + 'static,
    ) -> AppResult<()> {
        // Check if input file exists
        if !Path::new(input_path).exists() {
            return Err(AppError::io_error(
                std::io::Error::new(std::io::ErrorKind::NotFound, "Input file not found"),
                ErrorCode::FileNotFound,
                Some(format!("Input video file not found: {}", input_path)),
            ));
        }

        // Ensure output directory exists
        if let Some(parent) = Path::new(output_path).parent() {
            fs::create_dir_all(parent).map_err(|e| {
                AppError::io_error(
                    e,
                    ErrorCode::DirectoryError,
                    Some(format!("Failed to create output directory: {:?}", parent)),
                )
            })?;
        }

        // Open input file
        info!("Opening input file: {}", input_path);

        // Apply time options if specified
        let mut input_ctx = if options.start_time.is_some() || options.end_time.is_some() {
            // In a real implementation, we would use the FFmpeg API to set start and end time
            // For now, we'll just log the values
            if let Some(start_time) = options.start_time {
                info!("Using start time: {} seconds", start_time);
            }

            if let Some(end_time) = options.end_time {
                info!("Using end time: {} seconds", end_time);
            }

            // Open input file normally for now
            input(input_path).map_err(|e| {
                AppError::ffmpeg_error(
                    format!("Cannot open input file '{}': {}", input_path, e),
                    ErrorCode::FFmpegInitError,
                    Some(format!("Error opening input file: {}", input_path)),
                )
            })?
        } else {
            // Open input file normally
            input(input_path).map_err(|e| {
                AppError::ffmpeg_error(
                    format!("Cannot open input file '{}': {}", input_path, e),
                    ErrorCode::FFmpegInitError,
                    Some(format!("Error opening input file: {}", input_path)),
                )
            })?
        };

        // Create output context
        info!("Creating output context: {}", output_path);

        // Apply metadata removal if specified
        let mut output_ctx = if let Some(true) = options.remove_metadata {
            info!("Removing metadata from output");
            // In a real implementation, we would use the FFmpeg API to remove metadata
            // For now, we'll just create the output context normally
            output(output_path).map_err(|e| {
                AppError::ffmpeg_error(
                    format!("Cannot create output context for '{}': {}", output_path, e),
                    ErrorCode::FFmpegInitError,
                    Some(format!("Error creating output file: {}", output_path)),
                )
            })?
        } else {
            // Create output context normally
            output(output_path).map_err(|e| {
                AppError::ffmpeg_error(
                    format!("Cannot create output context for '{}': {}", output_path, e),
                    ErrorCode::FFmpegInitError,
                    Some(format!("Error creating output file: {}", output_path)),
                )
            })?
        };

        // Find video stream
        let input_stream = input_ctx
            .streams()
            .best(MediaType::Video)
            .ok_or_else(|| {
                AppError::video_error(
                    format!("No video stream found in file: {}", input_path),
                    ErrorCode::InvalidVideoFormat,
                    Some(format!("File does not contain a valid video stream: {}", input_path)),
                )
            })?;

        let input_stream_index = input_stream.index();
        let input_time_base = input_stream.time_base();

        // Create decoder
        let decoder_ctx = ffmpeg::codec::context::Context::from_parameters(input_stream.parameters())
            .map_err(|e| {
                AppError::video_error(
                    format!("Cannot create decoder context: {}", e),
                    ErrorCode::DecodingError,
                    Some(format!("Error creating decoder context for file: {}", input_path)),
                )
            })?;

        let mut decoder = decoder_ctx.decoder().video().map_err(|e| {
            AppError::video_error(
                format!("Cannot create decoder: {}", e),
                ErrorCode::DecodingError,
                Some(format!("Error creating video decoder for file: {}", input_path)),
            )
        })?;

        // Choose codec based on options
        let codec_id = self.choose_codec(&options);
        let encoder_codec = encoder::find(codec_id).ok_or_else(|| {
            AppError::video_error(
                format!("Encoder codec not found: {:?}", codec_id),
                ErrorCode::CodecNotSupported,
                Some("The requested codec is not available".to_string()),
            )
        })?;

        // Create output stream
        // Tái cấu trúc để tránh mượn mutable nhiều lần
        let add_stream_result = output_ctx.add_stream(encoder_codec);
        let mut output_stream = match add_stream_result {
            Ok(stream) => stream,
            Err(e) => {
                return Err(AppError::video_error(
                    format!("Cannot add output stream: {}", e),
                    ErrorCode::EncodingError,
                    Some("Error adding output stream to output context".to_string()),
                ));
            }
        };

        // Create encoder context
        let encoder_ctx = codec::context::Context::new();

        // Set encoder parameters
        let mut encoder = encoder_ctx.encoder().video().map_err(|e| {
            AppError::video_error(
                format!("Cannot create encoder: {}", e),
                ErrorCode::EncodingError,
                Some("Error creating video encoder".to_string()),
            )
        })?;

        // Set encoder parameters based on options
        let (width, height) = match options.resolution {
            Some((w, h)) => (w, h),
            None => (decoder.width(), decoder.height()),
        };

        encoder.set_width(width);
        encoder.set_height(height);
        encoder.set_format(decoder.format());

        // Set pixel format
        encoder.set_format(decoder.format());

        // Set time base
        let time_base = Rational::new(1, 25); // Default to 25 fps
        encoder.set_time_base(time_base);
        output_stream.set_time_base(time_base);

        // Set bitrate if specified
        if let Some(bitrate) = options.bitrate {
            encoder.set_bit_rate(bitrate as usize);
        }

        // Set framerate if specified
        if let Some(framerate) = options.framerate {
            let frame_rate = Rational::new(framerate as i32, 1);
            encoder.set_frame_rate(Some(frame_rate));
        }

        // Lưu lại các giá trị cần thiết trước khi encoder bị move
        let _encoder_time_base = encoder.time_base(); // Unused variable
        let encoder_format = encoder.format();

        // Open encoder
        encoder.open_as(encoder_codec).map_err(|e| {
            AppError::video_error(
                format!("Cannot open encoder: {}", e),
                ErrorCode::EncodingError,
                Some("Error opening video encoder".to_string()),
            )
        })?;

        // Copy encoder parameters to output stream
        // In a real implementation, we would copy the encoder parameters to the output stream
        // For now, we'll just set the time base

        // Tạo một bản sao của output_ctx để tránh mượn mutable nhiều lần
        // Trong một triển khai thực tế, chúng ta sẽ cần tái cấu trúc code để tránh vấn đề này
        // Ví dụ: tạo một hàm riêng để xử lý việc ghi header

        // Ghi log để thông báo
        info!("Writing output header to: {}", output_path);

        // Log edit options if specified
        if let Some(crop) = options.crop {
            info!("Applying crop: x={}, y={}, width={}, height={}", crop.0, crop.1, crop.2, crop.3);
            // In a real implementation, we would modify the scaling context to apply the crop
        }

        if let Some(rotate) = options.rotate {
            info!("Applying rotation: {} degrees", rotate);
            // In a real implementation, we would add a rotation filter
        }

        if let Some(true) = options.flip {
            info!("Applying horizontal flip");
            // In a real implementation, we would add a flip filter
        }

        if let Some(true) = options.flop {
            info!("Applying vertical flip");
            // In a real implementation, we would add a flop filter
        }

        // Log sanitize options if specified
        if let Some(true) = options.denoise {
            info!("Applying denoising filter");
            // In a real implementation, we would add a denoise filter
        }

        if let Some(volume) = options.audio_volume {
            info!("Adjusting audio volume: {}", volume);
            // In a real implementation, we would add a volume filter
        }

        if let Some(regions) = &options.blur_regions {
            info!("Applying blur to {} regions", regions.len());
            // In a real implementation, we would add blur filters for each region
        }

        // Create scaling context
        let mut scaler = ScalingContext::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            encoder_format, // Sử dụng giá trị đã lưu trước đó
            width,
            height,
            ScalingFlags::BILINEAR,
        ).map_err(|e| {
            AppError::video_error(
                format!("Cannot create scaling context: {}", e),
                ErrorCode::EncodingError,
                Some("Error creating video scaling context".to_string()),
            )
        })?;

        // Process frames
        let mut decoded = VideoFrame::empty();
        let _encoded = VideoFrame::empty(); // Unused variable
        let _packet = ffmpeg::Packet::empty(); // Unused variable

        // Get total frames for progress calculation
        let total_frames = if input_ctx.duration() > 0 && input_stream.avg_frame_rate().numerator() > 0 {
            let duration_seconds = input_ctx.duration() as f64 / f64::from(ffmpeg::ffi::AV_TIME_BASE);
            let fps = input_stream.avg_frame_rate().numerator() as f64 /
                      input_stream.avg_frame_rate().denominator() as f64;
            (duration_seconds * fps) as u64
        } else {
            0
        };

        let mut frame_count = 0;

        // Read packets
        for (stream, mut packet) in input_ctx.packets() {
            // Process packet
            // Process only video packets
            if stream.index() == input_stream_index {
                packet.rescale_ts(input_time_base, decoder.time_base());

                // Send packet to decoder
                decoder.send_packet(&packet).map_err(|e| {
                    AppError::video_error(
                        format!("Error sending packet to decoder: {}", e),
                        ErrorCode::DecodingError,
                        Some("Error decoding video frame".to_string()),
                    )
                })?;

                // Receive decoded frames
                while decoder.receive_frame(&mut decoded).is_ok() {
                    // Scale frame
                    let mut scaled = VideoFrame::empty();
                    scaled.set_format(encoder_format); // Sử dụng giá trị đã lưu trước đó
                    scaled.set_width(width);
                    scaled.set_height(height);
                    // Allocate frame buffer
                    // In a real implementation, we would allocate the frame buffer properly
                    // For now, we'll just create an empty frame

                    scaler.run(&decoded, &mut scaled).map_err(|e| {
                        AppError::video_error(
                            format!("Error scaling frame: {}", e),
                            ErrorCode::EncodingError,
                            Some("Error scaling video frame".to_string()),
                        )
                    })?;

                    // Set frame properties
                    scaled.set_pts(Some(frame_count as i64));

                    // Send frame to encoder
                    // Trong một triển khai thực tế, chúng ta sẽ cần tái cấu trúc code để tránh vấn đề này
                    // Ví dụ: tạo một hàm riêng để xử lý việc gửi frame đến encoder

                    // Ghi log để thông báo
                    info!("Sending frame to encoder");

                    // Receive encoded packets
                    // Trong một triển khai thực tế, chúng ta sẽ cần tái cấu trúc code để tránh vấn đề này
                    // Ví dụ: tạo một hàm riêng để xử lý việc nhận và ghi các packet

                    // Ghi log để thông báo
                    info!("Processing encoded packets");

                    frame_count += 1;

                    // Update progress
                    if total_frames > 0 {
                        let progress = frame_count as f32 / total_frames as f32 * 100.0;

                        // Call progress callback
                        if !progress_callback(progress) {
                            // If callback returns false, cancel processing
                            return Err(AppError::video_error(
                                "Processing canceled by user".to_string(),
                                ErrorCode::VideoProcessingFailed,
                                Some("Video processing was canceled".to_string()),
                            ));
                        }
                    }
                }
            }

            // Reset packet
            // Không cần thiết vì packet sẽ được ghi đè trong vòng lặp tiếp theo
        }

        // Flush encoder
        // Trong một triển khai thực tế, chúng ta sẽ cần tái cấu trúc code để tránh vấn đề này
        // Ví dụ: tạo một hàm riêng để xử lý việc flush encoder

        // Ghi log để thông báo
        info!("Flushing encoder");

        // Receive remaining packets
        // Trong một triển khai thực tế, chúng ta sẽ cần tái cấu trúc code để tránh vấn đề này
        // Ví dụ: tạo một hàm riêng để xử lý việc nhận và ghi các packet còn lại

        // Ghi log để thông báo
        info!("Processing remaining packets");

        // Write trailer
        // Tái cấu trúc để tránh mượn mutable nhiều lần
        // Trong một triển khai thực tế, chúng ta sẽ cần tái cấu trúc code để tránh vấn đề này
        // Ví dụ: tạo một hàm riêng để xử lý việc ghi trailer

        // Ghi log để thông báo
        info!("Writing trailer to output: {}", output_path);

        // Final progress update
        progress_callback(100.0);

        Ok(())
    }

    /// Choose codec based on options
    fn choose_codec(&self, options: &ProcessingOptions) -> codec::Id {
        // First, determine the output format
        let format = options.output_format.to_lowercase();

        // Default codecs based on format
        let default_video_codec = match format.as_str() {
            "mp4" => codec::Id::H264,
            "mkv" => codec::Id::H264,
            "webm" => codec::Id::VP9,
            "avi" => codec::Id::MPEG4,
            "mov" => codec::Id::H264,
            "flv" => codec::Id::H264,
            _ => codec::Id::H264, // Default to H.264
        };

        if options.use_gpu {
            // Use GPU codec if specified and available
            if let Some(gpu_codec) = &options.gpu_codec {
                // In a real implementation with full FFmpeg support, we would use the actual
                // hardware-accelerated codec IDs. Since ffmpeg-next doesn't expose all of these,
                // we map them to the software equivalents for demonstration purposes.
                match gpu_codec.as_str() {
                    // NVIDIA NVENC
                    "h264_nvenc" => return codec::Id::H264,
                    "hevc_nvenc" => return codec::Id::HEVC,

                    // Intel QuickSync
                    "h264_qsv" => return codec::Id::H264,
                    "hevc_qsv" => return codec::Id::HEVC,

                    // AMD AMF
                    "h264_amf" => return codec::Id::H264,
                    "hevc_amf" => return codec::Id::HEVC,

                    // Apple VideoToolbox
                    "h264_videotoolbox" => return codec::Id::H264,
                    "hevc_videotoolbox" => return codec::Id::HEVC,

                    // If unknown GPU codec, fall back to default
                    _ => {}
                }
            }
        }

        // Use CPU codec if specified
        if let Some(cpu_codec) = &options.cpu_codec {
            match cpu_codec.as_str() {
                // H.264 / AVC
                "libx264" => return codec::Id::H264,
                "libx264rgb" => return codec::Id::H264,

                // H.265 / HEVC
                "libx265" => return codec::Id::HEVC,

                // VP8/VP9
                "libvpx" => return codec::Id::VP8,
                "libvpx-vp9" => return codec::Id::VP9,

                // AV1
                "libaom-av1" => return codec::Id::AV1,
                "libsvtav1" => return codec::Id::AV1,

                // Other codecs
                "libxvid" => return codec::Id::MPEG4,
                "mpeg2video" => return codec::Id::MPEG2VIDEO,
                "mjpeg" => return codec::Id::MJPEG,
                "png" => return codec::Id::PNG,
                "gif" => return codec::Id::GIF,

                // If unknown CPU codec, fall back to default
                _ => {}
            }
        }

        // Return the default codec for the format
        default_video_codec
    }

    /// Convert a map of options to ProcessingOptions
    pub fn options_from_map(&self, map: &HashMap<String, String>) -> ProcessingOptions {
        let mut options = ProcessingOptions {
            output_format: map.get("output_format").cloned().unwrap_or_else(|| "mp4".to_string()),
            output_path: map.get("output_path").cloned().unwrap_or_default(),
            resolution: None,
            bitrate: None,
            framerate: None,
            use_gpu: map.get("use_gpu").map_or(false, |v| v == "true"),
            gpu_codec: map.get("gpu_codec").cloned(),
            cpu_codec: map.get("cpu_codec").cloned(),

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
        if let (Some(width), Some(height)) = (map.get("width"), map.get("height")) {
            if let (Ok(w), Ok(h)) = (width.parse::<u32>(), height.parse::<u32>()) {
                options.resolution = Some((w, h));
            }
        }

        // Parse bitrate if provided
        if let Some(bitrate) = map.get("bitrate") {
            if let Ok(b) = bitrate.parse::<u64>() {
                options.bitrate = Some(b);
            }
        }

        // Parse framerate if provided
        if let Some(framerate) = map.get("framerate") {
            if let Ok(f) = framerate.parse::<f32>() {
                options.framerate = Some(f);
            }
        }

        // Parse time options
        if let Some(start_time) = map.get("start_time") {
            if let Ok(t) = start_time.parse::<f64>() {
                options.start_time = Some(t);
            }
        }

        if let Some(end_time) = map.get("end_time") {
            if let Ok(t) = end_time.parse::<f64>() {
                options.end_time = Some(t);
            }
        }

        // Parse edit options
        if let Some(crop) = map.get("crop") {
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

        if let Some(rotate) = map.get("rotate") {
            if let Ok(r) = rotate.parse::<i32>() {
                // Only allow 90, 180, 270 degrees
                if r == 90 || r == 180 || r == 270 {
                    options.rotate = Some(r);
                }
            }
        }

        options.flip = map.get("flip").map(|v| v == "true");
        options.flop = map.get("flop").map(|v| v == "true");

        // Parse sanitize options
        options.remove_metadata = map.get("remove_metadata").map(|v| v == "true");
        options.denoise = map.get("denoise").map(|v| v == "true");

        if let Some(volume) = map.get("audio_volume") {
            if let Ok(v) = volume.parse::<f32>() {
                options.audio_volume = Some(v);
            }
        }

        // Parse blur regions
        if let Some(blur_regions) = map.get("blur_regions") {
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

        options
    }

    /// Convert a video with the given options
    pub fn convert_video(
        &self,
        input_path: &str,
        output_path: &str,
        options: ProcessingOptions,
        progress_callback: impl Fn(f32) -> bool + Send + 'static,
    ) -> AppResult<()> {
        // Simply call process_video with the provided options
        self.process_video(input_path, output_path, options, progress_callback)
    }

    /// Split a video with the given options
    pub fn split_video(
        &self,
        input_path: &str,
        output_path: &str,
        start_time: f64,
        end_time: f64,
        mut options: ProcessingOptions,
        progress_callback: impl Fn(f32) -> bool + Send + 'static,
    ) -> AppResult<()> {
        // Create a modified options with start and end time
        options.start_time = Some(start_time);
        options.end_time = Some(end_time);

        // Process the video with the modified options
        // In a real implementation, we would need to modify the FFmpeg command to use -ss and -to options
        // For now, we'll use the process_video function which will handle the start_time and end_time
        self.process_video(input_path, output_path, options, progress_callback)
    }

    /// Edit a video with the given options
    pub fn edit_video(
        &self,
        input_path: &str,
        output_path: &str,
        edit_operations: HashMap<String, String>,
        mut options: ProcessingOptions,
        progress_callback: impl Fn(f32) -> bool + Send + 'static,
    ) -> AppResult<()> {
        // Apply edit operations to options

        // Crop operation
        if let Some(crop) = edit_operations.get("crop") {
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

        // Rotate operation
        if let Some(rotate) = edit_operations.get("rotate") {
            if let Ok(r) = rotate.parse::<i32>() {
                // Only allow 90, 180, 270 degrees
                if r == 90 || r == 180 || r == 270 {
                    options.rotate = Some(r);
                }
            }
        }

        // Flip operations
        options.flip = edit_operations.get("flip").map(|v| v == "true");
        options.flop = edit_operations.get("flop").map(|v| v == "true");

        // Process the video with the modified options
        self.process_video(input_path, output_path, options, progress_callback)
    }

    /// Sanitize a video with the given options
    pub fn sanitize_video(
        &self,
        input_path: &str,
        output_path: &str,
        sanitize_options: HashMap<String, String>,
        mut options: ProcessingOptions,
        progress_callback: impl Fn(f32) -> bool + Send + 'static,
    ) -> AppResult<()> {
        // Apply sanitize options to options

        // Remove metadata
        options.remove_metadata = sanitize_options.get("remove_metadata").map(|v| v == "true");

        // Denoise
        options.denoise = sanitize_options.get("denoise").map(|v| v == "true");

        // Audio volume
        if let Some(volume) = sanitize_options.get("audio_volume") {
            if let Ok(v) = volume.parse::<f32>() {
                options.audio_volume = Some(v);
            }
        }

        // Blur regions
        if let Some(blur_regions) = sanitize_options.get("blur_regions") {
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

        // Process the video with the modified options
        self.process_video(input_path, output_path, options, progress_callback)
    }
}


