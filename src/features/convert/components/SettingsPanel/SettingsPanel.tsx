import React from 'react';
import { Dropdown } from 'primereact/dropdown';
import { InputText } from 'primereact/inputtext';
import { Slider } from 'primereact/slider';
import { SelectButton } from 'primereact/selectbutton';
import { Button } from 'primereact/button';
import { ProgressBar } from 'primereact/progressbar';
import { Preset } from '../../../../types';
import { Container, AdvancedOptions, ActionsContainer, ProgressContainer } from './SettingsPanel.styles';

interface SettingsPanelProps {
  presets: Preset[];
  selectedPreset: string;
  outputFormat: string;
  outputPath: string;
  resolution: string;
  bitrate: number;
  framerate: number;
  use_gpu: boolean;
  isConverting: boolean;
  conversionProgress: number;
  showAdvanced: boolean;
  onPresetChange: (presetName: string) => void;
  onOutputFormatChange: (format: string) => void;
  onOutputPathChange: (path: string) => void;
  onResolutionChange: (resolution: string) => void;
  onBitrateChange: (bitrate: number) => void;
  onFramerateChange: (framerate: number) => void;
  onUseGpuChange: (useGpu: boolean) => void;
  onToggleAdvanced: () => void;
  onSavePreset: () => void;
  onStartConversion: () => void;
  onBrowseOutput: () => void;
  className?: string;
}

export const SettingsPanel: React.FC<SettingsPanelProps> = ({
  presets,
  selectedPreset,
  outputFormat,
  outputPath,
  resolution,
  bitrate,
  framerate,
  use_gpu,
  isConverting,
  conversionProgress,
  showAdvanced,
  onPresetChange,
  onOutputFormatChange,
  onOutputPathChange,
  onResolutionChange,
  onBitrateChange,
  onFramerateChange,
  onUseGpuChange,
  onToggleAdvanced,
  onSavePreset,
  onStartConversion,
  onBrowseOutput,
  className
}) => {
  // Format options
  const formatOptions = [
    { label: 'MP4', value: 'mp4' },
    { label: 'MKV', value: 'mkv' },
    { label: 'WebM', value: 'webm' },
    { label: 'AVI', value: 'avi' }
  ];

  // Resolution options
  const resolutionOptions = [
    { label: 'Original', value: 'original' },
    { label: '4K (3840x2160)', value: '3840x2160' },
    { label: '1440p (2560x1440)', value: '2560x1440' },
    { label: '1080p (1920x1080)', value: '1920x1080' },
    { label: '720p (1280x720)', value: '1280x720' },
    { label: '480p (854x480)', value: '854x480' },
    { label: '360p (640x360)', value: '640x360' }
  ];

  // GPU options
  const gpuOptions = [
    { label: 'Yes', value: true },
    { label: 'No', value: false }
  ];

  // Preset options
  const presetOptions = presets.map(preset => ({
    label: preset.name,
    value: preset.name
  }));

  return (
    <Container className={className}>
      <h3>Conversion Options</h3>

      <div className="p-field">
        <label htmlFor="preset">Preset</label>
        <div className="p-inputgroup">
          <Dropdown
            id="preset"
            value={selectedPreset}
            options={presetOptions}
            onChange={e => onPresetChange(e.value)}
            placeholder="Select a preset"
            className="w-full"
          />
          <Button
            icon="pi pi-save"
            onClick={onSavePreset}
            disabled={isConverting}
            tooltip="Save current settings as preset"
            tooltipOptions={{ position: 'top' }}
          />
        </div>
      </div>

      <div className="p-field">
        <label htmlFor="outputFormat">Output Format</label>
        <Dropdown
          id="outputFormat"
          value={outputFormat}
          options={formatOptions}
          onChange={e => onOutputFormatChange(e.value)}
          placeholder="Select a format"
          className="w-full"
          disabled={isConverting}
        />
      </div>

      <div className="p-field">
        <label htmlFor="outputPath">Output Location</label>
        <div className="p-grid p-nogutter">
          {/* Directory selection */}
          <div className="p-col-12 p-mb-2">
            <div className="p-inputgroup">
              <span className="p-inputgroup-addon">Directory:</span>
              <InputText
                id="outputDirectory"
                value={outputPath.substring(0, outputPath.lastIndexOf('/') + 1) || outputPath}
                placeholder="Select output directory"
                className="w-full"
                disabled={true}
              />
              <Button
                icon="pi pi-folder-open"
                onClick={onBrowseOutput}
                disabled={isConverting}
                tooltip="Select output directory"
                tooltipOptions={{ position: 'top' }}
              />
            </div>
          </div>

          {/* Filename editing */}
          <div className="p-col-12">
            <div className="p-inputgroup">
              <span className="p-inputgroup-addon">Filename:</span>
              <InputText
                id="outputFilename"
                value={outputPath.substring(outputPath.lastIndexOf('/') + 1) || ''}
                onChange={e => {
                  const dirPath = outputPath.substring(0, outputPath.lastIndexOf('/') + 1) || outputPath;
                  onOutputPathChange(dirPath + e.target.value);
                }}
                placeholder="Enter filename (will be auto-generated if empty)"
                className="w-full"
                disabled={isConverting || !outputPath.includes('/')}
              />
              <span className="p-inputgroup-addon">.{outputFormat}</span>
            </div>
          </div>
        </div>
      </div>

      <div className="p-field">
        <label htmlFor="resolution">Resolution</label>
        <Dropdown
          id="resolution"
          value={resolution}
          options={resolutionOptions}
          onChange={e => onResolutionChange(e.value)}
          placeholder="Select a resolution"
          className="w-full"
          disabled={isConverting}
        />
      </div>

      <Button
        label={showAdvanced ? "Hide Advanced Options" : "Show Advanced Options"}
        icon={showAdvanced ? "pi pi-chevron-up" : "pi pi-chevron-down"}
        className="p-button-text p-button-sm mb-3"
        onClick={onToggleAdvanced}
        disabled={isConverting}
      />

      {showAdvanced && (
        <AdvancedOptions>
          <div className="p-field">
            <label htmlFor="bitrate">Bitrate: {bitrate} Kbps</label>
            <Slider
              id="bitrate"
              value={bitrate}
              onChange={e => onBitrateChange(e.value as number)}
              min={500}
              max={10000}
              step={500}
              disabled={isConverting}
            />
          </div>

          <div className="p-field">
            <label htmlFor="framerate">Framerate: {framerate} FPS</label>
            <Slider
              id="framerate"
              value={framerate}
              onChange={e => onFramerateChange(e.value as number)}
              min={15}
              max={60}
              step={5}
              disabled={isConverting}
            />
          </div>

          <div className="p-field">
            <label htmlFor="use_gpu">Use GPU</label>
            <SelectButton
              id="use_gpu"
              value={use_gpu}
              options={gpuOptions}
              onChange={e => onUseGpuChange(e.value)}
              disabled={isConverting}
            />
          </div>
        </AdvancedOptions>
      )}

      <ActionsContainer>
        <Button
          label="Start Conversion"
          icon="pi pi-play"
          onClick={onStartConversion}
          disabled={isConverting}
        />
      </ActionsContainer>

      {isConverting && (
        <ProgressContainer>
          <h4>Converting... {conversionProgress}%</h4>
          <ProgressBar value={conversionProgress} />
        </ProgressContainer>
      )}
    </Container>
  );
};

export default SettingsPanel;
