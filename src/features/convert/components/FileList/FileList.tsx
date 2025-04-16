import React from 'react';
import { Button } from 'primereact/button';
import { FileListProps } from './types';
import { DropZoneComponent } from '../DropZone';
import { FileListItem } from './FileListItem';
import {
  FileListPanel,
  FileListHeader,
  FileListContainer
} from './FileList.styles';

export const FileList: React.FC<FileListProps> = ({
  files,
  selectedFileId,
  isDragging,
  isUploading,
  dropZoneRef,
  onDragOver,
  onDragLeave,
  onDrop,
  onFileSelect,
  onFileRemove,
  onAddFiles
}) => {
  return (
    <FileListPanel
      ref={dropZoneRef}
      onDragOver={onDragOver}
      onDragLeave={onDragLeave}
      onDrop={onDrop}
    >
      <FileListHeader>
        <h3>Video Files</h3>
        <Button
          label="Add Files"
          icon="pi pi-plus"
          className="p-button-sm"
          onClick={onAddFiles}
        />
      </FileListHeader>

      {/* File list or drop zone */}
      {files.length > 0 ? (
        <FileListContainer>
          {files.map(file => (
            <FileListItem
              key={file.id}
              file={file}
              isSelected={selectedFileId === file.id}
              onSelect={onFileSelect}
              onRemove={onFileRemove}
            />
          ))}
        </FileListContainer>
      ) : (
        <DropZoneComponent
          isDragging={isDragging}
          isUploading={isUploading}
          onClick={onAddFiles}
          onDragOver={onDragOver}
          onDragLeave={onDragLeave}
          onDrop={onDrop}
          ref={dropZoneRef}
        />
      )}
    </FileListPanel>
  );
};
