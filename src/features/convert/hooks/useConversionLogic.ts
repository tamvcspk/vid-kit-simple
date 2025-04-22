import { useState } from 'react';
import { videoService } from '../../../services';
import { useError } from '../../../hooks';
import { ProcessingOptions } from '../../../types';
import { ErrorCategory } from '../../../utils';
import { useTasksStore, usePresetsStore } from '../../../store';

export interface ConversionOptions {
  outputFormat: string;
  resolution: string;
  bitrate: number;
  fps: string;
  use_gpu: boolean;
  outputPath: string;
}

export const useConversionLogic = () => {
  const [isConverting, setIsConverting] = useState<boolean>(false);
  const [showSuccessDialog, setShowSuccessDialog] = useState<boolean>(false);
  const [outputPath, setOutputPath] = useState<string>('');
  const { error, setError, clearError } = useError();

  // Use the new task store
  const { addTask } = useTasksStore();

  // Start conversion
  const startConversion = async (options: ConversionOptions, files: any[]) => {
    setIsConverting(true);
    clearError(); // Use clearError instead of setError(null)

    try {
      // 1. Get the selected file ID
      const selectedFileId = files.find(f => f.selected)?.id;

      if (!selectedFileId) {
        setError({ message: 'Please select a file first', category: ErrorCategory.Validation, timestamp: new Date() });
        setIsConverting(false);
        return;
      }

      // 2. Find the full file object based on ID
      const fileToConvert = files.find(f => f.id === selectedFileId);

      if (!fileToConvert || !fileToConvert.path) {
        setError({ message: 'Selected file path not found. Please try adding the file again.', category: ErrorCategory.Validation, timestamp: new Date() });
        setIsConverting(false);
        return;
      }

      // 3. Get the actual input file path
      const inputFilePath = fileToConvert.path;

      // --- Handle output path ---
      // 4. Use the outputPath value from state
      let finalOutputPath: string = options.outputPath || '';

      // 5. If outputPath is not set, generate a default path
      if (!finalOutputPath) {
        console.log("Output path not set, generating default...");
        const generatedPath = await videoService.generateOutputPath(inputFilePath, options.outputFormat);
        if (generatedPath) {
          finalOutputPath = generatedPath;
          console.log("Generated default output path:", finalOutputPath);
        }

        // If we still can't generate a default path -> error
        if (!finalOutputPath) {
          setError({ message: 'Could not generate default output path', category: ErrorCategory.Task, timestamp: new Date() });
          setIsConverting(false);
          return;
        }
        // Update the form state on UI
        setOutputPath(finalOutputPath);
      }
      // --- End output path handling ---

      // 6. Ensure the output path has the appropriate extension for the output format
      if (!finalOutputPath.toLowerCase().endsWith(`.${options.outputFormat.toLowerCase()}`)) {
        // If the path doesn't end with the appropriate extension, add it
        finalOutputPath = `${finalOutputPath}.${options.outputFormat}`;
        console.log("Added extension to output path:", finalOutputPath);
      }

      // Build the options object for the backend
      const processingOptions: ProcessingOptions = {
        output_format: options.outputFormat,
        output_path: finalOutputPath,
        resolution: options.resolution !== 'original' ? parseResolution(options.resolution) : undefined,
        bitrate: options.bitrate ? options.bitrate * 1000 : undefined, // Assuming backend needs bps
        framerate: options.fps !== 'original' ? parseFloat(options.fps) : undefined,
        use_gpu: options.use_gpu,
        gpu_codec: undefined, // Will be set below if use_gpu is true
        cpu_codec: undefined, // Will be set below if use_gpu is false
      };

      // 7. Determine codec based on GPU/CPU choice
      if (processingOptions.use_gpu) {
        processingOptions.gpu_codec = await videoService.getGpuCodec(processingOptions.output_format);
        console.log("Using GPU codec:", processingOptions.gpu_codec);
      } else {
        processingOptions.cpu_codec = videoService.getCpuCodec(processingOptions.output_format);
        console.log("Using CPU codec:", processingOptions.cpu_codec);
      }

      // 8. Call the service to create a task in the backend with the correct path and options
      console.log("Creating conversion task with input:", inputFilePath, "and options:", processingOptions);

      // Create a task using the new task store
      const taskId = await addTask({
        input_path: inputFilePath,
        output_path: finalOutputPath,
        config: convertOptionsToConfig(processingOptions),
        type: 'convert'
      });

      if (taskId) {
        console.log("Task created with ID:", taskId);

        // Start the task
        await videoService.startTask(taskId);

        // Show success dialog when task is complete
        // This will be handled by the task store events
        setShowSuccessDialog(true);
        setIsConverting(false);
      } else {
        console.error("Failed to create conversion task.");
        setError({ message: 'Cannot create conversion task', category: ErrorCategory.Task, timestamp: new Date() });
        setIsConverting(false);
      }
    } catch (error: any) {
      console.error('Error during conversion process:', error); // Log lỗi chi tiết hơn
      const errorMessage = error.message || 'An unknown error occurred during conversion.';
      setError({ message: errorMessage, category: ErrorCategory.Task, timestamp: new Date() });
      setIsConverting(false);
    }
  };

  // Load video information and update file object
  const loadVideoInfo = async (filePath: string) => {
    // Skip if path is empty or invalid
    if (!filePath || filePath.trim() === '') {
      console.log('Skipping video info loading for empty path');
      return null;
    }

    try {
      return await videoService.getVideoInfo(filePath);
    } catch (err) {
      console.error('Error loading video information:', err);
      setError({ message: 'Error loading video information', category: ErrorCategory.Task, timestamp: new Date() });
      return null;
    }
  };

  // Helper function to parse resolution string to [width, height]
  const parseResolution = (resolution: string): [number, number] | undefined => {
    if (resolution === 'original') return undefined;

    // Handle preset resolutions
    switch (resolution) {
      case '4K':
        return [3840, 2160];
      case '1080p':
        return [1920, 1080];
      case '720p':
        return [1280, 720];
      case '480p':
        return [854, 480];
      case '360p':
        return [640, 360];
      default:
        // Try to parse custom resolution (format: "1920x1080")
        const match = resolution.match(/(\d+)x(\d+)/);
        if (match) {
          return [parseInt(match[1]), parseInt(match[2])];
        }
        return undefined;
    }
  };

  // Helper function to convert ProcessingOptions to config map
  const convertOptionsToConfig = (options: ProcessingOptions): Record<string, string> => {
    const config: Record<string, string> = {
      output_format: options.output_format,
      output_path: options.output_path,
      use_gpu: options.use_gpu ? 'true' : 'false',
    };

    if (options.resolution) {
      config.width = options.resolution[0].toString();
      config.height = options.resolution[1].toString();
    }

    if (options.bitrate) {
      config.bitrate = options.bitrate.toString();
    }

    if (options.framerate) {
      config.framerate = options.framerate.toString();
    }

    if (options.gpu_codec) {
      config.gpu_codec = options.gpu_codec;
    }

    if (options.cpu_codec) {
      config.cpu_codec = options.cpu_codec;
    }

    return config;
  };

  return {
    isConverting,
    showSuccessDialog,
    setShowSuccessDialog,
    outputPath,
    setOutputPath,
    startConversion,
    loadVideoInfo,
    error
  };
};
