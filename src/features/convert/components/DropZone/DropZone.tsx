import React, { forwardRef } from 'react';
import { DropZoneProps } from './types';
import {
  DropZoneContainer,
  UploadIcon,
  UploadText,
  UploadingContainer
} from './DropZone.styles';

export const DropZoneComponent = forwardRef<HTMLDivElement, DropZoneProps>((
  {
    isDragging,
    isUploading,
    onClick,
    onDragOver,
    onDragLeave,
    onDrop
  },
  ref
) => {
  return (
    <DropZoneContainer
      ref={ref}
      isDragging={isDragging}
      hasFile={false}
      onClick={onClick}
      onDragOver={onDragOver}
      onDragLeave={onDragLeave}
      onDrop={onDrop}
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
    </DropZoneContainer>
  );
});
