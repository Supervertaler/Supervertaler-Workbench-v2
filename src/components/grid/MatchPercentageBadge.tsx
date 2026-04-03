import type { ICellRendererParams } from 'ag-grid-community';
import type { Segment } from '../../types/segment';

function getMatchColor(pct: number): string {
  if (pct === 100) return 'bg-green-100 text-green-800';
  if (pct >= 95) return 'bg-emerald-100 text-emerald-800';
  if (pct >= 75) return 'bg-yellow-100 text-yellow-800';
  if (pct >= 50) return 'bg-orange-100 text-orange-800';
  return 'bg-gray-100 text-gray-600';
}

export function MatchPercentageBadge(params: ICellRendererParams<Segment>) {
  const pct = params.value as number | null;
  if (pct == null) return null;

  const color = getMatchColor(pct);
  return (
    <span
      className={`inline-flex items-center px-1.5 py-0.5 rounded text-xs font-medium ${color}`}
    >
      {Math.round(pct)}%
    </span>
  );
}
