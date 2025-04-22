import { ProcessingOptions, ProcessingStatus } from './video.types';

/**
 * Represents a conversion preset
 */
export interface ConversionPreset {
  id: string;
  name: string;
  description: string;
  output_format: string;
  resolution: ResolutionSetting;
  bitrate?: number;
  fps?: number;
  codec?: string;
  use_gpu: boolean;
  audio_codec?: string;
  created_at: string;
  updated_at: string;
}

/**
 * Resolution setting for a preset
 */
export type ResolutionSetting = 
  | { type: 'original' }
  | { type: 'preset', width: number, height: number }
  | { type: 'custom', width: number, height: number };

/**
 * Represents a processing task
 */
export interface Task {
  id: string;
  input_path: string;
  output_path: string;
  status: TaskStatus;
  progress: number;
  error?: string;
  attempts: number;
  created_at: string;
  completed_at?: string;
  config: ProcessingOptions;
  type: 'convert' | 'split' | 'edit' | 'sanitize';
}

/**
 * Status of a task
 */
export enum TaskStatus {
  Pending = 'pending',
  Running = 'running',
  Paused = 'paused',
  Completed = 'completed',
  Failed = 'failed',
  Canceled = 'canceled'
}

/**
 * Structure of the presets store
 */
export interface PresetsStore {
  presets: ConversionPreset[];
}

/**
 * Structure of the tasks store
 */
export interface TasksStore {
  tasks: Task[];
  queue: string[]; // Array of task IDs in queue order
}

/**
 * Structure of the configuration store
 */
export interface ConfigStore {
  outputFolder: string;
  maxParallelJobs: number;
  retryLimit: number;
  selectedGpu: number;
  theme: 'light' | 'dark';
  defaultFormat: string;
  useGpu: boolean;
}
