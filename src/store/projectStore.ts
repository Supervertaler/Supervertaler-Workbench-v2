import { create } from 'zustand';
import type { Project } from '../types/project';

interface ProjectStore {
  project: Project | null;
  isLoading: boolean;
  setProject: (project: Project | null) => void;
  setLoading: (loading: boolean) => void;
}

export const useProjectStore = create<ProjectStore>((set) => ({
  project: null,
  isLoading: false,
  setProject: (project) => set({ project }),
  setLoading: (loading) => set({ isLoading: loading }),
}));
