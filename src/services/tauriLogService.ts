import { info, warn, error, debug, trace } from '@tauri-apps/plugin-log';
import { BaseService } from './baseService';
import { ErrorCategory } from '../utils';

/**
 * Service for interacting with the Tauri logging plugin
 */
export class TauriLogService extends BaseService {

  /**
   * Log an info message
   * @param message The message to log
   */
  async logInfo(message: string): Promise<void> {
    return this.withErrorHandling(
      async () => {
        await info(message);
      },
      'Failed to log info message',
      ErrorCategory.IO
    );
  }

  /**
   * Log a warning message
   * @param message The message to log
   */
  async logWarning(message: string): Promise<void> {
    return this.withErrorHandling(
      async () => {
        await warn(message);
      },
      'Failed to log warning message',
      ErrorCategory.IO
    );
  }

  /**
   * Log an error message
   * @param message The message to log
   */
  async logError(message: string): Promise<void> {
    return this.withErrorHandling(
      async () => {
        await error(message);
      },
      'Failed to log error message',
      ErrorCategory.IO
    );
  }

  /**
   * Log a debug message
   * @param message The message to log
   */
  async logDebug(message: string): Promise<void> {
    return this.withErrorHandling(
      async () => {
        await debug(message);
      },
      'Failed to log debug message',
      ErrorCategory.IO
    );
  }

  /**
   * Log a trace message
   * @param message The message to log
   */
  async logTrace(message: string): Promise<void> {
    return this.withErrorHandling(
      async () => {
        await trace(message);
      },
      'Failed to log trace message',
      ErrorCategory.IO
    );
  }
}

// Create singleton instance
export const tauriLogService = new TauriLogService();
