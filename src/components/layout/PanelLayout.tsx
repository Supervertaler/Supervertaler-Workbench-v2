import { TranslationGrid } from '../grid/TranslationGrid';
import { TMResultsPanel } from '../tm/TMResultsPanel';
import { TermLensPanel } from '../termlens/TermLensPanel';
import { useUiStore } from '../../store/uiStore';

export function PanelLayout() {
  const showTMResults = useUiStore((s) => s.showTMResults);
  const showTermLens = useUiStore((s) => s.showTermLens);

  return (
    <div style={{ display: 'flex', flex: 1, minHeight: 0 }}>
      {/* Main grid area */}
      <div style={{ flex: 1, minHeight: 0, minWidth: 0 }}>
        <TranslationGrid />
      </div>

      {/* Right panel: TM results + TermLens */}
      {(showTMResults || showTermLens) && (
        <div className="flex flex-col w-80 border-l border-gray-200 dark:border-gray-700 overflow-hidden">
          {showTMResults && (
            <div className="flex-1 overflow-auto">
              <TMResultsPanel />
            </div>
          )}
          {showTermLens && (
            <div className="flex-1 overflow-auto border-t border-gray-200 dark:border-gray-700">
              <TermLensPanel />
            </div>
          )}
        </div>
      )}
    </div>
  );
}
