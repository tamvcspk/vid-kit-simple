import React from 'react';

export interface FileItemData {
  id: string;
  name: string;
  path: string;
  size: number;
  fileType: string;
  duration?: number | null;
  resolution?: [number, number] | null;
  thumbnail?: string | null;
}

export interface FileListProps {
  files: FileItemData[];
  selectedFileId: string | null;
  isDragging: boolean;
  isUploading: boolean;
  dropZoneRef: React.RefObject<HTMLDivElement>;
  onDragOver: (e: React.DragEvent) => void;
  onDragLeave: (e: React.DragEvent) => void;
  onDrop: (e: React.DragEvent) => void;
  onFileSelect: (file: FileItemData) => void;
  onFileRemove: (file: FileItemData) => void;
  onAddFiles: () => void;
}

export interface FileListItemProps {
  file: FileItemData;
  isSelected: boolean;
  onSelect: (file: FileItemData) => void;
  onRemove: (file: FileItemData) => void;
}
