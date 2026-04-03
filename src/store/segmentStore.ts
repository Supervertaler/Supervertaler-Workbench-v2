import { create } from 'zustand';
import type { Segment } from '../types/segment';

interface SegmentStore {
  segments: Segment[];
  activeSegmentId: number | null;
  setSegments: (segments: Segment[]) => void;
  setActiveSegment: (id: number) => void;
  updateSegment: (id: number, updates: Partial<Segment>) => void;
}

export const useSegmentStore = create<SegmentStore>((set) => ({
  segments: [],
  activeSegmentId: null,
  setSegments: (segments) => set({ segments }),
  setActiveSegment: (id) => set({ activeSegmentId: id }),
  updateSegment: (id, updates) =>
    set((state) => ({
      segments: state.segments.map((s) =>
        s.id === id ? { ...s, ...updates } : s
      ),
    })),
}));
