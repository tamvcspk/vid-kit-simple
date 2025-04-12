export interface VideoInfo {
  path: string;
  format: string;
  duration: number;
  width: number;
  height: number;
  bitrate: number;
  codec: string;
  framerate: number;
}

export interface ProcessingOptions {
  outputFormat: string;
  outputPath: string;
  resolution?: [number, number];
  bitrate?: number;
  framerate?: number;
  use_gpu: boolean;
  gpu_codec?: string;
  cpu_codec?: string;
}

export enum ProcessingStatus {
  Pending = 'pending',
  Running = 'running',
  Complete = 'complete',
  Failed = 'failed'
}

export interface ProcessingTask {
  id: string;
  inputFile: string;
  options: ProcessingOptions;
  status: ProcessingStatus;
  progress?: number;
}

// GPU info interface
export interface GpuInfo {
  name: string;
  vendor: string;
  isAvailable: boolean;
  supportedCodecs: string[];
}