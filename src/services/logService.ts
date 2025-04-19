import { invoke } from '@tauri-apps/api/core';
import { BaseService } from './baseService';
import { ErrorCategory } from '../utils';

/**
 * Service for accessing application logs
 */
export class LogService extends BaseService {
  /**
   * Get the path to the current log file
   * @returns The path to the current log file or null if an error occurred
   */
  async getCurrentLogFilePath(): Promise<string | null> {
    return this.withErrorHandling(
      async () => {
        return await invoke<string>('get_current_log_file_path');
      },
      'Failed to get log file path',
      ErrorCategory.IO
    );
  }

  /**
   * Open the current log file in the default text editor
   * @returns True if the log file was opened successfully, false otherwise
   */
  async openCurrentLogFile(): Promise<boolean | null> {
    return this.withErrorHandling(
      async () => {
        return await invoke<boolean>('open_log_file');
      },
      'Failed to open log file',
      ErrorCategory.IO,
      false
    );
  }

  /**
   * Open the log directory in the file explorer
   * @returns True if the log directory was opened successfully, false otherwise
   */
  async openLogDirectory(): Promise<boolean | null> {
    return this.withErrorHandling(
      async () => {
        return await invoke<boolean>('open_log_directory');
      },
      'Failed to open log directory',
      ErrorCategory.IO,
      false
    );
  }
}

// Create singleton instance
export const logService = new LogService();
