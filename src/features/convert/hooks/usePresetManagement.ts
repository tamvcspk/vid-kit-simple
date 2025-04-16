import { useState, useEffect } from 'react';
import { presetService } from '../../../services';
import { usePreferences } from '../../../hooks/usePreferences';
import { useError } from '../../../hooks';
import { Preset, ProcessingOptions } from '../../../types';
import { ErrorCategory } from '../../../utils';

export const usePresetManagement = () => {
  const [selectedPreset, setSelectedPreset] = useState<string>('default');
  const [availablePresets, setAvailablePresets] = useState<Preset[]>([]);
  const [showSavePresetDialog, setShowSavePresetDialog] = useState<boolean>(false);
  const [newPresetName, setNewPresetName] = useState<string>('');
  const [newPresetDescription, setNewPresetDescription] = useState<string>('');

  const { preferences, updatePreferences } = usePreferences();
  const { setError } = useError();

  // Load preset list when component mounts
  useEffect(() => {
    const loadPresets = async () => {
      try {
        // Ensure default presets are created
        await presetService.createDefaultPresets();

        // Load preset list
        const presets = await presetService.listPresets();
        if (presets.length > 0) {
          setAvailablePresets(presets);
          setSelectedPreset(presets[0].name);
        }
      } catch (error) {
        console.error('Error loading presets:', error);
      }
    };

    loadPresets();
  }, []);

  // Apply preset
  const applyPreset = async (
    presetName: string,
    setOutputFormat: (format: string) => void,
    setResolution: (res: string) => void,
    setBitrate: (rate: number) => void,
    setFps: (fps: string) => void,
    setUseGpu: (useGpu: boolean) => void
  ) => {
    try {
      const presetOptions = await presetService.getPresetOptions(presetName);
      if (presetOptions) {
        const newFormat = presetOptions.output_format || 'mp4';
        setOutputFormat(newFormat);

        if (presetOptions.resolution) {
          // Convert from [width, height] to string option
          const [width, height] = presetOptions.resolution;
          const resolutionString = presetService.resolutionToString(width, height);
          setResolution(resolutionString);
        } else {
          setResolution('original');
        }

        setBitrate(presetOptions.bitrate || 5000);

        if (presetOptions.framerate) {
          setFps(presetOptions.framerate.toString());
        } else {
          setFps('original');
        }

        const newUseGpu = presetOptions.use_gpu;
        setUseGpu(newUseGpu);

        // Save format and GPU settings to preferences
        if (preferences) {
          updatePreferences({
            ...preferences,
            default_format: newFormat,
            use_gpu: newUseGpu
          });
        }
      }
    } catch (error) {
      console.error('Error applying preset:', error);
    }
  };

  // Save current preset
  const saveCurrentPreset = async (
    outputFormat: string,
    resolution: string,
    bitrate: number,
    fps: string,
    use_gpu: boolean
  ) => {
    if (!newPresetName.trim()) {
      setError({ message: 'Please enter a preset name', category: ErrorCategory.Validation, timestamp: new Date() });
      return;
    }

    try {
      // Create options object from current settings
      const options: ProcessingOptions = {
        output_format: outputFormat,
        output_path: '', // Will be generated automatically when converting
        use_gpu: use_gpu,
      };

      // Set resolution
      if (resolution !== 'original') {
        options.resolution = presetService.resolutionToArray(resolution);
      }

      // Set bitrate
      options.bitrate = bitrate * 1000; // Convert from Kbps to bps

      // Set framerate
      if (fps !== 'original') {
        options.framerate = parseFloat(fps);
      }

      // Save preset
      const success = await presetService.createPreset(
        newPresetName,
        newPresetDescription,
        options
      );

      if (success) {
        // Update preset list
        const presets = await presetService.listPresets();
        setAvailablePresets(presets);
        setSelectedPreset(newPresetName);

        // Close dialog
        setShowSavePresetDialog(false);
        setNewPresetName('');
        setNewPresetDescription('');
      } else {
        setError({ message: 'Unable to save preset', category: ErrorCategory.Preset, timestamp: new Date() });
      }
    } catch (error) {
      console.error('Error saving preset:', error);
      setError({ message: 'Error saving preset', category: ErrorCategory.Preset, timestamp: new Date() });
    }
  };

  return {
    selectedPreset,
    setSelectedPreset,
    availablePresets,
    showSavePresetDialog,
    setShowSavePresetDialog,
    newPresetName,
    setNewPresetName,
    newPresetDescription,
    setNewPresetDescription,
    applyPreset,
    saveCurrentPreset
  };
};
