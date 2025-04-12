import React, { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Button } from 'primereact/button';
import { Dropdown } from 'primereact/dropdown';
import { InputText } from 'primereact/inputtext';
import { Slider } from 'primereact/slider';
import { ProgressBar } from 'primereact/progressbar';
import { SelectButton } from 'primereact/selectbutton';
import { Dialog } from 'primereact/dialog';
import { Message } from 'primereact/message';
import { Divider } from 'primereact/divider';

import { useError } from '../../hooks';
import { presetService } from '../../services';
import { videoService } from '../../services';
import { Preset, VideoInfo, ProcessingOptions } from '../../types';
import { formatErrorForUser, ErrorCategory } from '../../utils/errorUtils';

// Import styled components
import {
  Container,
  DropZone,
  UploadIcon,
  UploadText,
  FileName,
  BatchIndicator,
  UploadingContainer,
  FileActions,
  VideoInfoCard,
  InfoItem,
  ConversionOptions,
  AdvancedOptions,
  ConversionActions,
  ConversionProgress,
  SuccessMessage
} from './ConvertView.styles';

// Extend File interface to add path property
declare global {
  interface File {
    path?: string;
  }
}

const ConvertView: React.FC = () => {
  // State for video file
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [videoInfo, setVideoInfo] = useState<VideoInfo | null>(null);
  const [isDragging, setIsDragging] = useState<boolean>(false);
  const [isUploading, setIsUploading] = useState<boolean>(false);
  const [batchFiles, setBatchFiles] = useState<string[]>([]);

  // State for conversion options
  const [outputFormat, setOutputFormat] = useState<string>('mp4');
  const [resolution, setResolution] = useState<string>('original');
  const [bitrate, setBitrate] = useState<number>(5000);
  const [fps, setFps] = useState<string>('original');
  const [use_gpu, setUseGpu] = useState<boolean>(true);
  const [showAdvanced, setShowAdvanced] = useState<boolean>(false);
  const [selectedPreset, setSelectedPreset] = useState<string>('default');
  const [availablePresets, setAvailablePresets] = useState<Preset[]>([]);
  const [showSavePresetDialog, setShowSavePresetDialog] = useState<boolean>(false);
  const [newPresetName, setNewPresetName] = useState<string>('');
  const [newPresetDescription, setNewPresetDescription] = useState<string>('');

  // State for conversion process
  const [isConverting, setIsConverting] = useState<boolean>(false);
  const [progress, setProgress] = useState<number>(0);
  const [showSuccessDialog, setShowSuccessDialog] = useState<boolean>(false);
  const [outputPath, setOutputPath] = useState<string>('');

  // Use our custom error hook
  const { error, setError } = useError();

  // Refs
  const dropZoneRef = useRef<HTMLDivElement>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  // Supported formats
  const supportedFormats = [
    { label: 'MP4', value: 'mp4' },
    { label: 'MKV', value: 'mkv' },
    { label: 'AVI', value: 'avi' },
    { label: 'WebM', value: 'webm' },
    { label: 'MOV', value: 'mov' },
  ];

  // Resolution options
  const resolutionOptions = [
    { label: 'Original', value: 'original' },
    { label: '480p', value: '480p' },
    { label: '720p', value: '720p' },
    { label: '1080p', value: '1080p' },
    { label: '4K', value: '4k' },
  ];

  // Framerate options
  const fpsOptions = [
    { label: 'Original', value: 'original' },
    { label: '24 FPS', value: '24' },
    { label: '30 FPS', value: '30' },
    { label: '60 FPS', value: '60' },
  ];

  // GPU acceleration options
  const gpuOptions = [
    { label: 'Yes', value: true },
    { label: 'No', value: false },
  ];

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

  // Handle drag and drop
  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(true);
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);
  };

  const handleDrop = async (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);

    const files = Array.from(e.dataTransfer.files);
    const videoFiles = files.filter(
      file =>
        file.type.startsWith('video/') ||
        ['.mp4', '.mkv', '.avi', '.webm', '.mov'].some(ext => file.name.toLowerCase().endsWith(ext))
    );

    if (videoFiles.length === 0) {
      setError({ message: 'Please select a valid video file', category: ErrorCategory.Validation, timestamp: new Date() });
      return;
    }

    // If there are multiple files, process in batch
    if (videoFiles.length > 1) {
      const filePaths = videoFiles.map(file => file.path || '').filter(Boolean);
      setBatchFiles(filePaths);
      if (filePaths.length > 0) {
        setSelectedFile(filePaths[0]);
        await loadVideoInfo(filePaths[0]);
      }
    } else if (videoFiles[0].path) {
      setSelectedFile(videoFiles[0].path);
      await loadVideoInfo(videoFiles[0].path);
    }
  };

  // Select video file using dialog
  const handleBrowseClick = async () => {
    const selectedPath = await videoService.selectVideoFile();
    if (selectedPath) {
      setSelectedFile(selectedPath);
      await loadVideoInfo(selectedPath);
    }
  };

  // Load video information
  const loadVideoInfo = async (filePath: string) => {
    setIsUploading(true);
    setError(null);

    try {
      const info = await videoService.getVideoInfo(filePath);
      console.log(info);
      if (info) {
        setVideoInfo(info);
      } else {
        setError({ message: 'Unable to load video information', category: ErrorCategory.Validation, timestamp: new Date() });
      }
    } catch (err) {
      console.error('Error loading video information:', err);
      setError({ message: 'Error loading video information', category: ErrorCategory.Task, timestamp: new Date() });
    } finally {
      setIsUploading(false);
    }
  };

  // Apply preset
  const applyPreset = async (presetName: string) => {
    try {
      const presetOptions = await presetService.getPresetOptions(presetName);
      if (presetOptions) {
        setOutputFormat(presetOptions.outputFormat || 'mp4');

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

        setUseGpu(presetOptions.use_gpu);
      }
    } catch (error) {
      console.error('Error applying preset:', error);
    }
  };

  // Save current preset
  const saveCurrentPreset = async () => {
    if (!newPresetName.trim()) {
      setError({ message: 'Please enter a preset name', category: ErrorCategory.Validation, timestamp: new Date() });
      return;
    }

    try {
      // Create options object from current settings
      const options: ProcessingOptions = {
        outputFormat: outputFormat,
        outputPath: '', // Will be generated automatically when converting
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

  // Start conversion
  const startConversion = async () => {
    if (!selectedFile) {
      setError({ message: 'Please select a video file before converting', category: ErrorCategory.Validation, timestamp: new Date() });
      return;
    }

    setIsConverting(true);
    setProgress(0);
    setError(null);

    try {
      // Prepare conversion options
      const options: ProcessingOptions = {
        outputFormat: outputFormat,
        outputPath: '', // Will be generated automatically
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

      // Set codec
      if (use_gpu) {
        options.gpu_codec = await videoService.getGpuCodec(outputFormat);
      } else {
        // Get CPU codec suitable for the format
        options.cpu_codec = videoService.getCpuCodec(outputFormat);
      }

      // Create and run conversion task
      const taskId = await videoService.createConversionTask(selectedFile, options);
      if (taskId) {
        const success = await videoService.startConversion(taskId);
        if (success) {
          // Simulate progress updates (in reality would come from backend)
          const interval = setInterval(() => {
            setProgress(prev => {
              if (prev >= 100) {
                clearInterval(interval);
                setIsConverting(false);
                setShowSuccessDialog(true);
                setOutputPath(options.outputPath);
                return 100;
              }
              return prev + 5;
            });
          }, 500);
        } else {
          setError({ message: 'Cannot start conversion process', category: ErrorCategory.Task, timestamp: new Date() });
          setIsConverting(false);
        }
      } else {
        setError({ message: 'Cannot create conversion task', category: ErrorCategory.Task, timestamp: new Date() });
        setIsConverting(false);
      }
    } catch (error) {
      console.error('Error converting video:', error);
      setError({ message: 'Error converting video', category: ErrorCategory.Task, timestamp: new Date() });
      setIsConverting(false);
    }
  };

  // Display video information
  const renderVideoInfo = () => {
    if (!videoInfo) return null;

    return (
      <VideoInfoCard>
        <h3>Video Information</h3>
        <div className="p-grid">
          <div className="p-col-6">
            <InfoItem>
              <label>Format:</label>
              <span>{videoInfo.format}</span>
            </InfoItem>
            <InfoItem>
              <label>Resolution:</label>
              <span>
                {videoInfo.width} x {videoInfo.height}
              </span>
            </InfoItem>
            <InfoItem>
              <label>Duration:</label>
              <span>
                {Math.floor(videoInfo.duration / 60)}:
                {Math.floor(videoInfo.duration % 60)
                  .toString()
                  .padStart(2, '0')}
              </span>
            </InfoItem>
          </div>
          <div className="p-col-6">
            <InfoItem>
              <label>Codec:</label>
              <span>{videoInfo.codec}</span>
            </InfoItem>
            <InfoItem>
              <label>Bitrate:</label>
              <span>{Math.round(videoInfo.bitrate / 1000)} Kbps</span>
            </InfoItem>
            <InfoItem>
              <label>Framerate:</label>
              <span>{videoInfo.framerate} FPS</span>
            </InfoItem>
          </div>
        </div>
      </VideoInfoCard>
    );
  };

  // Save preset dialog
  const renderSavePresetDialog = () => {
    return (
      <Dialog
        header="Save New Preset"
        visible={showSavePresetDialog}
        onHide={() => setShowSavePresetDialog(false)}
        style={{ width: '450px' }}
        footer={
          <div>
            <Button
              label="Cancel"
              icon="pi pi-times"
              onClick={() => setShowSavePresetDialog(false)}
              className="p-button-text"
            />
            <Button label="Save" icon="pi pi-save" onClick={saveCurrentPreset} autoFocus />
          </div>
        }
      >
        <div className="p-field">
          <label htmlFor="presetName">Preset Name</label>
          <InputText
            id="presetName"
            value={newPresetName}
            onChange={e => setNewPresetName(e.target.value)}
            placeholder="Enter preset name"
            className="w-full"
            required
            autoFocus
          />
        </div>

        <div className="p-field">
          <label htmlFor="presetDescription">Description</label>
          <InputText
            id="presetDescription"
            value={newPresetDescription}
            onChange={e => setNewPresetDescription(e.target.value)}
            placeholder="Enter description (optional)"
            className="w-full"
          />
        </div>
      </Dialog>
    );
  };

  return (
    <Container>
      <h2>Convert Video</h2>

      {/* Drag and drop area */}
      <DropZone
        ref={dropZoneRef}
        isDragging={isDragging}
        hasFile={!!selectedFile}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
        onClick={() => fileInputRef.current?.click()}
      >
        {isUploading ? (
          <UploadingContainer>
            <UploadIcon className="pi pi-spin pi-spinner"></UploadIcon>
            <UploadText>Loading file...</UploadText>
          </UploadingContainer>
        ) : selectedFile ? (
          <>
            <UploadIcon className="pi pi-video"></UploadIcon>
            <FileName>{selectedFile.split('/').pop()?.split('\\').pop()}</FileName>
            {batchFiles.length > 1 && (
              <BatchIndicator>+{batchFiles.length - 1} other files</BatchIndicator>
            )}
          </>
        ) : (
          <>
            <UploadIcon className="pi pi-cloud-upload"></UploadIcon>
            <UploadText>Drag and drop video files here or click to select</UploadText>
          </>
        )}
      </DropZone>

      <input
        type="file"
        ref={fileInputRef}
        style={{ display: 'none' }}
        accept="video/*"
        onChange={e => {
          const file = e.target.files?.[0];
          if (file && file.path) {
            setSelectedFile(file.path);
            loadVideoInfo(file.path);
          }
        }}
      />

      <FileActions>
        <Button label="Select File" icon="pi pi-folder-open" onClick={handleBrowseClick} />
        {selectedFile && (
          <Button
            label="Delete"
            icon="pi pi-trash"
            className="p-button-danger"
            onClick={() => {
              setSelectedFile(null);
              setVideoInfo(null);
              setBatchFiles([]);
            }}
          />
        )}
      </FileActions>

      {error && (
        <Message
          severity="error"
          text={formatErrorForUser(error)}
          style={{ width: '100%', marginBottom: '1rem' }}
        />
      )}

      {/* Display video information */}
      {videoInfo && renderVideoInfo()}

      {/* Conversion options form */}
      {selectedFile && (
        <ConversionOptions>
          <h3>Conversion Options</h3>

          <div className="p-grid">
            <div className="p-col-12 p-md-6">
              <div className="p-field">
                <label htmlFor="preset">Preset</label>
                <div className="p-inputgroup">
                  <Dropdown
                    id="preset"
                    value={selectedPreset}
                    options={availablePresets.map(p => ({ label: p.name, value: p.name }))}
                    onChange={e => {
                      setSelectedPreset(e.value);
                      applyPreset(e.value);
                    }}
                    placeholder="Select preset"
                    className="w-full"
                  />
                  <Button
                    icon="pi pi-save"
                    tooltip="Save new preset"
                    onClick={() => setShowSavePresetDialog(true)}
                  />
                </div>
              </div>

              <div className="p-field">
                <label htmlFor="outputFormat">Output Format</label>
                <Dropdown
                  id="outputFormat"
                  value={outputFormat}
                  options={supportedFormats}
                  onChange={e => setOutputFormat(e.value)}
                  placeholder="Select format"
                  className="w-full"
                />
              </div>

              <div className="p-field">
                <label htmlFor="resolution">Resolution</label>
                <Dropdown
                  id="resolution"
                  value={resolution}
                  options={resolutionOptions}
                  onChange={e => setResolution(e.value)}
                  placeholder="Select resolution"
                  className="w-full"
                />
              </div>
            </div>

            <div className="p-col-12 p-md-6">
              <div className="p-field">
                <label htmlFor="bitrate">Bitrate: {bitrate} Kbps</label>
                <Slider
                  id="bitrate"
                  value={bitrate}
                  onChange={e => setBitrate(e.value as number)}
                  min={500}
                  max={20000}
                  step={500}
                  className="w-full"
                />
              </div>

              <div className="p-field">
                <label htmlFor="fps">Framerate</label>
                <Dropdown
                  id="fps"
                  value={fps}
                  options={fpsOptions}
                  onChange={e => setFps(e.value)}
                  placeholder="Select framerate"
                  className="w-full"
                />
              </div>

              <div className="p-field">
                <label htmlFor="use_gpu">Use GPU</label>
                <SelectButton
                  id="use_gpu"
                  value={use_gpu}
                  options={gpuOptions}
                  onChange={e => setUseGpu(e.value)}
                  className="w-full"
                />
              </div>
            </div>
          </div>

          <div className="p-field">
            <Button
              label={showAdvanced ? 'Hide advanced options' : 'Show advanced options'}
              icon={`pi ${showAdvanced ? 'pi-chevron-up' : 'pi-chevron-down'}`}
              className="p-button-text"
              onClick={() => setShowAdvanced(!showAdvanced)}
            />
          </div>

          {showAdvanced && (
            <AdvancedOptions>
              <div className="p-field">
                <label>Advanced options will appear here</label>
              </div>
            </AdvancedOptions>
          )}

          <Divider />

          <ConversionActions>
            <Button
              label="Start Conversion"
              icon="pi pi-play"
              className="p-button-success"
              onClick={startConversion}
              disabled={isConverting || !selectedFile}
            />
          </ConversionActions>

          {isConverting && (
            <ConversionProgress>
              <h4>Converting... {progress}%</h4>
              <ProgressBar value={progress} showValue={false} />
            </ConversionProgress>
          )}
        </ConversionOptions>
      )}

      {/* Save preset dialog */}
      {renderSavePresetDialog()}

      {/* Dialog shown when conversion is successful */}
      <Dialog
        header="Conversion Successful"
        visible={showSuccessDialog}
        onHide={() => setShowSuccessDialog(false)}
        style={{ width: '50vw' }}
        footer={
          <div>
            <Button
              label="Close"
              onClick={() => setShowSuccessDialog(false)}
              className="p-button-text"
            />
            <Button
              label="Open Folder"
              icon="pi pi-folder-open"
              onClick={() => {
                // Open folder containing the converted file
                if (outputPath) {
                  const folderPath = outputPath.substring(0, outputPath.lastIndexOf('/'));
                  invoke('plugin:opener|open_item', { path: folderPath });
                }
              }}
            />
          </div>
        }
      >
        <SuccessMessage>
          <i
            className="pi pi-check-circle"
            style={{ fontSize: '2rem', color: 'var(--green-500)' }}
          ></i>
          <p>Video has been successfully converted!</p>
          <p>Path: {outputPath}</p>
        </SuccessMessage>
      </Dialog>
    </Container>
  );
};

export { ConvertView };
