// Type definitions for state from Rust

// GPU information
export interface GpuInfo {
  name: string;
  vendor: string;
  is_available: boolean;
  supported_codecs: string[];
}

// General application information
export interface AppState {
  is_initialized: boolean;
  app_version: string;
  ffmpeg_version: string | null;
  gpu_available: boolean;
  gpus: GpuInfo[];
  selected_gpu_index: number; // -1 for CPU, 0+ for GPU
}

// Video file information
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

// Video conversion state
export interface ConversionState {
  active_tasks: string[];
  completed_tasks: string[];
  failed_tasks: string[];
  current_progress: number;
  files: FileInfo[];
  selected_file_id: string | null;
}

// User preferences
export interface UserPreferencesState {
  default_output_dir: string | null;
  default_format: string;
  use_gpu: boolean;
  theme: string;
}

// Global state combining all states
export interface GlobalState {
  app: AppState;
  conversion: ConversionState;
  preferences: UserPreferencesState;
}
