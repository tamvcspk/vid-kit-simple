import React from 'react';

export interface DropZoneProps {
  isDragging: boolean;
  isUploading: boolean;
  onClick: () => void;
  onDragOver: (e: React.DragEvent) => void;
  onDragLeave: (e: React.DragEvent) => void;
  onDrop: (e: React.DragEvent) => void;
}
