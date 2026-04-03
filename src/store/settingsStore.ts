import { create } from 'zustand';

interface Settings {
  general: {
    restoreLastProject: boolean;
    autoPropagate: boolean;
    autoCenterActiveSegment: boolean;
    gridFontSize: number;
    enableSoundEffects: boolean;
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
