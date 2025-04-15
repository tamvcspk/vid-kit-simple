import { useState } from 'react';
import { Button } from 'primereact/button';
import { Dialog } from 'primereact/dialog';
import useAppStore from '../../states/app-state';
import useConversionStore from '../../states/conversion-state';
import usePreferencesStore from '../../states/preferences-state';

export function StateDebugger() {
  const [visible, setVisible] = useState(false);

  // Sử dụng các store riêng biệt
  const appState = useAppStore(state => state.data);
  const appLoading = useAppStore(state => state.isLoading);
  const appError = useAppStore(state => state.error);

  const conversionState = useConversionStore(state => state.data);
  const conversionLoading = useConversionStore(state => state.isLoading);
  const conversionError = useConversionStore(state => state.error);

  const preferencesState = usePreferencesStore(state => state.data);
  const preferencesLoading = usePreferencesStore(state => state.isLoading);
  const preferencesError = usePreferencesStore(state => state.error);

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
          <h3>App State {appLoading && '(Loading...)'}</h3>
          {appError && <div className="p-error">{appError}</div>}
          <pre>{JSON.stringify(appState, null, 2)}</pre>

          <h3>Conversion State {conversionLoading && '(Loading...)'}</h3>
          {conversionError && <div className="p-error">{conversionError}</div>}
          <pre>{JSON.stringify(conversionState, null, 2)}</pre>

          <h3>Preferences {preferencesLoading && '(Loading...)'}</h3>
          {preferencesError && <div className="p-error">{preferencesError}</div>}
          <pre>{JSON.stringify(preferencesState, null, 2)}</pre>
        </div>
      </Dialog>
    </>
  );
}
