import React, { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Button } from 'primereact/button';
import { Dialog } from 'primereact/dialog';
import { Message } from 'primereact/message';
import { InputText } from 'primereact/inputtext';
// import { v4 as uuidv4 } from 'uuid';

import { useError } from '../../hooks';
import { presetService } from '../../services';
import { videoService } from '../../services';
import { Preset, ProcessingOptions } from '../../types';
import { formatErrorForUser, ErrorCategory } from '../../utils/errorUtils';
import { SettingsPanel, FileListItem, FileItemData } from './components';

// Import styled components
import {
  Container,
  TwoColumnLayout,
  FileListPanel,
  FileListHeader,
  FileListContainer,
  SettingsPanelContainer,
  DropZone,
  UploadIcon,
  UploadText,
  UploadingContainer,
  SuccessMessage
} from './ConvertView.styles';

// Extend File interface to add path property
declare global {
  interface File {
    path?: string;
  }
}

const ConvertView: React.FC = () => {
  // State for files
  const [files, setFiles] = useState<FileItemData[]>([]);
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [isDragging, setIsDragging] = useState<boolean>(false);
  const [isUploading, setIsUploading] = useState<boolean>(false);

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

  // These options are now handled by the SettingsPanel component

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
    setIsUploading(true);

    const droppedFiles = Array.from(e.dataTransfer.files);
    const videoFiles = droppedFiles.filter(
      file =>
        file.type.startsWith('video/') ||
        ['.mp4', '.mkv', '.avi', '.webm', '.mov'].some(ext => file.name.toLowerCase().endsWith(ext))
    );

    if (videoFiles.length === 0) {
      setError({ message: 'Please select a valid video file', category: ErrorCategory.Validation, timestamp: new Date() });
      setIsUploading(false);
      return;
    }

    // Add files to the list
    const newFiles = videoFiles.map(file => ({
      id: crypto.randomUUID(),
      name: file.name,
      path: file.path || '',
      size: file.size,
      type: file.type || 'video/mp4',
    }));

    setFiles(prevFiles => [...prevFiles, ...newFiles]);

    // Select the first file if none is selected
    if (!selectedFile && newFiles.length > 0) {
      const firstFile = newFiles[0];
      setSelectedFile(firstFile.path);
      await loadVideoInfo(firstFile.path);
    }

    setIsUploading(false);
  };

  // Add file to list
  const addFileToList = async (filePath: string, fileName: string, fileSize: number, fileType: string) => {
    const newFile = {
      id: crypto.randomUUID(),
      name: fileName,
      path: filePath,
      size: fileSize,
      type: fileType || 'video/mp4',
    };

    setFiles(prevFiles => [...prevFiles, newFile]);
    setSelectedFile(filePath);
    await loadVideoInfo(filePath);
  };

  // Handle file selection from list
  const handleFileSelect = (file: FileItemData) => {
    setSelectedFile(file.path);
    loadVideoInfo(file.path);
  };

  // Handle file selection using native dialog
  const handleSelectFile = async () => {
    try {
      setIsUploading(true);
      setError(null);

      // Use videoService to open native file dialog
      const filePath = await videoService.selectVideoFile();

      if (!filePath) {
        // User canceled
        setIsUploading(false);
        return;
      }

      // Extract file name from path
      const fileName = filePath.split(/[\\\/]/).pop() || 'video';

      // Add file to the list
      // Note: We don't have size and type information from native dialog
      // so we'll use placeholder values
      await addFileToList(filePath, fileName, 0, 'video/mp4');

      setIsUploading(false);
    } catch (error) {
      console.error('Error selecting file:', error);
      setError({ message: 'Error selecting file', category: ErrorCategory.IO, timestamp: new Date() });
      setIsUploading(false);
    }
  };



  // Remove file from list
  const handleFileRemove = (file: FileItemData) => {
    setFiles(prevFiles => prevFiles.filter(f => f.id !== file.id));

    // If the removed file was selected, clear selection or select another file
    if (selectedFile === file.path) {
      const remainingFiles = files.filter(f => f.id !== file.id);
      if (remainingFiles.length > 0) {
        handleFileSelect(remainingFiles[0]);
      } else {
        setSelectedFile(null);
      }
    }
  };

  // Load video information and update file object
  const loadVideoInfo = async (filePath: string) => {
    setIsUploading(true);
    setError(null);

    try {
      const info = await videoService.getVideoInfo(filePath);
      if (info) {
        // Update the file object with video info
        const fileIndex = files.findIndex(f => f.path === filePath);
        if (fileIndex >= 0) {
          const updatedFiles = [...files];
          updatedFiles[fileIndex] = {
            ...updatedFiles[fileIndex],
            videoInfo: info
          };
          setFiles(updatedFiles);
        }
        return info;
      } else {
        setError({ message: 'Unable to load video information', category: ErrorCategory.Validation, timestamp: new Date() });
        return null;
      }
    } catch (err) {
      console.error('Error loading video information:', err);
      setError({ message: 'Error loading video information', category: ErrorCategory.Task, timestamp: new Date() });
      return null;
    } finally {
      setIsUploading(false);
    }
  };

  // Apply preset
  const applyPreset = async (presetName: string) => {
    try {
      const presetOptions = await presetService.getPresetOptions(presetName);
      if (presetOptions) {
        setOutputFormat(presetOptions.output_format || 'mp4');

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
        output_format: outputFormat,
        output_path: outputPath,
        use_gpu: use_gpu,
      };

      // Ensure output path is set
      if (!outputPath) {
        setError({ message: 'Please select an output directory', category: ErrorCategory.Validation, timestamp: new Date() });
        setIsConverting(false);
        return;
      }

      // Add file extension to the output path
      options.output_path = `${outputPath}.${outputFormat}`;

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
                setOutputPath(options.output_path);
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

  // This function is no longer needed as we're using the VideoInfoCard component directly

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

      <TwoColumnLayout>
        {/* Left column - File list */}
        <FileListPanel
          ref={dropZoneRef}
          onDragOver={handleDragOver}
          onDragLeave={handleDragLeave}
          onDrop={handleDrop}
        >
          <FileListHeader>
            <h3>Video Files</h3>
            <Button
              label="Add Files"
              icon="pi pi-plus"
              className="p-button-sm"
              onClick={handleSelectFile}
            />
          </FileListHeader>

          {/* File list or drop zone */}
          {files.length > 0 ? (
            <FileListContainer>
              {files.map(file => (
                <FileListItem
                  key={file.id}
                  file={file}
                  isSelected={selectedFile === file.path}
                  onSelect={handleFileSelect}
                  onRemove={handleFileRemove}
                />
              ))}
            </FileListContainer>
          ) : (
            <DropZone
              isDragging={isDragging}
              hasFile={false}
              onClick={handleSelectFile}
            >
              {isUploading ? (
                <UploadingContainer>
                  <UploadIcon className="pi pi-spin pi-spinner"></UploadIcon>
                  <UploadText>Loading file...</UploadText>
                </UploadingContainer>
              ) : (
                <>
                  <UploadIcon className="pi pi-cloud-upload"></UploadIcon>
                  <UploadText>Drag and drop video files here or click to select</UploadText>
                </>
              )}
            </DropZone>
          )}
        </FileListPanel>

        {/* Right column - Settings panel */}
        <SettingsPanelContainer>
          {/* Error message */}
          {error && (
            <Message
              severity="error"
              text={formatErrorForUser(error)}
              style={{ width: '100%', marginBottom: '1rem' }}
            />
          )}

          {/* Conversion options form */}
          {selectedFile && (
            <SettingsPanel
              presets={availablePresets}
              selectedPreset={selectedPreset}
              outputFormat={outputFormat}
              outputPath={outputPath || ''}
              resolution={resolution}
              bitrate={bitrate}
              framerate={parseInt(fps) || 30}
              use_gpu={use_gpu}
              isConverting={isConverting}
              conversionProgress={progress}
              showAdvanced={showAdvanced}
              onPresetChange={(presetName) => {
                setSelectedPreset(presetName);
                applyPreset(presetName);
              }}
              onOutputFormatChange={(format) => setOutputFormat(format)}
              onOutputPathChange={(path) => setOutputPath(path)}
              onResolutionChange={(res) => setResolution(res)}
              onBitrateChange={(rate) => setBitrate(rate)}
              onFramerateChange={(rate) => setFps(rate.toString())}
              onUseGpuChange={(useGpu) => setUseGpu(useGpu)}
              onToggleAdvanced={() => setShowAdvanced(!showAdvanced)}
              onSavePreset={() => setShowSavePresetDialog(true)}
              onStartConversion={startConversion}
              onBrowseOutput={async () => {
                try {
                  // Chọn thư mục đầu ra
                  const selectedDir = await videoService.selectDirectory();
                  if (selectedDir) {
                    // Tạo tên file mặc định dựa trên tên file đầu vào
                    let defaultFileName = 'output_converted';

                    // Nếu đã chọn file đầu vào, sử dụng tên của nó làm cơ sở
                    if (selectedFile) {
                      const fileName = selectedFile.split(/[\\\/]/).pop() || '';
                      const fileNameWithoutExt = fileName.split('.')[0];
                      if (fileNameWithoutExt) {
                        defaultFileName = `${fileNameWithoutExt}_converted`;
                      }
                    }

                    // Tạo đường dẫn đầy đủ với tên file mặc định
                    // Không bao gồm phần mở rộng, vì nó sẽ được thêm vào sau
                    const fullPath = `${selectedDir}/${defaultFileName}`;
                    setOutputPath(fullPath);
                  }
                } catch (error) {
                  console.error('Error selecting output directory:', error);
                  setError({ message: 'Error selecting output directory', category: ErrorCategory.IO, timestamp: new Date() });
                }
              }}
            />
          )}
        </SettingsPanelContainer>
      </TwoColumnLayout>

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
