import { invoke } from '@tauri-apps/api/core';
import { BaseService } from './baseService';
import { ProcessingOptions } from '../types/video.types';
import { Preset } from '../types/preset.types';
import { ErrorCategory } from '../utils';

// Preset service for managing conversion presets

class PresetService extends BaseService {
  /**
   * Get list of all presets
   */
  async listPresets(): Promise<Preset[]> {
    const result = await this.withErrorHandling(
      async () => {
        return await invoke<Preset[]>('list_presets');
      },
      'Failed to get preset list',
      ErrorCategory.Preset
    );
    return result || [];
  }

  /**
   * Get preset information by name
   */
  async getPreset(name: string): Promise<Preset | null> {
    return this.withErrorHandling(
      async () => {
        return await invoke<Preset>('get_preset', { id: name });
      },
      `Failed to get preset "${name}"`,
      ErrorCategory.Preset
    );
  }

  /**
   * Get preset options by name
   */
  async getPresetOptions(name: string): Promise<ProcessingOptions | null> {
    const preset = await this.getPreset(name);
    return preset ? preset.options : null;
  }

  /**
   * Save a new preset or update an existing one
   */
  async savePreset(preset: Preset): Promise<boolean> {
    const result = await this.withErrorHandling(
      async () => {
        await invoke<void>('save_preset', { preset });
        return true;
      },
      `Failed to save preset "${preset.name}"`,
      ErrorCategory.Preset
    );
    return result === null ? false : result;
  }

  /**
   * Create a new preset from current options
   */
  async createPreset(
    name: string,
    description: string,
    options: ProcessingOptions
  ): Promise<boolean> {
    const result = await this.withErrorHandling(
      async () => {
        const preset: Preset = {
          name,
          description,
          options
        };

        return await this.savePreset(preset);
      },
      `Failed to create preset "${name}"`,
      ErrorCategory.Preset
    );
    return result === null ? false : result;
  }

  /**
   * Delete a preset by name
   */
  async deletePreset(name: string): Promise<boolean> {
    const result = await this.withErrorHandling(
      async () => {
        await invoke<void>('delete_preset', { name });
        return true;
      },
      `Failed to delete preset "${name}"`,
      ErrorCategory.Preset
    );
    return result === null ? false : result;
  }

  /**
   * Create default presets if they don't exist
   */
  async createDefaultPresets(): Promise<boolean> {
    const result = await this.withErrorHandling(
      async () => {
        await invoke<void>('create_default_presets');
        return true;
      },
      'Failed to create default presets',
      ErrorCategory.Preset
    );
    return result === null ? false : result;
  }

  /**
   * Convert resolution from string to array [width, height]
   */
  resolutionToArray(resolution: string): [number, number] | undefined {
    switch (resolution) {
      case '480p': return [854, 480];
      case '720p': return [1280, 720];
      case '1080p': return [1920, 1080];
      case '4k': return [3840, 2160];
      default: return undefined;
    }
  }

  /**
   * Convert resolution from array [width, height] to string
   */
  resolutionToString(_width: number, height: number): string {
    if (height === 480) return '480p';
    if (height === 720) return '720p';
    if (height === 1080) return '1080p';
    if (height === 2160) return '4k';
    return 'original';
  }
}

// Export singleton instance
export default new PresetService();