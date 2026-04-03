import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { useProjectStore } from '../../store/projectStore';
import { useSegmentStore } from '../../store/segmentStore';

export function MenuBar() {
  const setProject = useProjectStore((s) => s.setProject);
  const setLoading = useProjectStore((s) => s.setLoading);
  const setSegments = useSegmentStore((s) => s.setSegments);

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
      ],
    });
    if (!path) return;

    setLoading(true);
    try {
      const project = await invoke('load_project', { path });
      setProject(project as any);
      const segments = await invoke('get_segments');
      setSegments(segments as any);
    } catch (err) {
      console.error('Failed to open project:', err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex items-center gap-1 px-2 py-1 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 text-sm">
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
      <div className="flex-1" />
      <button className="px-3 py-1 rounded hover:bg-gray-200 dark:hover:bg-gray-700">
        Settings
      </button>
    </div>
  );
}
