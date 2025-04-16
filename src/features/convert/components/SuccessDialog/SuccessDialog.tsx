import React from 'react';
import { Dialog } from 'primereact/dialog';
import { Button } from 'primereact/button';
import { invoke } from '@tauri-apps/api/core';
import { SuccessDialogProps } from './types';
import { SuccessMessage } from './SuccessDialog.styles';

export const SuccessDialog: React.FC<SuccessDialogProps> = ({
  visible,
  onHide,
  outputPath
}) => {
  const handleOpenFolder = () => {
    if (outputPath) {
      const folderPath = outputPath.substring(0, outputPath.lastIndexOf('/'));
      invoke('plugin:opener|open_item', { path: folderPath });
    }
  };

  return (
    <Dialog
      header="Conversion Successful"
      visible={visible}
      onHide={onHide}
      style={{ width: '50vw' }}
      footer={
        <div>
          <Button
            label="Close"
            onClick={onHide}
            className="p-button-text"
          />
          <Button
            label="Open Folder"
            icon="pi pi-folder-open"
            onClick={handleOpenFolder}
          />
        </div>
      }
    >
      <SuccessMessage>
        <i
          className="pi pi-check-circle"
          style={{ fontSize: '2rem', color: 'var(--green-500)' }}
        ></i>
        <p>Video has been successfully converted!</p>
        <p>Path: {outputPath}</p>
      </SuccessMessage>
    </Dialog>
  );
};
