import React, { useState, useEffect } from 'react';
import { useAppState } from '../../hooks/useAppState';
import { useTasksStore, useFilesStore } from '../../store';
import { usePreferences } from '../../hooks/usePreferences';
import { useError } from '../../hooks';
import { videoService } from '../../services';

// Import custom hooks
import { useConversionLogic, useFileManagement, usePresetManagement } from './hooks';

// Import components
import { FileList } from './components/FileList';
import { ConversionForm } from './components/ConversionForm';
import { PresetDialog } from './components/PresetDialog';
import { SuccessDialog } from './components/SuccessDialog';
import { SettingsPanel } from './components';
import { FileItemData } from './components/FileList/types';

// Import styled components
import { Container, TwoColumnLayout } from './ConvertView.styles';

// Extend File interface to add path property
declare global {
  interface File {
    path?: string;
  }
}

const ConvertView: React.FC = () => {
  // Get global state
  const { appState } = useAppState();
  const { tasks } = useTasksStore();
  const { files: fileList, selectedFileId } = useFilesStore();
  const { preferences } = usePreferences();

  // State for conversion options
  const [outputFormat, setOutputFormat] = useState<string>('mp4');
  const [resolution, setResolution] = useState<string>('original');
  const [bitrate, setBitrate] = useState<number>(5000);
  const [fps, setFps] = useState<string>('original');
  const [use_gpu, setUseGpu] = useState<boolean>(true);
  const [showAdvanced, setShowAdvanced] = useState<boolean>(false);

  // Use custom hooks
  const { error } = useError();

  const {
    isConverting,
    showSuccessDialog,
    setShowSuccessDialog,
    outputPath,
    setOutputPath,
    startConversion,
    loadVideoInfo
  } = useConversionLogic();

  const {
    isDragging,
    isUploading,
    dropZoneRef,
    handleDragOver,
    handleDragLeave,
    handleDrop,
    handleFileSelect,
    handleSelectFile,
    handleFileRemove
  } = useFileManagement(loadVideoInfo);

  const {
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
  } = usePresetManagement();

  // Convert store files to FileItemData[]
  const files: FileItemData[] = fileList.map(file => ({
    id: file.id,
    name: file.name,
    path: file.path,
    size: file.size,
    fileType: file.type, // Map 'type' to 'fileType'
    duration: file.duration,
    resolution: file.resolution ? [file.resolution.width, file.resolution.height] : null,
    thumbnail: file.thumbnail,
    selected: file.selected
  }));

  // Initialize settings from preferences and check GPU availability
  useEffect(() => {
    // Initialize settings from preferences if available
    if (preferences) {
      setOutputFormat(preferences.default_format || 'mp4');
      setUseGpu(preferences.use_gpu);
    }

    // Check GPU availability from appState
    if (appState && !appState.gpu_available) {
      setUseGpu(false);
    }
  }, [appState, preferences]);

  // Handle preset change
  const handlePresetChange = (presetName: string) => {
    setSelectedPreset(presetName);
    applyPreset(
      presetName,
      setOutputFormat,
      setResolution,
      setBitrate,
      setFps,
      setUseGpu
    );
  };

  // Handle save preset
  const handleSavePreset = () => {
    saveCurrentPreset(
      outputFormat,
      resolution,
      bitrate,
      fps,
      use_gpu
    );
  };

  // Handle start conversion
  const handleStartConversion = () => {
    startConversion(
      {
        outputFormat,
        resolution,
        bitrate,
        fps,
        use_gpu,
        outputPath
      },
      files
    );
  };

  // Handle browse output
  const handleBrowseOutput = async () => {
    try {
      // Select output directory
      const selectedDir = await videoService.selectDirectory();
      if (selectedDir) {
        // Create default filename based on input filename
        let defaultFileName = 'output_converted';

        // If input file is selected, use its name as the base
        const selectedFileObj = files.find(f => f.id === selectedFileId);
        if (selectedFileObj && selectedFileObj.name) {
          const fileNameWithoutExt = selectedFileObj.name.split('.')[0];
          if (fileNameWithoutExt) {
            defaultFileName = `${fileNameWithoutExt}_converted`;
          }
        }

        // Create full path with default filename
        // Include extension corresponding to the output format
        const fullPath = `${selectedDir}/${defaultFileName}.${outputFormat}`;
        setOutputPath(fullPath);
      }
    } catch (error) {
      console.error('Error selecting output directory:', error);
    }
  };

  return (
    <Container>
      <h2>Convert Video</h2>

      <TwoColumnLayout>
        {/* Left column - File list */}
        <FileList
          files={files}
          selectedFileId={selectedFileId}
          isDragging={isDragging}
          isUploading={isUploading}
          dropZoneRef={dropZoneRef}
          onDragOver={handleDragOver}
          onDragLeave={handleDragLeave}
          onDrop={handleDrop}
          onFileSelect={handleFileSelect}
          onFileRemove={handleFileRemove}
          onAddFiles={handleSelectFile}
        />

        {/* Right column - Settings panel */}
        <ConversionForm
          error={error}
          selectedFile={selectedFileId}
        >
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
            conversionProgress={0} // TODO: Get progress from task store
            showAdvanced={showAdvanced}
            onPresetChange={handlePresetChange}
            onOutputFormatChange={(format) => setOutputFormat(format)}
            onOutputPathChange={(path) => setOutputPath(path)}
            onResolutionChange={(res) => setResolution(res)}
            onBitrateChange={(rate) => setBitrate(rate)}
            onFramerateChange={(rate) => setFps(rate.toString())}
            onUseGpuChange={(useGpu) => setUseGpu(useGpu)}
            onToggleAdvanced={() => setShowAdvanced(!showAdvanced)}
            onSavePreset={() => setShowSavePresetDialog(true)}
            onStartConversion={handleStartConversion}
            onBrowseOutput={handleBrowseOutput}
          />
        </ConversionForm>
      </TwoColumnLayout>

      {/* Save preset dialog */}
      <PresetDialog
        visible={showSavePresetDialog}
        onHide={() => setShowSavePresetDialog(false)}
        presetName={newPresetName}
        onPresetNameChange={setNewPresetName}
        presetDescription={newPresetDescription}
        onPresetDescriptionChange={setNewPresetDescription}
        onSave={handleSavePreset}
      />

      {/* Success dialog */}
      <SuccessDialog
        visible={showSuccessDialog}
        onHide={() => setShowSuccessDialog(false)}
        outputPath={outputPath}
      />
    </Container>
  );
};

export { ConvertView };
