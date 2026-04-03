import { useCallback, useMemo, useRef } from 'react';
import { AgGridReact } from 'ag-grid-react';
import type { ColDef, CellEditingStoppedEvent, RowClickedEvent } from 'ag-grid-community';
import { AllCommunityModule, ModuleRegistry, themeQuartz } from 'ag-grid-community';
import { useSegmentStore } from '../../store/segmentStore';
import { useSettingsStore } from '../../store/settingsStore';
import { StatusCellRenderer } from './StatusCellRenderer';
import { MatchPercentageBadge } from './MatchPercentageBadge';
import { SourceTextRenderer, TargetTextRenderer } from './TaggedTextRenderer';
import type { Segment } from '../../types/segment';

ModuleRegistry.registerModules([AllCommunityModule]);

const myTheme = themeQuartz;

export function TranslationGrid() {
  const gridRef = useRef<AgGridReact>(null);
  const segments = useSegmentStore((s) => s.segments);
  const setActiveSegment = useSegmentStore((s) => s.setActiveSegment);
  const updateSegment = useSegmentStore((s) => s.updateSegment);
  const fontSize = useSettingsStore((s) => s.settings.general.gridFontSize);

  const columnDefs = useMemo<ColDef<Segment>[]>(
    () => [
      {
        headerName: '#',
        field: 'segmentNumber',
        width: 60,
        pinned: 'left',
        sortable: false,
      },
      {
        headerName: '',
        field: 'status',
        width: 40,
        cellRenderer: StatusCellRenderer,
        sortable: false,
      },
      {
        headerName: 'Source',
        field: 'sourceText',
        flex: 1,
        wrapText: true,
        autoHeight: true,
        editable: false,
        cellRenderer: SourceTextRenderer,
      },
      {
        headerName: 'Target',
        field: 'targetText',
        flex: 1,
        wrapText: true,
        autoHeight: true,
        editable: true,
        cellStyle: { cursor: 'text' },
        cellRenderer: TargetTextRenderer,
      },
      {
        headerName: '%',
        field: 'matchPercentage',
        width: 60,
        cellRenderer: MatchPercentageBadge,
        sortable: true,
      },
    ],
    []
  );

  const defaultColDef = useMemo<ColDef>(
    () => ({
      resizable: true,
    }),
    []
  );

  const onRowClicked = useCallback(
    (event: RowClickedEvent<Segment>) => {
      if (event.data) {
        setActiveSegment(event.data.id);
      }
    },
    [setActiveSegment]
  );

  const onCellEditingStopped = useCallback(
    (event: CellEditingStoppedEvent<Segment>) => {
      if (event.data && event.colDef.field === 'targetText') {
        updateSegment(event.data.id, {
          targetText: event.newValue,
          status: event.data.status === 'new' ? 'draft' : event.data.status,
        });
      }
    },
    [updateSegment]
  );

  return (
    <div style={{ width: '100%', height: '100%', fontSize: `${fontSize}px` }}>
      <AgGridReact<Segment>
        ref={gridRef}
        rowData={segments}
        columnDefs={columnDefs}
        defaultColDef={defaultColDef}
        getRowId={(params) => String(params.data.id)}
        onRowClicked={onRowClicked}
        onCellEditingStopped={onCellEditingStopped}
        rowSelection="single"
        suppressMovableColumns
        theme={myTheme}
      />
    </div>
  );
}
