import type { TmMatch } from '../../types/tm';

// TODO: Wire up to actual TM results from Rust backend
const mockMatches: TmMatch[] = [];

export function TMResultsPanel() {
  return (
    <div className="p-2">
      <h3 className="text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300">
        Translation Memory
      </h3>
      {mockMatches.length === 0 ? (
        <p className="text-xs text-gray-500">No TM matches</p>
      ) : (
        <div className="space-y-2">
          {mockMatches.map((match) => (
            <div
              key={match.id}
              className="p-2 rounded border border-gray-200 dark:border-gray-700 text-xs cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-800"
            >
              <div className="flex justify-between mb-1">
                <span className="font-medium text-blue-600">
                  {Math.round(match.matchPercentage)}%
                </span>
                <span className="text-gray-400">{match.origin}</span>
              </div>
              <div className="text-gray-600 dark:text-gray-400 mb-1">
                {match.sourceText}
              </div>
              <div className="text-gray-900 dark:text-gray-100">
                {match.targetText}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
