/** A content part: either plain text or a tag marker */
export type ContentPart =
  | { type: 'text'; text: string }
  | { type: 'tag_open'; id: string; tag_type: string; display: string }
  | { type: 'tag_close'; id: string; tag_type: string; display: string }
  | { type: 'standalone'; id: string; tag_type: string; display: string };

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

/** Tag display mode matching Trados Studio's three modes */
export type TagDisplayMode = 'none' | 'partial' | 'full';

export interface Segment {
  id: number;
  segmentNumber: number;
  sourceText: string;
  targetText: string;
  status: SegmentStatus;
  matchPercentage: number | null;
  matchOrigin: string | null;
  sourceParts: ContentPart[];
  targetParts: ContentPart[];
  notes: string | null;
  createdBy: string | null;
  modifiedBy: string | null;
  modifiedAt: string | null;
  sdlxliffConfirmation?: string;
  sdlxliffComments?: Comment[];
}
