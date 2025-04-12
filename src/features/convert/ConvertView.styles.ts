import styled from '@emotion/styled';
import { Card } from 'primereact/card';

// Main container
export const Container = styled.div`
  padding: 1.5rem;

  h2 {
    margin-bottom: 1.5rem;
    color: var(--primary-color);
  }

  // Responsive adjustments
  @media (max-width: 768px) {
    .p-grid {
      display: flex;
      flex-direction: column;
    }

    .p-dialog {
      width: 90vw !important;
    }
  }
`;

// Drop zone
export const DropZone = styled.div<{ isDragging: boolean; hasFile: boolean }>`
  // Extend the base drop-zone styles with specific styles for convert view
  margin-bottom: 1rem;
  position: relative;
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

// Video info card
export const VideoInfoCard = styled(Card)`
  margin-bottom: 1.5rem;

  h3 {
    margin-top: 0;
    margin-bottom: 1rem;
    color: var(--primary-color);
  }
`;

// Info item
export const InfoItem = styled.div`
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
