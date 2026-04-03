export interface InlineTag {
  type: 'b' | 'i' | 'u' | 's' | 'sup' | 'sub' | 'cf' | 'placeholder';
  id: string;
  content?: string;
  position: number;
}

export interface Comment {
  author: string;
  date: string;
  text: string;
}

export type SegmentStatus =
  | 'new'
  | 'draft'
  | 'translated'
  | 'confirmed'
  | 'approved'
  | 'rejected'
  | 'locked';

export interface Segment {
  id: number;
  segmentNumber: number;
  sourceText: string;
  targetText: string;
  status: SegmentStatus;
  matchPercentage: number | null;
  matchOrigin: string | null;
  sourceInlineTags: InlineTag[];
  targetInlineTags: InlineTag[];
  notes: string | null;
  createdBy: string | null;
  modifiedBy: string | null;
  modifiedAt: string | null;
  sdlxliffConfirmation?: string;
  sdlxliffComments?: Comment[];
}
