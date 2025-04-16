import { useState, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { videoService } from '../../../services';
import { useError } from '../../../hooks';
import { ErrorCategory } from '../../../utils';
import { FileItemData } from '../components/FileList/types';

export const useFileManagement = (loadVideoInfo: (path: string) => Promise<any>) => {
  const [isDragging, setIsDragging] = useState<boolean>(false);
  const [isUploading, setIsUploading] = useState<boolean>(false);
  const { setError } = useError();
  const dropZoneRef = useRef<HTMLDivElement>(null);

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

    // Add file to global state
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

    // Load video information
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
    // Remove file from global state
    await invoke('remove_file_from_list', { fileId: file.id });
  };

  return {
    isDragging,
    isUploading,
    dropZoneRef,
    handleDragOver,
    handleDragLeave,
    handleDrop,
    handleFileSelect,
    handleSelectFile,
    handleFileRemove
  };
};
