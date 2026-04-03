import type { CustomCellRendererProps } from 'ag-grid-react';
import type { Segment, ContentPart, TagDisplayMode } from '../../types/segment';
import { useSettingsStore } from '../../store/settingsStore';

/**
 * Renders a cell with inline tags shown as styled badges.
 * Supports three display modes (like Trados Studio):
 * - "none": Tags shown as small coloured markers (no text)
 * - "partial": Tags shown as numbered badges (e.g., {1}, {/1})
 * - "full": Full tag display text visible
 *
 * In partial mode, tag numbers are computed visually using a stack:
 * each opening tag gets the next sequential number, and each closing
 * tag gets the number of the most recent unclosed opening tag.
 * This ensures {1}{2}...{/2}{/1} pairing regardless of raw XLIFF IDs.
 */

/** Colour based on tag type */
function tagColour(tagType: string): string {
  switch (tagType) {
    case 'bold': return '#7c3aed';       // purple
    case 'italic': return '#2563eb';     // blue
    case 'underline': return '#059669';  // green
    case 'superscript': return '#d97706'; // amber
    case 'subscript': return '#d97706';  // amber
    case 'link': return '#0891b2';       // cyan
    case 'placeholder': return '#dc2626'; // red
    default: return '#6b7280';           // grey
  }
}

interface VisualTag {
  part: ContentPart;
  visualNumber: number;
}

/**
 * Assign sequential visual numbers to tags.
 * Opening tags get the next number (1, 2, 3...),
 * closing tags get the number of the matching open tag (stack-based).
 * Standalone tags get their own number.
 */
function assignVisualNumbers(parts: ContentPart[]): VisualTag[] {
  let nextNum = 1;
  const openStack: number[] = []; // stack of visual numbers for open tags
  let standaloneNum = 0;

  // First pass: count how many paired opens there are to offset standalone numbers
  const pairedCount = parts.filter(p => p.type === 'tag_open').length;

  return parts.map((part) => {
    if (part.type === 'text') {
      return { part, visualNumber: 0 };
    }
    if (part.type === 'tag_open') {
      const num = nextNum++;
      openStack.push(num);
      return { part, visualNumber: num };
    }
    if (part.type === 'tag_close') {
      const num = openStack.length > 0 ? openStack.pop()! : nextNum++;
      return { part, visualNumber: num };
    }
    // standalone
    standaloneNum++;
    return { part, visualNumber: pairedCount + standaloneNum };
  });
}

interface TagBadgeProps {
  part: ContentPart;
  visualNumber: number;
  mode: TagDisplayMode;
}

function TagBadge({ part, visualNumber, mode }: TagBadgeProps) {
  if (part.type === 'text') return null;

  const colour = tagColour(part.tag_type);
  const isOpen = part.type === 'tag_open';
  const isClose = part.type === 'tag_close';
  const isStandalone = part.type === 'standalone';

  // Build label based on mode
  let label: string;
  if (mode === 'none') {
    if (isStandalone) label = '\u25C6'; // ◆
    else if (isOpen) label = '\u25B8';  // ▸
    else label = '\u25C2';              // ◂
  } else if (mode === 'partial') {
    const n = String(visualNumber);
    if (isStandalone) label = `{${n}}`;
    else if (isOpen) label = `{${n}}`;
    else label = `{/${n}}`;
  } else {
    // Full mode — show raw display text
    label = part.display;
  }

  const baseStyle: React.CSSProperties = {
    display: 'inline-block',
    backgroundColor: colour + '20', // 12% opacity
    color: colour,
    border: `1px solid ${colour}60`,
    borderRadius: mode === 'none' ? '2px' : '3px',
    padding: mode === 'none' ? '0 1px' : mode === 'full' ? '0 3px' : '0 2px',
    fontSize: mode === 'full' ? '0.75em' : mode === 'none' ? '0.65em' : '0.8em',
    lineHeight: '1.4',
    verticalAlign: 'baseline',
    fontFamily: mode === 'full' ? 'monospace' : 'inherit',
    whiteSpace: 'nowrap',
    userSelect: 'none',
    margin: '0 0.5px',
  };

  return (
    <span style={baseStyle} title={part.display}>
      {label}
    </span>
  );
}

function RenderParts({ parts, mode }: { parts: ContentPart[]; mode: TagDisplayMode }) {
  const visualTags = assignVisualNumbers(parts);

  return (
    <span>
      {visualTags.map((vt, i) =>
        vt.part.type === 'text' ? (
          <span key={i}>{vt.part.text}</span>
        ) : (
          <TagBadge key={i} part={vt.part} visualNumber={vt.visualNumber} mode={mode} />
        )
      )}
    </span>
  );
}

export function SourceTextRenderer(props: CustomCellRendererProps<Segment>) {
  const mode = useSettingsStore((s) => s.settings.general.tagDisplayMode);
  const segment = props.data;
  if (!segment) return null;

  const parts = segment.sourceParts;

  if (!parts || parts.length === 0 || parts.every((p) => p.type === 'text')) {
    return <span>{segment.sourceText}</span>;
  }

  return <RenderParts parts={parts} mode={mode} />;
}

export function TargetTextRenderer(props: CustomCellRendererProps<Segment>) {
  const mode = useSettingsStore((s) => s.settings.general.tagDisplayMode);
  const segment = props.data;
  if (!segment) return null;

  const parts = segment.targetParts;

  if (!parts || parts.length === 0 || parts.every((p) => p.type === 'text')) {
    return <span>{segment.targetText}</span>;
  }

  return <RenderParts parts={parts} mode={mode} />;
}
