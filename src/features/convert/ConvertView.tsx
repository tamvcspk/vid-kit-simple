import React, { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Button } from 'primereact/button';
import { Dialog } from 'primereact/dialog';
import { Message } from 'primereact/message';
import { InputText } from 'primereact/inputtext';
// import { v4 as uuidv4 } from 'uuid';

import { useError } from '../../hooks';
import { usePreferences } from '../../hooks/usePreferences';
import { useConversionState } from '../../hooks/useConversionState';
import { useAppState } from '../../hooks/useAppState';
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
  // Get global state
  const { appState } = useAppState();
  const { conversionState, addTask, markTaskFailed } = useConversionState();
  const { preferences, updatePreferences } = usePreferences();

  // State for drag and drop
  const [isDragging, setIsDragging] = useState<boolean>(false);
  const [isUploading, setIsUploading] = useState<boolean>(false);

  // Lấy danh sách file và file được chọn từ global state
  const files = conversionState?.files || [];
  const selectedFile = conversionState?.selected_file_id || null;

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
  const [showSuccessDialog, setShowSuccessDialog] = useState<boolean>(false);
  const [outputPath, setOutputPath] = useState<string>('');

  // Use our custom error hook
  const { error, setError } = useError();

  // Refs
  const dropZoneRef = useRef<HTMLDivElement>(null);

  // These options are now handled by the SettingsPanel component

  // Load preset list and initialize settings from preferences when component mounts
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

    // Initialize settings from preferences if available
    if (preferences) {
      setOutputFormat(preferences.default_format || 'mp4');
      setUseGpu(preferences.use_gpu);
    }

    // Check GPU availability from appState
    if (appState) {
      // Only enable GPU if it's available
      if (!appState.gpu_available) {
        setUseGpu(false);
      }
    }
  }, [preferences, appState]);

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

    // Add each file to global state
    for (const file of videoFiles) {
      const id = crypto.randomUUID();
      await invoke('add_file_to_list', {
        id,
        name: file.name,
        path: file.path || '',
        size: file.size,
        fileType: file.type || 'video/mp4',
        duration: null,
        resolution: null,
        thumbnail: null
      });

      // Load video info for the first file
      if (videoFiles[0] === file) {
        await loadVideoInfo(file.path || '');
      }
    }

    setIsUploading(false);
  };

  // Add file to list
  const addFileToList = async (filePath: string, fileName: string, fileSize: number, fileType: string) => {
    const id = crypto.randomUUID();

    // Thêm file vào global state
    await invoke('add_file_to_list', {
      id,
      name: fileName,
      path: filePath,
      size: fileSize,
      fileType: fileType || 'video/mp4',
      duration: null,
      resolution: null,
      thumbnail: null
    });

    // Load thông tin video
    await loadVideoInfo(filePath);
  };

  // Handle file selection from list
  const handleFileSelect = async (file: FileItemData) => {
    await invoke('select_file', { fileId: file.id });
    await loadVideoInfo(file.path);
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
  const handleFileRemove = async (file: FileItemData) => {
    // Xóa file khỏi global state
    await invoke('remove_file_from_list', { fileId: file.id });

    // Nếu file bị xóa là file đang được chọn, global state sẽ tự động chọn file khác
    // hoặc đặt selected_file_id = null nếu không còn file nào
  };

  // Load video information and update file object
  const loadVideoInfo = async (filePath: string) => {
    setIsUploading(true);
    setError(null);

    try {
      const info = await videoService.getVideoInfo(filePath);
      if (info) {
        // Thông tin video đã được load, nhưng chúng ta không cập nhật file object
        // vì chúng ta đang sử dụng global state
        // Có thể cập nhật thông tin video vào global state nếu cần
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
    setIsConverting(true);
    setError(null);

    try {
      // 1. Lấy ID của file đang được chọn từ state
      const selectedFileId = conversionState?.selected_file_id;

      if (!selectedFileId) {
        setError({ message: 'Please select a file first', category: ErrorCategory.Validation, timestamp: new Date() });
        setIsConverting(false);
        return;
      }

      // 2. Tìm đối tượng file đầy đủ dựa trên ID
      const fileToConvert = files.find(f => f.id === selectedFileId);

      if (!fileToConvert || !fileToConvert.path) {
        setError({ message: 'Selected file path not found. Please try adding the file again.', category: ErrorCategory.Validation, timestamp: new Date() });
        setIsConverting(false);
        return;
      }

      // 3. Lấy đường dẫn file đầu vào thực tế
      const inputFilePath = fileToConvert.path;

      // --- Xử lý đường dẫn đầu ra ---
      // 4. Sử dụng giá trị outputPath từ state
      let finalOutputPath: string = outputPath || '';

      // 5. Nếu outputPath chưa có (người dùng chưa chọn hoặc nhập), tạo đường dẫn mặc định
      if (!finalOutputPath) {
        console.log("Output path not set, generating default..."); // Thêm log để debug
        const generatedPath = await videoService.generateOutputPath(inputFilePath, outputFormat);
        if (generatedPath) {
          finalOutputPath = generatedPath;
          console.log("Generated default output path:", finalOutputPath); // Thêm log để debug
        }

        // Nếu vẫn không tạo được đường dẫn mặc định -> lỗi
        if (!finalOutputPath) {
          setError({ message: 'Could not generate default output path', category: ErrorCategory.Task, timestamp: new Date() });
          setIsConverting(false);
          return;
        }
        // Cập nhật state của form trên UI nếu cần (có thể không cần thiết nếu optionsForm tự cập nhật)
        setOutputPath(finalOutputPath);
      }
      // --- Kết thúc xử lý đường dẫn đầu ra ---

      // 6. Đảm bảo đường dẫn đầu ra có phần mở rộng phù hợp với định dạng đầu ra
      if (!finalOutputPath.toLowerCase().endsWith(`.${outputFormat.toLowerCase()}`)) {
        // Nếu đường dẫn không kết thúc bằng phần mở rộng phù hợp, thêm vào
        finalOutputPath = `${finalOutputPath}.${outputFormat}`;
        console.log("Added extension to output path:", finalOutputPath);
      }

      // Xây dựng đối tượng options cho backend
      const options: ProcessingOptions = {
        output_format: outputFormat,
        output_path: finalOutputPath, // <-- Sử dụng biến 'finalOutputPath' đã được xử lý ở trên
        resolution: resolution !== 'original' ? presetService.resolutionToArray(resolution) : undefined,
        bitrate: bitrate ? bitrate * 1000 : undefined, // Giả sử backend cần bps
        framerate: fps !== 'original' ? parseFloat(fps) : undefined,
        use_gpu: use_gpu,
        gpu_codec: undefined, // Sẽ được set bên dưới nếu use_gpu là true
        cpu_codec: undefined, // Sẽ được set bên dưới nếu use_gpu là false
      };

      // 7. Xác định codec dựa trên lựa chọn GPU/CPU
      if (options.use_gpu) {
        options.gpu_codec = await videoService.getGpuCodec(options.output_format);
         console.log("Using GPU codec:", options.gpu_codec); // Thêm log để debug
      } else {
        options.cpu_codec = videoService.getCpuCodec(options.output_format);
         console.log("Using CPU codec:", options.cpu_codec); // Thêm log để debug
      }

      // 8. Gọi service để tạo task ở backend với đường dẫn và options chính xác
       console.log("Creating conversion task with input:", inputFilePath, "and options:", options); // Thêm log để debug
      const taskId = await videoService.createConversionTask(inputFilePath, options);

      // 9. Xử lý kết quả tạo task và bắt đầu chạy task
      if (taskId) {
         console.log("Task created with ID:", taskId); // Thêm log để debug
        await addTask(taskId);
        const success = await videoService.startConversion(taskId);
        if (success) {
           console.log("Conversion started successfully for task:", taskId); // Thêm log để debug
          // Logic xử lý tiến độ... (giữ nguyên)
          const checkProgress = setInterval(() => {
            // Lấy trạng thái chuyển đổi hiện tại từ hook
            const { conversionState: currentConversionState } = useConversionState();
            if (currentConversionState) {
               // Tìm đúng task nếu có nhiều task? Hoặc giả định chỉ có 1 task chạy?
               // Hiện tại đang dựa vào current_progress chung
               const taskProgress = currentConversionState.current_progress; // Hoặc tìm progress của task cụ thể
               console.log(`Task ${taskId} progress: ${taskProgress}`); // Thêm log để debug
               if (taskProgress >= 100) {
                 clearInterval(checkProgress);
                 setIsConverting(false);
                 setShowSuccessDialog(true);
                 setOutputPath(options.output_path); // Có thể không cần thiết nếu đã set ở trên
                 console.log("Conversion complete for task:", taskId); // Thêm log để debug
               }
            }
          }, 1000); // Tăng interval lên 1 giây để giảm log
        } else {
           console.error("Failed to start conversion for task:", taskId); // Thêm log để debug
          markTaskFailed(taskId);
          setError({ message: 'Cannot start conversion process', category: ErrorCategory.Task, timestamp: new Date() });
          setIsConverting(false);
        }
      } else {
         console.error("Failed to create conversion task."); // Thêm log để debug
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
                  isSelected={selectedFile === file.id}
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
              conversionProgress={conversionState ? conversionState.current_progress : 0}
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
                    const selectedFileObj = files.find(f => f.id === selectedFile);
                    if (selectedFileObj && selectedFileObj.name) {
                      const fileNameWithoutExt = selectedFileObj.name.split('.')[0];
                      if (fileNameWithoutExt) {
                        defaultFileName = `${fileNameWithoutExt}_converted`;
                      }
                    }

                    // Tạo đường dẫn đầy đủ với tên file mặc định
                    // Bao gồm phần mở rộng tương ứng với định dạng đầu ra
                    const fullPath = `${selectedDir}/${defaultFileName}.${outputFormat}`;
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
