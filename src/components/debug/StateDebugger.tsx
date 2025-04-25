import { useState, useEffect } from 'react';
import { Button } from 'primereact/button';
import { Dialog } from 'primereact/dialog';
import { useAppStore, useFilesStore, useConfigStore } from '../../store';
import { ErrorDisplay } from '../common';
import { createError, ErrorCategory } from '../../utils';

export function StateDebugger() {
  const [visible, setVisible] = useState(false);

  // Chỉ lấy state khi dialog hiển thị để tránh re-render không cần thiết
  const [stateSnapshot, setStateSnapshot] = useState<{
    appState: any;
    filesState: { files: any[]; selectedFileId: string | null };
    filesLoading: boolean;
    filesError: string | null;
    configState: any;
    configLoading: boolean;
    configError: string | null;
  }>({
    appState: {},
    filesState: { files: [], selectedFileId: null },
    filesLoading: false,
    filesError: null,
    configState: {},
    configLoading: false,
    configError: null
  });

  // Cập nhật state snapshot khi dialog mở
  useEffect(() => {
    if (visible) {
      setStateSnapshot({
        appState: useAppStore.getState(),
        filesState: {
          files: useFilesStore.getState().files,
          selectedFileId: useFilesStore.getState().selectedFileId
        },
        filesLoading: useFilesStore.getState().isLoading,
        filesError: useFilesStore.getState().error,
        configState: useConfigStore.getState(),
        configLoading: useConfigStore.getState().isLoading,
        configError: useConfigStore.getState().error
      });
    }
  }, [visible]);

  return (
    <>
      <Button
        icon="pi pi-cog"
        className="p-button-rounded p-button-text p-button-sm"
        onClick={() => setVisible(true)}
        tooltip="Debug State"
        tooltipOptions={{ position: 'left' }}
      />

      <Dialog
        header="State Debugger"
        visible={visible}
        style={{ width: '80vw' }}
        onHide={() => setVisible(false)}
        maximizable
      >
        <div style={{ maxHeight: '70vh', overflow: 'auto' }}>
          <h3>App State</h3>
          <pre>{JSON.stringify(stateSnapshot.appState, null, 2)}</pre>

          <h3>Files State {stateSnapshot.filesLoading && '(Loading...)'}</h3>
          {stateSnapshot.filesError && (
            <ErrorDisplay
              error={createError(ErrorCategory.Task, stateSnapshot.filesError)}
              showDismissButton={false}
            />
          )}
          <pre>{JSON.stringify(stateSnapshot.filesState, null, 2)}</pre>

          <h3>Config State {stateSnapshot.configLoading && '(Loading...)'}</h3>
          {stateSnapshot.configError && (
            <ErrorDisplay
              error={createError(ErrorCategory.Other, stateSnapshot.configError)}
              showDismissButton={false}
            />
          )}
          <pre>{JSON.stringify(stateSnapshot.configState, null, 2)}</pre>
        </div>
      </Dialog>
    </>
  );
}
