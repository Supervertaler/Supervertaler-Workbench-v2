import { useProjectStore } from '../../store/projectStore';
import { useSegmentStore } from '../../store/segmentStore';

export function StatusBar() {
  const project = useProjectStore((s) => s.project);
  const segments = useSegmentStore((s) => s.segments);
  const activeId = useSegmentStore((s) => s.activeSegmentId);

  const activeSegment = segments.find((s) => s.id === activeId);
  const confirmedCount = segments.filter(
    (s) => s.status === 'confirmed' || s.status === 'approved'
  ).length;

  return (
    <div className="flex items-center gap-4 px-3 py-1 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 text-xs text-gray-600 dark:text-gray-400">
      {project ? (
        <>
          <span>{project.name}</span>
          <span>
            {project.sourceLanguage} &rarr; {project.targetLanguage}
          </span>
          <span>
            {confirmedCount}/{segments.length} confirmed
          </span>
          {activeSegment && (
            <span>Segment {activeSegment.segmentNumber}</span>
          )}
        </>
      ) : (
        <span>No project open</span>
      )}
    </div>
  );
}
