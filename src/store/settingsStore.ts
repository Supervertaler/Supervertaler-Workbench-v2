import { create } from 'zustand';
import type { TagDisplayMode } from '../types/segment';

interface Settings {
  general: {
    restoreLastProject: boolean;
    autoPropagate: boolean;
    autoCenterActiveSegment: boolean;
    gridFontSize: number;
    enableSoundEffects: boolean;
    tagDisplayMode: TagDisplayMode;
  };
  apiKeys: Record<string, string>;
  ui: {
    theme: string;
  };
}

interface SettingsStore {
  settings: Settings;
  setSettings: (settings: Settings) => void;
  updateGeneral: (updates: Partial<Settings['general']>) => void;
}

const defaultSettings: Settings = {
  general: {
    restoreLastProject: false,
    autoPropagate: true,
    autoCenterActiveSegment: true,
    gridFontSize: 11,
    enableSoundEffects: false,
    tagDisplayMode: 'partial' as TagDisplayMode,
  },
  apiKeys: {},
  ui: {
    theme: 'light',
  },
};

export const useSettingsStore = create<SettingsStore>((set) => ({
  settings: defaultSettings,
  setSettings: (settings) => set({ settings }),
  updateGeneral: (updates) =>
    set((state) => ({
      settings: {
        ...state.settings,
        general: { ...state.settings.general, ...updates },
      },
    })),
}));
