# Supervertaler Workbench v2 – Claude Code Reference

> Cross-platform AI translation workbench. Tauri 2.x + React + TypeScript frontend, Rust backend.
> Rewrite of the Python/PyQt6 Supervertaler Workbench (see `SUPERVERTALER_WORKBENCH_REWRITE.md` for full spec).

## Quick Commands
- `npm run tauri dev` – development with hot-reload
- `npm run tauri build` – production build
- `cargo test` – run Rust tests (from src-tauri/)
- `npx tsc --noEmit` – check TypeScript
- `npm test` – run frontend tests (when configured)

## Key Paths
| What | Path |
|------|------|
| Tauri config | `src-tauri/tauri.conf.json` |
| Rust entry | `src-tauri/src/lib.rs` |
| Rust commands | `src-tauri/src/commands/` |
| Rust parsers | `src-tauri/src/parsers/` (XLIFF, SDLXLIFF) |
| Rust TM matching | `src-tauri/src/matching/` |
| Rust DB schema | `src-tauri/src/db/schema.rs` |
| React components | `src/components/` |
| Zustand stores | `src/store/` |
| TypeScript types | `src/types/` |
| LLM clients | `src/services/llm/` (TODO) |
| Prompts | `prompts/` (.svprompt files, shared with Trados plugin) |
| Architecture spec | `SUPERVERTALER_WORKBENCH_REWRITE.md` |

## Architecture
- **Rust backend:** File I/O, SQLite (TM + termbase via rusqlite), Okapi sidecar management, TM fuzzy matching (strsim), XML parsing (quick-xml)
- **React frontend:** All UI (AG Grid for translation grid), LLM API calls, prompt management, state management (Zustand)
- **Okapi sidecar:** Java process for DOCX/PPTX extraction (managed by Rust, REST on localhost:8090)

## Component Layout
```
AppLayout
├── MenuBar (file open/save, translate, settings)
├── PanelLayout
│   ├── TranslationGrid (AG Grid – source/target/status/match%)
│   └── Right Panel
│       ├── TMResultsPanel (translation memory matches)
│       └── TermLensPanel (terminology matches)
└── StatusBar (project info, segment count, progress)
```

## Conventions
- Use en dashes (–) not em dashes (—) in all user-facing text
- British English for UI text
- TypeScript strict mode
- Tailwind CSS v4 for styling (imported via @tailwindcss/vite plugin)
- AG Grid Community (MIT) for the translation grid
- Zustand for state management
- .svprompt files use YAML frontmatter + Markdown body (shared format with Supervertaler for Trados)

## Rust Toolchain
- Target: `stable-x86_64-pc-windows-msvc`
- Global shortcut plugin uses builder: `tauri_plugin_global_shortcut::Builder::new().build()`

## Pitfalls
1. Tauri IPC is async – always `await invoke()`
2. WebView CORS: route localhost API calls through Rust commands in production
3. AG Grid: use `applyTransaction` for row updates, not full data replacement
4. Okapi sidecar: test on all platforms (Java process paths differ)
5. Settings files must remain JSON-compatible with v1 (Python) format
6. rusqlite `fts5` feature is not available in v0.32 – FTS5 is available at runtime via SQLite bundled build, just use `CREATE VIRTUAL TABLE ... USING fts5(...)` in SQL
7. Lib name in Cargo.toml is `supervertaler_workbench_lib` (referenced in main.rs)
