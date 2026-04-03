import { create } from 'zustand';

interface UiStore {
  showSettings: boolean;
  showTermLens: boolean;
  showTMResults: boolean;
  showSuperlookup: boolean;
  setShowSettings: (show: boolean) => void;
  setShowTermLens: (show: boolean) => void;
  setShowTMResults: (show: boolean) => void;
  setShowSuperlookup: (show: boolean) => void;
}

export const useUiStore = create<UiStore>((set) => ({
  showSettings: false,
  showTermLens: true,
  showTMResults: true,
  showSuperlookup: false,
  setShowSettings: (show) => set({ showSettings: show }),
  setShowTermLens: (show) => set({ showTermLens: show }),
  setShowTMResults: (show) => set({ showTMResults: show }),
  setShowSuperlookup: (show) => set({ showSuperlookup: show }),
}));
