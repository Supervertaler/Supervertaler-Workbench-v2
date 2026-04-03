export interface TMEntry {
  id: number;
  sourceText: string;
  targetText: string;
  sourceLanguage: string;
  targetLanguage: string;
  createdBy: string;
  createdAt: string;
  modifiedAt: string;
  context?: string;
  origin?: string;
}

export interface TmMatch {
  id: number;
  sourceText: string;
  targetText: string;
  matchPercentage: number;
  origin: string;
}
