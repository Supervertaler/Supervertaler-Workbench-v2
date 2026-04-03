import type { ICellRendererParams } from 'ag-grid-community';
import type { Segment, SegmentStatus } from '../../types/segment';

const statusColors: Record<SegmentStatus, string> = {
  new: 'bg-gray-300',
  draft: 'bg-yellow-400',
  translated: 'bg-blue-400',
  confirmed: 'bg-green-500',
  approved: 'bg-green-700',
  rejected: 'bg-red-500',
  locked: 'bg-gray-600',
};

const statusLabels: Record<SegmentStatus, string> = {
  new: 'New',
  draft: 'Draft',
  translated: 'Translated',
  confirmed: 'Confirmed',
  approved: 'Approved',
  rejected: 'Rejected',
  locked: 'Locked',
};

export function StatusCellRenderer(params: ICellRendererParams<Segment>) {
  const status = params.value as SegmentStatus;
  const color = statusColors[status] ?? 'bg-gray-300';
  const label = statusLabels[status] ?? status;

  return (
    <div className="flex items-center justify-center h-full" title={label}>
      <div className={`w-3 h-3 rounded-full ${color}`} />
    </div>
  );
}
