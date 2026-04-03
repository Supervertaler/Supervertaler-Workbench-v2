import { useCallback, useMemo, useRef } from 'react';
import { AgGridReact } from 'ag-grid-react';
import type { ColDef, CellEditingStoppedEvent, RowClickedEvent } from 'ag-grid-community';
import { AllCommunityModule, ModuleRegistry, themeQuartz } from 'ag-grid-community';
import { invoke } from '@tauri-apps/api/core';
import { useSegmentStore } from '../../store/segmentStore';
import { useSettingsStore } from '../../store/settingsStore';
import { StatusCellRenderer } from './StatusCellRenderer';
import { MatchPercentageBadge } from './MatchPercentageBadge';
import { SourceTextRenderer, TargetTextRenderer } from './TaggedTextRenderer';
import { TaggedCellEditor, getLastEditedParts } from './TaggedCellEditor';
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
        cellEditor: TaggedCellEditor,
        cellEditorPopup: false,
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
      if (event.data && event.colDef.field === 'targetText' && event.valueChanged) {
        const newTarget = event.newValue ?? '';

        // Get updated parts from the editor (includes tags + new text)
        const editedParts = getLastEditedParts();

        // Update frontend state — both targetText and targetParts
        updateSegment(event.data.id, {
          targetText: newTarget,
          targetParts: editedParts ?? event.data.targetParts,
          status: event.data.status === 'new' ? 'draft' : event.data.status,
        });

        // Force AG Grid to re-render the row — it doesn't know targetParts changed
        setTimeout(() => {
          event.api.refreshCells({
            rowNodes: event.node ? [event.node] : undefined,
            force: true,
          });
        }, 0);

        // Sync to Rust backend so Save writes the correct data
        const updatedParts = editedParts ?? event.data.targetParts;
        invoke('save_segment', {
          segmentId: event.data.id,
          target: newTarget,
          targetParts: updatedParts.length > 0 ? updatedParts : null,
        }).catch((err) => console.error('Failed to sync segment to backend:', err));
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
