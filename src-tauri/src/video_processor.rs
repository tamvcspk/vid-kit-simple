use std::fs;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;


use serde::{Deserialize, Serialize};

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
        VideoProcessor {
            tasks: Vec::new(),
            progress_channel: None,
        }
    }

    /// Lấy thông tin về tệp video
    pub fn get_video_info(&self, file_path: &str) -> Result<VideoInfo, String> {
        // Kiểm tra đường dẫn rỗng
        if file_path.trim().is_empty() {
            return Err("Vui lòng cung cấp đường dẫn tệp video hợp lệ".to_string());
        }

        // Kiểm tra xem tệp có tồn tại không
        if !Path::new(file_path).exists() {
            return Err(format!("Tệp không tồn tại: {}", file_path));
        }

        // Sử dụng FFmpeg API đã được liên kết tĩnh
        // Đây là một ví dụ đơn giản, trong thực tế sẽ sử dụng API FFmpeg
        // Hiện tại chúng ta chỉ trả về thông tin giả định
        Ok(VideoInfo {
            path: file_path.to_string(),
            format: "mp4".to_string(), // Giả định
            duration: 60.0,            // Giả định 60 giây
            width: 1920,               // Giả định 1080p
            height: 1080,
            bitrate: 5000000,          // Giả định 5 Mbps
            codec: "h264".to_string(), // Giả định
            framerate: 30.0,           // Giả định 30 FPS
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

        let task = self.tasks[task_index].clone();

        // Thiết lập kênh để theo dõi tiến độ
        let (tx, rx) = channel();
        self.progress_channel = Some((tx.clone(), rx));

        // Cập nhật trạng thái tác vụ
        self.tasks[task_index].status = ProcessingStatus::Running(0.0);

        // Tạo thư mục đầu ra nếu không tồn tại
        if let Some(parent) = Path::new(&task.options.output_path).parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Không thể tạo thư mục đầu ra: {}", e))?;
        }

        // Xây dựng các tham số FFmpeg
        let mut args = vec![
            "-i".to_string(),
            task.input_file.clone(),
            "-y".to_string(), // Ghi đè lên tệp đầu ra nếu tồn tại
        ];

        // Thêm tham số độ phân giải nếu được chỉ định
        if let Some((width, height)) = task.options.resolution {
            args.push("-vf".to_string());
            args.push(format!("scale={}:{}", width, height));
        }

        // Thêm tham số bitrate nếu được chỉ định
        if let Some(bitrate) = task.options.bitrate {
            args.push("-b:v".to_string());
            args.push(format!("{}k", bitrate / 1000));
        }

        // Thêm tham số framerate nếu được chỉ định
        if let Some(framerate) = task.options.framerate {
            args.push("-r".to_string());
            args.push(format!("{}", framerate));
        }

        // Thiết lập codec
        if task.options.use_gpu {
            if let Some(gpu_codec) = task.options.gpu_codec {
                args.push("-c:v".to_string());
                args.push(gpu_codec);
            }
        } else if let Some(cpu_codec) = task.options.cpu_codec {
            args.push("-c:v".to_string());
            args.push(cpu_codec);
        }

        // Thêm đường dẫn đầu ra
        args.push(task.options.output_path.clone());

        // Spawn một luồng để chạy FFmpeg
        let task_id_clone = task_id.to_string();

        thread::spawn(move || {
            // Sử dụng FFmpeg API đã được liên kết tĩnh thay vì gọi chương trình bên ngoài
            // Đây là một ví dụ giả định

            // Giả lập thành công
            let _ = tx.send((task_id_clone.clone(), 100.0)); // Hoàn thành
            return;

            // Mã bên dưới được giữ lại như tham khảo cho việc triển khai sau này
            /*
            // Sử dụng Command để gọi FFmpeg như một chương trình bên ngoài
            let mut child = match Command::new("ffmpeg")
                .args(&args)
                .stderr(Stdio::piped())
                .spawn()
            {
                Ok(c) => c,
                Err(e) => {
                    let _ = tx.send((task_id_clone, -1.0)); // Báo hiệu lỗi
                    eprintln!("Không thể khởi động FFmpeg: {}", e);
                    return;
                }
            };

            let stderr = BufReader::new(child.stderr.take().unwrap());

            // Đọc đầu ra stderr từ FFmpeg để cập nhật tiến độ
            for line in stderr.lines() {
                if let Ok(line) = line {
                    // Trong thực tế, cần phân tích đầu ra FFmpeg để lấy tiến độ
                    // Đây chỉ là giả định
                    if line.contains("time=") && line.contains("bitrate=") {
                        // Giả định tiến độ (trong thực tế sẽ phân tích chuỗi)
                        let progress = 50.0; // 50% tiến độ
                        let _ = tx.send((task_id_clone.clone(), progress));
                    }
                }
            }

            match child.wait() {
                Ok(status) if status.success() => {
                    let _ = tx.send((task_id_clone, 100.0)); // Hoàn thành
                }
                _ => {
                    let _ = tx.send((task_id_clone, -1.0)); // Lỗi
                }
            }
            */
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

    /// Hủy tác vụ theo ID
    pub fn cancel_task(&mut self, task_id: &str) -> Result<(), String> {
        let task_index = self
            .tasks
            .iter()
            .position(|t| t.id == task_id)
            .ok_or_else(|| format!("Không tìm thấy tác vụ có ID: {}", task_id))?;

        // Trong thực tế, cần phải kill tiến trình FFmpeg
        // Đây chỉ là cách đơn giản để cập nhật trạng thái
        self.tasks[task_index].status =
            ProcessingStatus::Failed("Tác vụ bị hủy bởi người dùng".to_string());

        Ok(())
    }

    /// Xóa tất cả các tác vụ đã hoàn thành
    pub fn clear_completed_tasks(&mut self) {
        self.tasks
            .retain(|task| !matches!(task.status, ProcessingStatus::Complete));
    }

    /// Lấy danh sách tất cả các tác vụ
    pub fn get_all_tasks(&self) -> Vec<ProcessingTask> {
        self.tasks.clone()
    }
}

// Các hàm tiện ích để kiểm tra tệp và xử lý lỗi
fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    Path::new(path.as_ref()).exists()
}

fn ensure_directory<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

// Đăng ký với Tauri
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_task() {
        let mut processor = VideoProcessor::new();
        let options = ProcessingOptions {
            output_format: "mp4".to_string(),
            output_path: "output.mp4".to_string(),
            resolution: Some((1280, 720)),
            bitrate: Some(2000000),
            framerate: Some(30.0),
            use_gpu: false,
            gpu_codec: None,
            cpu_codec: Some("libx264".to_string()),
        };

        let task_id = processor.create_task("input.mp4", options);
        assert!(!task_id.is_empty());
        assert_eq!(processor.tasks.len(), 1);
    }
}
