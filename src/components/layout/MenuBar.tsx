import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { useProjectStore } from '../../store/projectStore';
import { useSegmentStore } from '../../store/segmentStore';
import { useSettingsStore } from '../../store/settingsStore';
import type { Segment, ContentPart, TagDisplayMode } from '../../types/segment';

interface ProjectResponse {
  path: string;
  name: string;
  source_language: string;
  target_language: string;
  segment_count: number;
}

interface ContentPartResponse {
  type: 'text' | 'tag_open' | 'tag_close' | 'standalone';
  text?: string;
  id?: string;
  tag_type?: string;
  display?: string;
}

interface SegmentResponse {
  id: number;
  segment_number: number;
  source_text: string;
  target_text: string;
  status: string;
  match_percentage: number | null;
  match_origin: string | null;
  source_parts: ContentPartResponse[];
  target_parts: ContentPartResponse[];
}

function mapContentPart(p: ContentPartResponse): ContentPart {
  if (p.type === 'text') {
    return { type: 'text', text: p.text ?? '' };
  }
  return {
    type: p.type,
    id: p.id ?? '',
    tag_type: p.tag_type ?? 'formatting',
    display: p.display ?? '',
  } as ContentPart;
}

const tagModeLabels: Record<TagDisplayMode, string> = {
  none: 'No Tag Text',
  partial: 'Partial Tag Text',
  full: 'Full Tag Text',
};

const tagModeCycle: Record<TagDisplayMode, TagDisplayMode> = {
  none: 'partial',
  partial: 'full',
  full: 'none',
};

export function MenuBar() {
  const setProject = useProjectStore((s) => s.setProject);
  const setLoading = useProjectStore((s) => s.setLoading);
  const setSegments = useSegmentStore((s) => s.setSegments);
  const tagDisplayMode = useSettingsStore((s) => s.settings.general.tagDisplayMode);
  const updateGeneral = useSettingsStore((s) => s.updateGeneral);

  const handleOpen = async () => {
    const path = await open({
      filters: [
        {
          name: 'Translation Files',
          extensions: [
            'xliff', 'xlf', 'sdlxliff', 'sdlppx', 'sdlrpx',
            'mqxliff', 'docx', 'txt', 'md',
          ],
        },
        {
          name: 'All Files',
          extensions: ['*'],
        },
      ],
    });
    if (!path) return;

    setLoading(true);
    try {
      const projectResp = await invoke<ProjectResponse>('load_project', { path });

      setProject({
        path: projectResp.path,
        name: projectResp.name,
        sourceLanguage: projectResp.source_language,
        targetLanguage: projectResp.target_language,
        segments: [],
        sourceFile: { originalPath: projectResp.path, format: 'xliff' },
        tmDatabases: [],
        termbases: [],
        settings: {
          autoPropagate: true,
          autoCenterActiveSegment: true,
          gridFontSize: 11,
        },
      });

      const segResp = await invoke<SegmentResponse[]>('get_segments');
      const segments: Segment[] = segResp.map((s) => ({
        id: s.id,
        segmentNumber: s.segment_number,
        sourceText: s.source_text,
        targetText: s.target_text,
        status: s.status as Segment['status'],
        matchPercentage: s.match_percentage,
        matchOrigin: s.match_origin,
        sourceParts: (s.source_parts || []).map(mapContentPart),
        targetParts: (s.target_parts || []).map(mapContentPart),
        notes: null,
        createdBy: null,
        modifiedBy: null,
        modifiedAt: null,
      }));
      setSegments(segments);
    } catch (err) {
      console.error('Failed to open project:', err);
      alert(`Failed to open file: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex items-center gap-1 px-2 py-1 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 text-sm select-none">
      <button
        onClick={handleOpen}
        className="px-3 py-1 rounded hover:bg-gray-200 dark:hover:bg-gray-700"
      >
        Open
      </button>
      <button className="px-3 py-1 rounded hover:bg-gray-200 dark:hover:bg-gray-700">
        Save
      </button>
      <div className="border-l border-gray-300 dark:border-gray-600 h-5 mx-1" />
      <button className="px-3 py-1 rounded hover:bg-gray-200 dark:hover:bg-gray-700">
        Translate
      </button>
      <button className="px-3 py-1 rounded hover:bg-gray-200 dark:hover:bg-gray-700">
        Superlookup
      </button>
      <div className="border-l border-gray-300 dark:border-gray-600 h-5 mx-1" />
      <button
        onClick={() => updateGeneral({ tagDisplayMode: tagModeCycle[tagDisplayMode] })}
        className="px-3 py-1 rounded hover:bg-gray-200 dark:hover:bg-gray-700 text-xs"
        title={`Tag display: ${tagModeLabels[tagDisplayMode]}. Click to cycle.`}
      >
        Tags: {tagModeLabels[tagDisplayMode]}
      </button>
      <div className="flex-1" />
      <button className="px-3 py-1 rounded hover:bg-gray-200 dark:hover:bg-gray-700">
        Settings
      </button>
    </div>
  );
}
