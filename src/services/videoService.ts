import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { appDataDir, join } from '@tauri-apps/api/path';
import { mkdir, exists } from '@tauri-apps/plugin-fs';

import { BaseService } from './baseService';
import { ErrorCategory } from '../utils';
import { GpuInfo, ProcessingOptions, VideoInfo } from '../types';

class VideoService extends BaseService {
  /**
   * Open dialog to select video file
   */
  async selectVideoFile(): Promise<string | null> {
    return this.withErrorHandling(
      async () => {
        const selectedPath = await open({
          multiple: false,
          filters: [{
            name: 'Video',
            extensions: ['mp4', 'avi', 'mkv', 'mov', 'wmv', 'webm']
          }]
        });

        if (selectedPath === null) {
          return null; // User canceled
        }

        return selectedPath as string;
      },
      'Failed to open file selection dialog',
      ErrorCategory.IO
    );
  }

  /**
   * Open dialog to select a directory
   */
  async selectDirectory(): Promise<string | null> {
    return this.withErrorHandling(
      async () => {
        const selectedPath = await open({
          directory: true,
          multiple: false
        });

        if (selectedPath === null) {
          return null; // User canceled
        }

        return selectedPath as string;
      },
      'Failed to open directory selection dialog',
      ErrorCategory.IO
    );
  }

  /**
   * Get video file information from backend
   */
  async getVideoInfo(filePath: string): Promise<VideoInfo | null> {
    return this.withErrorHandling(
      async () => {
        return await invoke<VideoInfo>('get_video_info', { path: filePath });
      },
      'Failed to get video information',
      ErrorCategory.FFmpeg
    );
  }

  /**
   * Create output directory for converted videos
   */
  async createOutputDirectory(): Promise<string | null> {
    return this.withErrorHandling(
      async () => {
        const appDataDirPath = await appDataDir();
        const outputDir = await join(appDataDirPath, 'converted_videos');

        const dirExists = await exists(outputDir);
        if (!dirExists) {
          await mkdir(outputDir, { recursive: true });
        }

        return outputDir;
      },
      'Failed to create output directory',
      ErrorCategory.IO
    );
  }

  /**
   * Generate output path based on input file and format
   */
  async generateOutputPath(inputFilePath: string, outputFormat: string): Promise<string | null> {
    return this.withErrorHandling(
      async () => {
        const fileName = inputFilePath.split('/').pop()?.split('\\').pop() || 'output';
        const fileNameWithoutExt = fileName.split('.').slice(0, -1).join('.');

        const outputDir = await this.createOutputDirectory();
        if (!outputDir) return null;

        return await join(outputDir, `${fileNameWithoutExt}_converted.${outputFormat}`);
      },
      'Failed to generate output path',
      ErrorCategory.IO
    );
  }

  /**
   * Create a new video conversion task
   */
  async createConversionTask(
    inputFilePath: string,
    options: ProcessingOptions
  ): Promise<string | null> {
    return this.withErrorHandling(
      async () => {
        // Ensure output path is provided
        if (!options.output_path) {
          throw new Error('Output path is required');
        }

        return await invoke<string>('create_processing_task', {
          inputFile: inputFilePath,
          options: options
        });
      },
      'Failed to create conversion task',
      ErrorCategory.Task
    );
  }

  /**
   * Start video conversion
   */
  async startConversion(taskId: string): Promise<boolean> {
    const result = await this.withErrorHandling(
      async () => {
        await invoke<void>('run_processing_task', { taskId });
        return true;
      },
      'Failed to start conversion',
      ErrorCategory.Task
    );
    return result === null ? false : result;
  }

  /**
   * Get information about GPU and supported codecs
   */
  async getGpuInfo(): Promise<GpuInfo | null> {
    return this.withErrorHandling(
      async () => {
        return await invoke<GpuInfo>('check_gpu_availability');
      },
      'Failed to get GPU information',
      ErrorCategory.Gpu
    );
  }

  /**
   * Get GPU codec based on GPU information if available
   */
  async getGpuCodec(format: string = 'mp4'): Promise<string | undefined> {
    const result = await this.withErrorHandling<string | undefined>(
      async () => {
        const gpuInfo = await this.getGpuInfo();

        if (!gpuInfo || !gpuInfo.isAvailable) {
          return undefined;
        }

        // Logic for selecting codec based on GPU and format
        if (gpuInfo.vendor.toLowerCase().includes('nvidia')) {
          return format === 'mp4' ? 'h264_nvenc' : 'hevc_nvenc';
        } else if (gpuInfo.vendor.toLowerCase().includes('intel')) {
          return 'h264_qsv';
        } else if (gpuInfo.vendor.toLowerCase().includes('amd')) {
          return 'h264_amf';
        }

        return undefined;
      },
      'Failed to determine GPU codec',
      ErrorCategory.Gpu
    );
    return result === null ? undefined : result;
  }

  /**
   * Get CPU codec for output format
   */
  getCpuCodec(format: string): string {
    switch (format) {
      case 'mp4': return 'libx264';
      case 'webm': return 'libvpx-vp9';
      case 'mkv': return 'libx264';
      case 'avi': return 'libx264';
      case 'mov': return 'libx264';
      default: return 'libx264';
    }
  }
}

// Export singleton instance
export default new VideoService();