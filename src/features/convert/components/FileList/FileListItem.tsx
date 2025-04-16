import React, { useState, useEffect } from 'react';
import { Button } from 'primereact/button';
import { Card } from 'primereact/card';
import styled from '@emotion/styled';
import { VideoInfo } from '../../../../types';
import { videoService } from '../../../../services';
import { formatFileSize, formatDuration } from '../../../../utils/formatUtils';
import { FileListItemProps } from './types';

// Styled components
const FileCard = styled(Card)`
  margin-bottom: 0.75rem;
  cursor: pointer;
  transition: all 0.2s ease;

  &.selected {
    border-left: 3px solid var(--primary-color);
    background-color: var(--surface-100);

    &:hover {
      background-color: var(--surface-100);
    }
  }

  &:not(.selected) {
    border-left: 3px solid transparent;
    background-color: var(--surface-card);

    &:hover {
      background-color: var(--surface-hover);
    }
  }

  .p-card-body {
    padding: 0.75rem;
  }

  .p-card-content {
    padding: 0;
  }

  h3 {
    margin-top: 0;
    margin-bottom: 0.5rem;
    color: var(--primary-color);
    font-size: 1.1rem;
  }
`;

const FileHeader = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
`;

const FileName = styled.div`
  font-weight: 500;
  font-size: 1rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
`;

const InfoItem = styled.div`
  margin-bottom: 0.5rem;
  display: flex;

  label {
    font-weight: bold;
    margin-right: 0.5rem;
    min-width: 100px;
  }

  span {
    color: var(--text-color-secondary);
  }
`;

const InfoGrid = styled.div`
  display: flex;
  flex-wrap: wrap;

  .info-column {
    flex: 1;
    min-width: 200px;
  }
`;

export const FileListItem: React.FC<FileListItemProps> = ({
  file,
  isSelected,
  onSelect,
  onRemove
}) => {
  const [videoInfo, setVideoInfo] = useState<VideoInfo | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(true);

  useEffect(() => {
    // Load video info if not already loaded
    loadVideoInfo();
  }, [file.path]);

  const loadVideoInfo = async () => {
    setIsLoading(true);
    try {
      const info = await videoService.getVideoInfo(file.path);
      setVideoInfo(info);
    } catch (error) {
      console.error('Error loading video info:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const getFileExtension = (filename: string) => {
    return filename.split('.').pop()?.toUpperCase() || '';
  };

  return (
    <FileCard
      className={isSelected ? 'selected' : ''}
      onClick={() => onSelect(file)}
    >
      <FileHeader>
        <FileName>{file.name}</FileName>
        <Button
          icon="pi pi-times"
          className="p-button-rounded p-button-text p-button-sm"
          onClick={(e) => {
            e.stopPropagation();
            onRemove(file);
          }}
          aria-label="Remove file"
        />
      </FileHeader>

      {isLoading ? (
        <div style={{ display: 'flex', alignItems: 'center', padding: '1rem 0' }}>
          <i className="pi pi-spin pi-spinner" style={{ marginRight: '0.5rem' }}></i>
          <span>Loading video information...</span>
        </div>
      ) : videoInfo ? (
        <>
          <h3>Video Information</h3>
          <InfoGrid>
            <div className="info-column">
              <InfoItem>
                <label>Format:</label>
                <span>{videoInfo.format}</span>
              </InfoItem>
              <InfoItem>
                <label>Resolution:</label>
                <span>{videoInfo.width} x {videoInfo.height}</span>
              </InfoItem>
              <InfoItem>
                <label>Duration:</label>
                <span>{formatDuration(videoInfo.duration)}</span>
              </InfoItem>
            </div>
            <div className="info-column">
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
          </InfoGrid>
        </>
      ) : (
        <div style={{ padding: '0.5rem 0' }}>
          <InfoItem>
            <label>Size:</label>
            <span>{formatFileSize(file.size)}</span>
          </InfoItem>
          <InfoItem>
            <label>Type:</label>
            <span>{file.fileType || getFileExtension(file.name)}</span>
          </InfoItem>
        </div>
      )}
    </FileCard>
  );
};
