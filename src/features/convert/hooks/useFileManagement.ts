import { useState, useRef } from 'react';
import { videoService } from '../../../services';
import { useError } from '../../../hooks';
import { ErrorCategory } from '../../../utils';
import { FileItemData } from '../components/FileList/types';
import { useFilesStore } from '../../../store';

export const useFileManagement = (loadVideoInfo: (path: string) => Promise<any>) => {
  const [isDragging, setIsDragging] = useState<boolean>(false);
  const [isUploading, setIsUploading] = useState<boolean>(false);
  const { setError, clearError } = useError();
  const dropZoneRef = useRef<HTMLDivElement>(null);

  // Use the new files store
  const { addFile, updateFile, removeFile, selectFile } = useFilesStore();

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
      const fileId = await addFile({
        name: file.name,
        path: file.path || '',
        size: file.size,
        type: file.type || 'video/mp4'
      });

      // Load video info for the file
      const videoInfo = await loadVideoInfo(file.path || '');

      // Update file with video info
      if (videoInfo) {
        await updateFile(fileId, {
          duration: videoInfo.duration,
          resolution: videoInfo.resolution ? {
            width: videoInfo.resolution[0],
            height: videoInfo.resolution[1]
          } : undefined,
          thumbnail: videoInfo.thumbnail
        });
      }

      // Select the first file
      if (videoFiles[0] === file) {
        await selectFile(fileId);
      }
    }

    setIsUploading(false);
  };

  // Add file to list
  const addFileToList = async (filePath: string, fileName: string, fileSize: number, fileType: string) => {
    // Add file to global state
    const fileId = await addFile({
      name: fileName,
      path: filePath,
      size: fileSize,
      type: fileType || 'video/mp4'
    });

    // Load video information
    const videoInfo = await loadVideoInfo(filePath);

    // Update file with video info
    if (videoInfo) {
      await updateFile(fileId, {
        duration: videoInfo.duration,
        resolution: videoInfo.resolution ? {
          width: videoInfo.resolution[0],
          height: videoInfo.resolution[1]
        } : undefined,
        thumbnail: videoInfo.thumbnail
      });
    }

    // Select the file
    await selectFile(fileId);
  };

  // Handle file selection from list
  const handleFileSelect = async (file: FileItemData) => {
    await selectFile(file.id);
    await loadVideoInfo(file.path);
  };

  // Handle file selection using native dialog
  const handleSelectFile = async () => {
    try {
      setIsUploading(true);
      clearError();

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
    await removeFile(file.id);
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
