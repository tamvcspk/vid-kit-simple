import { Button } from 'primereact/button';
import styled from '@emotion/styled';
import { useState } from 'react';
import { logService } from '../../../services';

const LogButtonsContainer = styled.div`
  display: flex;
  align-items: center;
  gap: 0.5rem;
`;

const LogButton = styled(Button)`
  &.p-button {
    background: transparent;
    border: 1px solid var(--surface-border);
    color: var(--text-color);
    padding: 0.25rem 0.5rem;
    transition: all 0.2s ease;
    font-size: 0.75rem;

    &:hover {
      background: var(--surface-hover);
      border-color: var(--primary-color);
    }

    &:focus {
      box-shadow: none;
    }

    .p-button-icon {
      font-size: 0.875rem;
    }

    .p-button-label {
      font-size: 0.75rem;
    }
  }
`;

export function LogButtons() {
  const [isLoading, setIsLoading] = useState(false);

  const handleOpenLog = async () => {
    setIsLoading(true);
    try {
      await logService.openCurrentLogFile();
    } catch (error) {
      console.error('Failed to open log file:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleOpenLogLocation = async () => {
    setIsLoading(true);
    try {
      await logService.openLogDirectory();
    } catch (error) {
      console.error('Failed to open log location:', error);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <LogButtonsContainer>
      <LogButton
        icon="pi pi-file"
        onClick={handleOpenLog}
        loading={isLoading}
        tooltip="Open Log File"
        tooltipOptions={{ position: 'top' }}
        aria-label="Open Log File"
      />
      <LogButton
        icon="pi pi-folder"
        onClick={handleOpenLogLocation}
        loading={isLoading}
        tooltip="Open Log Folder"
        tooltipOptions={{ position: 'top' }}
        aria-label="Open Log Folder"
      />
    </LogButtonsContainer>
  );
}
