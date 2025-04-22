import { BaseService } from './baseService';
import { ProcessingOptions } from '../types/video.types';
import { Preset } from '../types/preset.types';
import { ErrorCategory } from '../utils';
import { usePresetsStore } from '../store/presets.store';
import { ConversionPreset } from '../types/store.types';

// Preset service for managing conversion presets

class PresetService extends BaseService {
  /**
   * Get list of all presets
   */
  async listPresets(): Promise<Preset[]> {
    return this.withErrorHandling(
      async () => {
        // Load presets from store
        await usePresetsStore.getState().loadPresets();
        return usePresetsStore.getState().presets as unknown as Preset[];
      },
      'Failed to get preset list',
      ErrorCategory.Preset
    ) || [];
  }

  /**
   * Get preset information by name
   */
  async getPreset(name: string): Promise<Preset | null> {
    return this.withErrorHandling(
      async () => {
        // Load presets if not loaded
        if (usePresetsStore.getState().presets.length === 0) {
          await usePresetsStore.getState().loadPresets();
        }
        // Find preset by name
        const preset = usePresetsStore.getState().presets.find(p => p.name === name);
        return preset as unknown as Preset;
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
        // Convert Preset to ConversionPreset
        const conversionPreset: ConversionPreset = {
          id: preset.name, // Use name as ID for backward compatibility
          name: preset.name,
          description: preset.description,
          output_format: preset.options.outputFormat,
          resolution: this.convertResolution(preset.options),
          bitrate: preset.options.bitrate,
          fps: preset.options.framerate ? parseInt(preset.options.framerate) : undefined,
          codec: preset.options.codec,
          use_gpu: preset.options.use_gpu,
          audio_codec: 'aac', // Default
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        };

        // Save to store
        await usePresetsStore.getState().savePreset(conversionPreset);
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
        // Find preset by name
        const preset = usePresetsStore.getState().presets.find(p => p.name === name);
        if (!preset) return false;

        // Delete from store
        await usePresetsStore.getState().deletePreset(preset.id);
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
        await usePresetsStore.getState().createDefaultPresets();
        return true;
      },
      'Failed to create default presets',
      ErrorCategory.Preset
    );
    return result === null ? false : result;
  }

  /**
   * Convert ProcessingOptions resolution to ConversionPreset resolution
   */
  private convertResolution(options: ProcessingOptions): any {
    if (!options.resolution || options.resolution === 'original') {
      return { type: 'original' };
    }

    const [width, height] = this.resolutionToArray(options.resolution) || [1280, 720];
    return { type: 'preset', width, height };
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