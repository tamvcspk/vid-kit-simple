import styled from '@emotion/styled';

// Main container
export const Container = styled.div`
  height: 100%;
  display: flex;
  flex-direction: column;

  h2 {
    margin-bottom: 1rem;
    color: var(--primary-color);
  }
`;

// Two-column layout container
export const TwoColumnLayout = styled.div`
  display: flex;
  flex: 1;
  gap: 1.5rem;
  overflow: hidden;
  height: 100%;

  // Responsive adjustments
  @media (max-width: 768px) {
    flex-direction: column;
  }
`;

// Left column - File list panel
export const FileListPanel = styled.div`
  flex: 7; // 70% of the space
  display: flex;
  flex-direction: column;
  background-color: var(--surface-50);
  border-radius: 8px;
  padding: 1rem;
  overflow: hidden;
  min-width: 300px;

  @media (max-width: 768px) {
    flex: 1;
  }
`;

// Right column - Settings panel container
export const SettingsPanelContainer = styled.div`
  flex: 3; // 30% of the space
  overflow-y: auto;
  padding-right: 0.5rem;

  &::-webkit-scrollbar {
    width: 8px;
  }

  &::-webkit-scrollbar-track {
    background: var(--surface-100);
    border-radius: 4px;
  }

  &::-webkit-scrollbar-thumb {
    background: var(--surface-300);
    border-radius: 4px;
  }
`;

// File list header container
export const FileListHeader = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;

  h3 {
    margin: 0;
    color: var(--primary-color);
  }
`;

// File list container with scrolling
export const FileListContainer = styled.div`
  flex: 1;
  overflow-y: auto;
  margin-bottom: 1rem;

  &::-webkit-scrollbar {
    width: 8px;
  }

  &::-webkit-scrollbar-track {
    background: var(--surface-100);
    border-radius: 4px;
  }

  &::-webkit-scrollbar-thumb {
    background: var(--surface-300);
    border-radius: 4px;
  }
`;

// Drop zone
export const DropZone = styled.div<{ isDragging: boolean; hasFile: boolean }>`
  flex: 1;
  min-height: 150px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  border: 2px dashed var(--surface-300);
  border-radius: 8px;
  padding: 2rem;
  text-align: center;
  cursor: pointer;
  transition: all 0.3s ease;
  margin-bottom: 1rem;

  &:hover {
    border-color: var(--primary-color);
    background-color: var(--surface-50);
  }

  ${props => props.isDragging && `
    border-color: var(--primary-color);
    background-color: var(--surface-100);
  `}

  ${props => props.hasFile && `
    border-color: var(--primary-color);
    border-style: solid;
    background-color: var(--surface-50);
  `}

  .pi {
    margin-bottom: 0.5rem;
    color: var(--primary-color);
  }

  p {
    margin: 0;
    color: var(--text-color-secondary);
  }
`;

// Upload icon
export const UploadIcon = styled.i`
  font-size: 2rem;
  color: var(--primary-color);
  margin-bottom: 0.5rem;
`;

// Upload text
export const UploadText = styled.p`
  color: var(--text-color-secondary);
  margin-bottom: 0.5rem;
`;

// File name
export const FileName = styled.p`
  font-weight: bold;
  color: var(--text-color);
  margin-top: 0.5rem;
  word-break: break-all;
`;

// Batch indicator
export const BatchIndicator = styled.span`
  background-color: var(--primary-color);
  color: white;
  padding: 0.25rem 0.5rem;
  border-radius: 12px;
  font-size: 0.8rem;
  margin-top: 0.5rem;
`;

// Uploading container
export const UploadingContainer = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
`;

// File actions
export const FileActions = styled.div`
  display: flex;
  gap: 1rem;
  margin-bottom: 1.5rem;
`;

// File item in the list
export const FileItem = styled.div`
  display: flex;
  align-items: center;
  padding: 0.75rem;
  border-radius: 4px;
  margin-bottom: 0.5rem;
  background-color: var(--surface-card);
  border-left: 3px solid var(--primary-color);
  transition: all 0.2s ease;

  &:hover {
    background-color: var(--surface-hover);
  }

  &.selected {
    background-color: var(--primary-50);
    border-left-color: var(--primary-600);
  }
`;

// File icon
export const FileIcon = styled.i`
  font-size: 1.25rem;
  margin-right: 0.75rem;
  color: var(--primary-color);
`;

// File details
export const FileDetails = styled.div`
  flex: 1;
  overflow: hidden;

  .file-name {
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    margin-bottom: 0.25rem;
  }

  .file-info {
    font-size: 0.75rem;
    color: var(--text-color-secondary);
  }
`;

// Conversion options
export const ConversionOptions = styled.div`
  background-color: var(--surface-50);
  padding: 1.5rem;
  border-radius: 8px;
  margin-bottom: 1.5rem;

  h3 {
    margin-top: 0;
    margin-bottom: 1.5rem;
    color: var(--primary-color);
  }

  .p-field {
    margin-bottom: 1.5rem;

    label {
      display: block;
      margin-bottom: 0.5rem;
      font-weight: 500;
    }
  }
`;

// Advanced options
export const AdvancedOptions = styled.div`
  background-color: var(--surface-100);
  padding: 1rem;
  border-radius: 8px;
  margin-bottom: 1rem;
`;

// Conversion actions
export const ConversionActions = styled.div`
  margin-top: 1.5rem;
  display: flex;
  justify-content: flex-end;
`;

// Conversion progress
export const ConversionProgress = styled.div`
  margin-top: 1.5rem;

  h4 {
    margin: 0 0 0.5rem 0;
  }
`;

// Success message
export const SuccessMessage = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 1rem;

  i {
    margin-bottom: 1rem;
  }

  p {
    margin: 0.5rem 0;
    &:first-of-type {
      font-weight: bold;
      font-size: 1.1rem;
    }
  }
`;
