// Định nghĩa kiểu dữ liệu cho state từ Rust

// Thông tin về GPU
export interface GpuInfo {
  name: string;
  vendor: string;
  is_available: boolean;
  supported_codecs: string[];
}

// Thông tin chung về ứng dụng
export interface AppState {
  is_initialized: boolean;
  app_version: string;
  ffmpeg_version: string | null;
  gpu_available: boolean;
  gpus: GpuInfo[];
  selected_gpu_index: number; // -1 cho CPU, 0+ cho GPU
}

// Thông tin về file video
export interface FileInfo {
  id: string;
  name: string;
  path: string;
  size: number;
  type: string;
  duration?: number;
  resolution?: { width: number; height: number };
  thumbnail?: string;
}

// Trạng thái chuyển đổi video
export interface ConversionState {
  active_tasks: string[];
  completed_tasks: string[];
  failed_tasks: string[];
  current_progress: number;
  files: FileInfo[];
  selected_file_id: string | null;
}

// Tùy chọn người dùng
export interface UserPreferencesState {
  default_output_dir: string | null;
  default_format: string;
  use_gpu: boolean;
  theme: string;
}

// Global state kết hợp tất cả các state
export interface GlobalState {
  app: AppState;
  conversion: ConversionState;
  preferences: UserPreferencesState;
}
