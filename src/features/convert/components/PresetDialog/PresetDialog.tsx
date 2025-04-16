import React from 'react';
import { Dialog } from 'primereact/dialog';
import { Button } from 'primereact/button';
import { InputText } from 'primereact/inputtext';
import { PresetDialogProps } from './types';

export const PresetDialog: React.FC<PresetDialogProps> = ({
  visible,
  onHide,
  presetName,
  onPresetNameChange,
  presetDescription,
  onPresetDescriptionChange,
  onSave
}) => {
  return (
    <Dialog
      header="Save New Preset"
      visible={visible}
      onHide={onHide}
      style={{ width: '450px' }}
      footer={
        <div>
          <Button
            label="Cancel"
            icon="pi pi-times"
            onClick={onHide}
            className="p-button-text"
          />
          <Button label="Save" icon="pi pi-save" onClick={onSave} autoFocus />
        </div>
      }
    >
      <div className="p-field">
        <label htmlFor="presetName">Preset Name</label>
        <InputText
          id="presetName"
          value={presetName}
          onChange={e => onPresetNameChange(e.target.value)}
          placeholder="Enter preset name"
          className="w-full"
          required
          autoFocus
        />
      </div>

      <div className="p-field">
        <label htmlFor="presetDescription">Description</label>
        <InputText
          id="presetDescription"
          value={presetDescription}
          onChange={e => onPresetDescriptionChange(e.target.value)}
          placeholder="Enter description (optional)"
          className="w-full"
        />
      </div>
    </Dialog>
  );
};
