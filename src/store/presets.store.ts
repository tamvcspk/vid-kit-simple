import { create } from 'zustand';
import { LazyStore } from '@tauri-apps/plugin-store';
import { ConversionPreset } from '../types/store.types';
import { PRESETS_STORE_PATH, PRESETS_STORE_KEYS } from '../constants/stores';
import { v4 as uuidv4 } from 'uuid';

interface PresetsState {
  presets: ConversionPreset[];
  isLoading: boolean;
  error: string | null;
  selectedPresetId: string | null;

  // Actions
  loadPresets: () => Promise<void>;
  savePreset: (preset: ConversionPreset) => Promise<void>;
  deletePreset: (id: string) => Promise<void>;
  createDefaultPresets: () => Promise<void>;
  selectPreset: (id: string | null) => void;
  createPreset: (name: string, description: string) => Promise<ConversionPreset>;
  duplicatePreset: (id: string) => Promise<ConversionPreset | null>;
  getPresetById: (id: string) => ConversionPreset | undefined;
}

export const usePresetsStore = create<PresetsState>((set, get) => ({
  // State
  selectedPresetId: null,
  presets: [],
  isLoading: false,
  error: null,

  loadPresets: async () => {
    set({ isLoading: true, error: null });
    try {
      const store = new LazyStore(PRESETS_STORE_PATH);
      const presets = await store.get(PRESETS_STORE_KEYS.PRESETS) as ConversionPreset[] || [];
      set({ presets, isLoading: false });
    } catch (error) {
      console.error('Failed to load presets:', error);
      set({ error: String(error), isLoading: false });
    }
  },

  savePreset: async (preset) => {
    set({ isLoading: true, error: null });
    try {
      const store = new LazyStore(PRESETS_STORE_PATH);
      const presets = [...get().presets];
      const index = presets.findIndex(p => p.id === preset.id);

      if (index >= 0) {
        presets[index] = preset;
      } else {
        presets.push(preset);
      }

      await store.set(PRESETS_STORE_KEYS.PRESETS, presets);
      await store.save();
      set({ presets, isLoading: false });
    } catch (error) {
      console.error('Failed to save preset:', error);
      set({ error: String(error), isLoading: false });
    }
  },

  deletePreset: async (id) => {
    set({ isLoading: true, error: null });
    try {
      const store = new LazyStore(PRESETS_STORE_PATH);
      const presets = get().presets.filter(p => p.id !== id);
      await store.set(PRESETS_STORE_KEYS.PRESETS, presets);
      await store.save();
      set({ presets, isLoading: false });
    } catch (error) {
      console.error('Failed to delete preset:', error);
      set({ error: String(error), isLoading: false });
    }
  },

  selectPreset: (id: string | null) => {
    set({ selectedPresetId: id });
  },

  getPresetById: (id: string) => {
    return get().presets.find(preset => preset.id === id);
  },

  createPreset: async (name: string, description: string) => {
    const newPreset: ConversionPreset = {
      id: uuidv4(),
      name,
      description,
      output_format: 'mp4',
      resolution: { type: 'original' },
      bitrate: 8000,
      fps: 30,
      codec: 'libx264',
      use_gpu: false,
      audio_codec: 'aac',
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };

    await get().savePreset(newPreset);
    return newPreset;
  },

  duplicatePreset: async (id: string) => {
    const preset = get().presets.find(p => p.id === id);
    if (!preset) return null;

    const duplicatedPreset: ConversionPreset = {
      ...preset,
      id: uuidv4(),
      name: `${preset.name} (Copy)`,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };

    await get().savePreset(duplicatedPreset);
    return duplicatedPreset;
  },

  createDefaultPresets: async () => {
    const { presets } = get();
    if (presets.length > 0) return;

    const defaultPresets: ConversionPreset[] = [
      {
        id: 'default-mp4',
        name: 'Default MP4',
        description: 'Standard MP4 conversion with H.264',
        output_format: 'mp4',
        resolution: { type: 'original' },
        bitrate: 8000,
        fps: 30,
        codec: 'libx264',
        use_gpu: false,
        audio_codec: 'aac',
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      },
      {
        id: 'high-quality',
        name: 'High Quality',
        description: 'High quality conversion with H.264',
        output_format: 'mp4',
        resolution: { type: 'preset', width: 1920, height: 1080 },
        bitrate: 12000,
        fps: 60,
        codec: 'libx264',
        use_gpu: true,
        audio_codec: 'aac',
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      },
      {
        id: 'web-optimized',
        name: 'Web Optimized',
        description: 'Optimized for web streaming',
        output_format: 'mp4',
        resolution: { type: 'preset', width: 1280, height: 720 },
        bitrate: 5000,
        fps: 30,
        codec: 'libx264',
        use_gpu: false,
        audio_codec: 'aac',
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      },
    ];

    for (const preset of defaultPresets) {
      await get().savePreset(preset);
    }
  }
}));
