import type { TermMatch } from '../../types/termbase';

// TODO: Wire up to actual term lookups from Rust backend
const mockTerms: TermMatch[] = [];

export function TermLensPanel() {
  return (
    <div className="p-2">
      <h3 className="text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300">
        TermLens
      </h3>
      {mockTerms.length === 0 ? (
        <p className="text-xs text-gray-500">No terminology matches</p>
      ) : (
        <div className="space-y-1">
          {mockTerms.map((term) => (
            <div
              key={term.id}
              className="flex items-center justify-between p-1.5 rounded text-xs hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer"
              title={term.notes ?? undefined}
            >
              <span className="text-gray-600 dark:text-gray-400">
                {term.sourceTerm}
              </span>
              <span className="mx-1 text-gray-400">&rarr;</span>
              <span
                className={
                  term.forbidden
                    ? 'text-red-600 line-through'
                    : 'text-gray-900 dark:text-gray-100'
                }
              >
                {term.targetTerm}
              </span>
              {term.priority >= 90 && (
                <span className="ml-1 text-amber-500 text-[10px]">!</span>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
