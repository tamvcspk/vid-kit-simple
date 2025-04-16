export interface PresetDialogProps {
  visible: boolean;
  onHide: () => void;
  presetName: string;
  onPresetNameChange: (name: string) => void;
  presetDescription: string;
  onPresetDescriptionChange: (description: string) => void;
  onSave: () => void;
}
