import {
  forwardRef,
  useEffect,
  useRef,
} from 'react';
import type { CustomCellEditorProps } from 'ag-grid-react';
import type { Segment, ContentPart, TagDisplayMode } from '../../types/segment';
import { useSettingsStore } from '../../store/settingsStore';

/**
 * Custom AG Grid cell editor that renders inline tags as non-editable
 * badges while allowing text editing between them.
 *
 * Uses contentEditable with tag badges inserted as non-editable spans.
 * Calls props.onValueChange() on every input so AG Grid always has the
 * latest plain-text value. Also stashes the full ContentParts array
 * (with updated text) in a module-level variable so the grid can read
 * it after editing stops.
 */

/** Module-level stash for the latest parts from the editor.
 *  Read by TranslationGrid.onCellEditingStopped to update targetParts. */
let _lastEditedParts: ContentPart[] | null = null;
export function getLastEditedParts(): ContentPart[] | null {
  const parts = _lastEditedParts;
  _lastEditedParts = null; // consume once
  return parts;
}

function tagColour(tagType: string): string {
  switch (tagType) {
    case 'bold': return '#7c3aed';
    case 'italic': return '#2563eb';
    case 'underline': return '#059669';
    case 'superscript': return '#d97706';
    case 'subscript': return '#d97706';
    case 'link': return '#0891b2';
    case 'placeholder': return '#dc2626';
    default: return '#6b7280';
  }
}

function getTagLabel(part: ContentPart, visualNumber: number, mode: TagDisplayMode): string {
  if (part.type === 'text') return '';
  const isOpen = part.type === 'tag_open';
  const isStandalone = part.type === 'standalone';

  if (mode === 'none') {
    if (isStandalone) return '\u25C6';
    if (isOpen) return '\u25B8';
    return '\u25C2';
  }
  if (mode === 'partial') {
    const n = String(visualNumber);
    if (isStandalone) return `{${n}}`;
    if (isOpen) return `{${n}}`;
    return `{/${n}}`;
  }
  return part.display;
}

function assignVisualNumbers(parts: ContentPart[]): number[] {
  let nextNum = 1;
  const openStack: number[] = [];
  let standaloneNum = 0;
  const pairedCount = parts.filter(p => p.type === 'tag_open').length;

  return parts.map((part) => {
    if (part.type === 'text') return 0;
    if (part.type === 'tag_open') {
      const num = nextNum++;
      openStack.push(num);
      return num;
    }
    if (part.type === 'tag_close') {
      return openStack.length > 0 ? openStack.pop()! : nextNum++;
    }
    standaloneNum++;
    return pairedCount + standaloneNum;
  });
}

const TAG_DATA_ATTR = 'data-tag-index';

/** Extract plain text from the contentEditable, skipping tag badges */
function extractText(el: HTMLDivElement): string {
  let text = '';
  for (const node of Array.from(el.childNodes)) {
    if (node.nodeType === Node.TEXT_NODE) {
      text += node.textContent ?? '';
    } else if (node instanceof HTMLElement && node.hasAttribute(TAG_DATA_ATTR)) {
      // Tag badge — skip
    } else {
      text += node.textContent ?? '';
    }
  }
  return text;
}

/** Extract ContentParts from the contentEditable, preserving tags */
function extractParts(el: HTMLDivElement, originalParts: ContentPart[]): ContentPart[] {
  const parts: ContentPart[] = [];
  for (const node of Array.from(el.childNodes)) {
    if (node.nodeType === Node.TEXT_NODE) {
      const text = node.textContent ?? '';
      if (text) {
        parts.push({ type: 'text', text });
      }
    } else if (node instanceof HTMLElement && node.hasAttribute(TAG_DATA_ATTR)) {
      const idx = parseInt(node.getAttribute(TAG_DATA_ATTR)!, 10);
      if (idx >= 0 && idx < originalParts.length) {
        parts.push(originalParts[idx]);
      }
    }
  }
  return parts;
}

export const TaggedCellEditor = forwardRef(
  (props: CustomCellEditorProps<Segment>, ref) => {
    const segment = props.data;
    const mode = useSettingsStore((s) => s.settings.general.tagDisplayMode);
    const editorRef = useRef<HTMLDivElement>(null);

    const originalParts = segment?.targetParts ?? [];
    const hasTags = originalParts.some(p => p.type !== 'text');

    // Build initial HTML content — runs once on mount
    useEffect(() => {
      if (!editorRef.current) return;
      const el = editorRef.current;

      if (!hasTags) {
        el.textContent = props.value ?? '';
      } else {
        el.innerHTML = '';
        const visualNumbers = assignVisualNumbers(originalParts);

        for (let i = 0; i < originalParts.length; i++) {
          const part = originalParts[i];
          if (part.type === 'text') {
            el.appendChild(document.createTextNode(part.text));
          } else {
            const span = document.createElement('span');
            span.contentEditable = 'false';
            span.setAttribute(TAG_DATA_ATTR, String(i));
            const colour = tagColour(part.tag_type);
            const label = getTagLabel(part, visualNumbers[i], mode);

            Object.assign(span.style, {
              display: 'inline',
              backgroundColor: colour + '20',
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
              cursor: 'default',
            });
            span.textContent = label;
            span.title = part.display;
            el.appendChild(span);
          }
        }
      }

      // Focus and place cursor at end
      el.focus();
      const range = document.createRange();
      const sel = window.getSelection();
      range.selectNodeContents(el);
      range.collapse(false);
      sel?.removeAllRanges();
      sel?.addRange(range);
      // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    // On every input: push plain text to AG Grid AND stash parts
    const handleInput = () => {
      if (!editorRef.current) return;
      const text = extractText(editorRef.current);
      props.onValueChange(text);
      if (hasTags) {
        _lastEditedParts = extractParts(editorRef.current, originalParts);
      }
    };

    const handleKeyDown = (e: React.KeyboardEvent) => {
      if (e.key === 'Escape') {
        _lastEditedParts = null;
        props.stopEditing(true);
      } else if (e.key === 'Tab') {
        e.preventDefault();
        if (editorRef.current) {
          props.onValueChange(extractText(editorRef.current));
          if (hasTags) {
            _lastEditedParts = extractParts(editorRef.current, originalParts);
          }
        }
        props.stopEditing(false);
      }
    };

    return (
      <div
        ref={editorRef}
        contentEditable
        suppressContentEditableWarning
        onInput={handleInput}
        onKeyDown={handleKeyDown}
        style={{
          width: '100%',
          height: '100%',
          padding: '4px 8px',
          outline: 'none',
          lineHeight: '1.5',
          whiteSpace: 'pre-wrap',
          wordBreak: 'break-word',
          overflow: 'auto',
          boxSizing: 'border-box',
        }}
      />
    );
  }
);

TaggedCellEditor.displayName = 'TaggedCellEditor';
