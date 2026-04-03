export interface Term {
  id: number;
  sourceTerm: string;
  targetTerm: string;
  priority: number;
  forbidden: boolean;
  notes: string | null;
  domain: string | null;
  termbaseId: number;
}

export interface TermMatch {
  id: number;
  sourceTerm: string;
  targetTerm: string;
  priority: number;
  forbidden: boolean;
  notes: string | null;
}
