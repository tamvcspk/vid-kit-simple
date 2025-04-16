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
    Running(f32), // Tiến độ (0.0 - 100.0)
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

    /// Lấy thông tin về tệp video
    pub fn get_video_info(&self, file_path: &str) -> Result<VideoInfo, String> {
        // Chuẩn hóa đường dẫn để đảm bảo nó hoạt động trên mọi hệ điều hành
        let normalized_path = file_path.replace("\\", "/");

        let input_ctx = input(&normalized_path)
            .map_err(|e| format!("Không thể mở tệp video: {}", e))?;

        let stream = input_ctx
            .streams()
            .best(MediaType::Video)
            .ok_or_else(|| "Không tìm thấy luồng video".to_string())?;

        let codec_ctx = ffmpeg::codec::context::Context::from_parameters(stream.parameters())
            .map_err(|e| format!("Không thể tạo context từ parameters: {}", e))?;

        let decoder = codec_ctx.decoder()
            .video()
            .map_err(|e| format!("Không thể tạo decoder: {}", e))?;

        // Lấy thông tin định dạng
        let format_name = input_ctx.format().name().to_string();

        // Lấy thông tin thời lượng
        let duration = if input_ctx.duration() > 0 {
            input_ctx.duration() as f64 / f64::from(ffmpeg::ffi::AV_TIME_BASE)
        } else {
            0.0
        };

        // Lấy thông tin bitrate
        let bitrate = if input_ctx.bit_rate() > 0 {
            input_ctx.bit_rate() as u64
        } else {
            0
        };

        // Lấy thông tin framerate
        let framerate = if stream.avg_frame_rate().numerator() != 0 {
            stream.avg_frame_rate().numerator() as f32 / stream.avg_frame_rate().denominator() as f32
        } else {
            0.0
        };

        // Lấy thông tin codec
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

    /// Tạo một tác vụ xử lý mới
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

    /// Chạy tác vụ xử lý theo ID
    pub fn run_task(&mut self, task_id: &str) -> Result<(), String> {
        let task_index = self
            .tasks
            .iter()
            .position(|t| t.id == task_id)
            .ok_or_else(|| format!("Không tìm thấy tác vụ có ID: {}", task_id))?;

        // Cập nhật trạng thái
        self.tasks[task_index].status = ProcessingStatus::Running(0.0);

        // Tạo channel để cập nhật tiến độ
        let (tx, rx) = channel();
        self.progress_channel = Some((tx.clone(), rx));

        // Clone task để sử dụng trong thread
        let task = self.tasks[task_index].clone();
        let task_id_clone = task_id.to_string();

        // Spawn một luồng để xử lý video
        thread::spawn(move || {
            // Tạo thư mục đầu ra nếu không tồn tại
            if let Some(parent) = Path::new(&task.options.output_path).parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    let _ = tx.send((task_id_clone.clone(), -1.0)); // Báo hiệu lỗi
                    eprintln!("Không thể tạo thư mục đầu ra: {}", e);
                    return;
                }
            }

            // Chuẩn hóa đường dẫn đầu vào
            let normalized_input_path = task.input_file.replace("\\", "/");
            eprintln!("Input path: {}", normalized_input_path);

            // Mở tệp đầu vào
            let mut input_ctx = match input(&normalized_input_path) {
                Ok(ctx) => ctx,
                Err(e) => {
                    let _ = tx.send((task_id_clone.clone(), -1.0)); // Báo hiệu lỗi
                    eprintln!("Không thể mở tệp đầu vào: {}", e);
                    return;
                }
            };

            // Chuẩn hóa đường dẫn đầu ra
            let normalized_output_path = task.options.output_path.replace("\\", "/");
            eprintln!("Output path: {}", normalized_output_path);

            // Tạo output context
            let mut output_ctx = match output(&normalized_output_path) {
                Ok(ctx) => ctx,
                Err(e) => {
                    let _ = tx.send((task_id_clone.clone(), -1.0)); // Báo hiệu lỗi
                    eprintln!("Không thể tạo output context: {}", e);
                    return;
                }
            };

            // Tìm luồng video
            let input_stream = match input_ctx.streams().best(MediaType::Video) {
                Some(stream) => stream,
                None => {
                    let _ = tx.send((task_id_clone, -1.0)); // Báo hiệu lỗi
                    eprintln!("Không tìm thấy luồng video");
                    return;
                }
            };

            let input_stream_index = input_stream.index();
            let input_time_base = input_stream.time_base();

            // Tạo decoder
            let decoder_ctx = match ffmpeg::codec::context::Context::from_parameters(input_stream.parameters()) {
                Ok(ctx) => ctx,
                Err(e) => {
                    let _ = tx.send((task_id_clone, -1.0)); // Báo hiệu lỗi
                    eprintln!("Không thể tạo decoder context: {}", e);
                    return;
                }
            };

            let mut decoder = match decoder_ctx.decoder().video() {
                Ok(dec) => dec,
                Err(e) => {
                    let _ = tx.send((task_id_clone, -1.0)); // Báo hiệu lỗi
                    eprintln!("Không thể tạo video decoder: {}", e);
                    return;
                }
            };

            // Sử dụng H264 codec thay vì MPEG4 để tránh lỗi B-frames
            let codec_id = codec::Id::H264; // H264 có nhiều tùy chọn hơn và được hỗ trợ rộng rãi

            let encoder_codec = match encoder::find(codec_id) {
                Some(codec) => codec,
                None => {
                    let _ = tx.send((task_id_clone, -1.0)); // Báo hiệu lỗi
                    eprintln!("Không tìm thấy encoder codec");
                    return;
                }
            };

            // Tạo stream đầu ra
            let mut output_stream = match output_ctx.add_stream(encoder_codec) {
                Ok(stream) => stream,
                Err(e) => {
                    let _ = tx.send((task_id_clone, -1.0)); // Báo hiệu lỗi
                    eprintln!("Không thể thêm stream đầu ra: {}", e);
                    return;
                }
            };

            // Tạo encoder context
            let encoder_ctx = codec::context::Context::new_with_codec(encoder_codec);

            // Tạo encoder
            let mut enc = match encoder_ctx.encoder().video() {
                Ok(enc) => enc,
                Err(e) => {
                    let _ = tx.send((task_id_clone.clone(), -1.0)); // Báo hiệu lỗi
                    eprintln!("Không thể tạo video encoder: {}", e);
                    return;
                }
            };

            // Thiết lập các tham số encoder
            let width = if let Some((w, _)) = task.options.resolution { w } else { decoder.width() };
            let height = if let Some((_, h)) = task.options.resolution { h } else { decoder.height() };
            let framerate = if let Some(fps) = task.options.framerate {
                Rational::new(fps as i32, 1)
            } else {
                decoder.frame_rate().unwrap_or(Rational::new(30, 1)) // Default to 30fps if not available
            };

            // Đảm bảo kích thước là số chẵn (yêu cầu của một số encoder)
            let width = width - (width % 2);
            let height = height - (height % 2);

            enc.set_width(width);
            enc.set_height(height);
            // Sử dụng pixel format phù hợp với encoder
            enc.set_format(ffmpeg::format::pixel::Pixel::YUV420P); // Định dạng pixel phổ biến nhất
            enc.set_time_base(Rational::new(1, 25));
            enc.set_frame_rate(Some(Rational::new(25, 1))); // Sử dụng framerate ổn định 25fps

            // Thiết lập các tham số codec
            enc.set_max_b_frames(0); // Không sử dụng B-frames để tránh lỗi
            enc.set_gop(25); // Thiết lập GOP size (1 giây với 25fps)

            // Thêm các tham số để tránh lỗi timestamp
            enc.set_flags(ffmpeg::codec::flag::Flags::GLOBAL_HEADER);
            output_stream.set_time_base(Rational::new(1, 25000)); // Timebase chi tiết hơn

            if let Some(bitrate) = task.options.bitrate {
                // Chuyển đổi bitrate từ u64 sang usize
                let bitrate_usize = bitrate.try_into().unwrap_or(0);
                enc.set_bit_rate(bitrate_usize);
            }

            // Thiết lập parameters cho output stream
            output_stream.set_parameters(&enc);

            // Mở encoder
            let mut enc = match enc.open_as(encoder_codec) {
                Ok(enc) => enc,
                Err(e) => {
                    let _ = tx.send((task_id_clone.clone(), -1.0)); // Báo hiệu lỗi
                    eprintln!("Không thể mở encoder: {}", e);
                    return;
                }
            };

            // Thiết lập time base cho output stream
            let output_time_base = output_stream.time_base();

            // Tạo scaler để chuyển đổi định dạng frame nếu cần
            let mut scaler = match ScalingContext::get(
                decoder.format(),
                decoder.width(),
                decoder.height(),
                ffmpeg::format::pixel::Pixel::YUV420P, // Đảm bảo định dạng pixel đầu ra là YUV420P
                width,
                height,
                ScalingFlags::BILINEAR,
            ) {
                Ok(s) => s,
                Err(e) => {
                    let _ = tx.send((task_id_clone.clone(), -1.0)); // Báo hiệu lỗi
                    eprintln!("Không thể tạo scaler: {}", e);
                    return;
                }
            };

            // Ghi header cho tệp đầu ra
            if let Err(e) = output_ctx.write_header() {
                let _ = tx.send((task_id_clone.clone(), -1.0)); // Báo hiệu lỗi
                eprintln!("Không thể ghi header: {}", e);
                return;
            }

            let mut decoded_frame = VideoFrame::empty();
            let mut scaled_frame = VideoFrame::empty();
            let mut frame_count = 0;
            let mut total_frames = 0;

            // Ước tính tổng số frame
            if input_ctx.duration() > 0 && framerate.numerator() > 0 {
                total_frames = (input_ctx.duration() as f64 / f64::from(ffmpeg::ffi::AV_TIME_BASE) * framerate.numerator() as f64 / framerate.denominator() as f64) as i32;
            }

            // Xử lý từng packet
            for (stream, packet) in input_ctx.packets() {
                if stream.index() == input_stream_index {
                    // Gửi packet đến decoder
                    if let Err(e) = decoder.send_packet(&packet) {
                        eprintln!("Lỗi khi gửi packet đến decoder: {}", e);
                        continue;
                    }

                    // Nhận các frame đã giải mã
                    while decoder.receive_frame(&mut decoded_frame).is_ok() {
                        frame_count += 1;

                        // Scale frame nếu cần
                        if let Err(e) = scaler.run(&decoded_frame, &mut scaled_frame) {
                            eprintln!("Lỗi khi scale frame: {}", e);
                            continue;
                        }

                        // Thiết lập timestamp cho frame với giá trị tăng dần
                        // Sử dụng frame_count * 1000 để đảm bảo không có timestamp trùng lặp
                        scaled_frame.set_pts(Some(frame_count * 1000));
                        scaled_frame.set_kind(decoded_frame.kind());

                        // Gửi frame đến encoder
                        if let Err(e) = enc.send_frame(&scaled_frame) {
                            eprintln!("Lỗi khi gửi frame đến encoder: {}", e);
                            continue;
                        }

                        // Nhận các packet đã mã hóa
                        let mut encoded_packet = ffmpeg::Packet::empty();
                        while enc.receive_packet(&mut encoded_packet).is_ok() {
                            // Thiết lập stream index và rescale timestamp
                            encoded_packet.set_stream(0);
                            encoded_packet.rescale_ts(input_time_base, output_time_base);

                            // Ghi packet vào tệp đầu ra
                            if let Err(e) = encoded_packet.write_interleaved(&mut output_ctx) {
                                eprintln!("Lỗi khi ghi packet: {}", e);
                            }
                        }

                        // Cập nhật tiến độ
                        if total_frames > 0 {
                            let progress = (frame_count as f32 / total_frames as f32) * 100.0;

                            // Gửi tiến độ qua channel
                            let _ = tx.send((task_id_clone.clone(), progress));
                        }
                    }
                }
            }

            // Flush decoder
            if let Err(e) = decoder.send_eof() {
                eprintln!("Lỗi khi gửi EOF đến decoder: {}", e);
            }

            // Nhận các frame còn lại từ decoder
            while decoder.receive_frame(&mut decoded_frame).is_ok() {
                // Scale frame nếu cần
                if let Err(e) = scaler.run(&decoded_frame, &mut scaled_frame) {
                    eprintln!("Lỗi khi scale frame: {}", e);
                    continue;
                }

                // Thiết lập timestamp cho frame với giá trị tăng dần
                frame_count += 1; // Tăng frame_count để tránh timestamp trùng lặp
                scaled_frame.set_pts(Some(frame_count * 1000));
                scaled_frame.set_kind(decoded_frame.kind());

                // Gửi frame đến encoder
                if let Err(e) = enc.send_frame(&scaled_frame) {
                    eprintln!("Lỗi khi gửi frame đến encoder: {}", e);
                    continue;
                }

                // Nhận các packet đã mã hóa
                let mut encoded_packet = ffmpeg::Packet::empty();
                while enc.receive_packet(&mut encoded_packet).is_ok() {
                    // Thiết lập stream index và rescale timestamp
                    encoded_packet.set_stream(0);
                    encoded_packet.rescale_ts(input_time_base, output_time_base);

                    // Ghi packet vào tệp đầu ra
                    if let Err(e) = encoded_packet.write_interleaved(&mut output_ctx) {
                        eprintln!("Lỗi khi ghi packet: {}", e);
                    }
                }
            }

            // Flush encoder
            if let Err(e) = enc.send_eof() {
                eprintln!("Lỗi khi gửi EOF đến encoder: {}", e);
            }

            // Nhận các packet còn lại từ encoder
            let mut encoded_packet = ffmpeg::Packet::empty();
            while enc.receive_packet(&mut encoded_packet).is_ok() {
                // Thiết lập stream index và rescale timestamp
                encoded_packet.set_stream(0);
                encoded_packet.rescale_ts(input_time_base, output_time_base);

                // Ghi packet vào tệp đầu ra
                if let Err(e) = encoded_packet.write_interleaved(&mut output_ctx) {
                    eprintln!("Lỗi khi ghi packet: {}", e);
                }
            }

            // Ghi trailer cho tệp đầu ra
            if let Err(e) = output_ctx.write_trailer() {
                let _ = tx.send((task_id_clone.clone(), -1.0)); // Báo hiệu lỗi
                eprintln!("Không thể ghi trailer: {}", e);
                return;
            }

            // Báo hiệu hoàn thành
            let _ = tx.send((task_id_clone.clone(), 100.0));
        });

        Ok(())
    }

    /// Kiểm tra tiến độ của các tác vụ
    pub fn check_progress(&mut self) -> Vec<(String, ProcessingStatus)> {
        if let Some((_, ref rx)) = self.progress_channel {
            // Nhận tất cả các cập nhật tiến độ hiện có
            while let Ok((task_id, progress)) = rx.try_recv() {
                if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
                    if progress < 0.0 {
                        task.status =
                            ProcessingStatus::Failed("Lỗi trong quá trình xử lý".to_string());
                    } else if progress >= 100.0 {
                        task.status = ProcessingStatus::Complete;
                    } else {
                        task.status = ProcessingStatus::Running(progress);
                    }
                }
            }
        }

        // Trả về trạng thái hiện tại của tất cả các tác vụ
        self.tasks
            .iter()
            .map(|task| (task.id.clone(), task.status.clone()))
            .collect()
    }

    /// Lấy thông tin về một tác vụ
    pub fn get_task(&self, task_id: &str) -> Option<&ProcessingTask> {
        self.tasks.iter().find(|t| t.id == task_id)
    }

    /// Lấy danh sách tất cả các tác vụ
    pub fn get_tasks(&self) -> &[ProcessingTask] {
        &self.tasks
    }
}
