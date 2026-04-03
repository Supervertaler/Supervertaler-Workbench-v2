import type { Segment } from './segment';

export type FileFormat =
  | 'xliff'
  | 'sdlxliff'
  | 'sdlppx'
  | 'docx'
  | 'mqxliff'
  | 'dejavu_rtf'
  | 'text'
  | 'markdown';

export interface SourceFile {
  originalPath: string;
  format: FileFormat;
  okapiManifest?: string;
}

export interface ProjectSettings {
  autoPropagate: boolean;
  autoCenterActiveSegment: boolean;
  gridFontSize: number;
}

export interface Project {
  path: string;
  name: string;
  sourceLanguage: string;
  targetLanguage: string;
  segments: Segment[];
  sourceFile: SourceFile;
  tmDatabases: string[];
  termbases: string[];
  settings: ProjectSettings;
}
