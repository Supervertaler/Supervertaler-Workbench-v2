# Supervertaler Workbench 2.0

A cross-platform AI-powered translation workbench for professional translators. Built to work alongside established CAT tools (memoQ, Trados Studio, CafeTran Espresso) by opening their bilingual file formats directly.

> **Status:** Early alpha – core grid and file parsing work, many features still in development.

## Features

- **Translation grid** – Fast AG Grid-based editor with source/target columns, status indicators, and match percentages
- **Bilingual file support** – Opens XLIFF 1.2, SDLXLIFF (Trados), MQXLIFF (memoQ), and more planned
- **Inline tag visualization** – Three display modes (No Tag Text, Partial Tag Text, Full Tag Text) with colour-coded badges
- **Translation memory** – SQLite-backed TM with FTS5 full-text search and Levenshtein fuzzy matching
- **Terminology lookup** – TermLens panel for termbase matches (planned)
- **AI translation** – LLM-powered translation and review via configurable prompts (planned)

## Supported Formats

| Format | Extension | Status |
|--------|-----------|--------|
| XLIFF 1.2 | .xliff, .xlf | Working |
| SDLXLIFF (Trados Studio) | .sdlxliff | Working |
| MQXLIFF (memoQ) | .mqxliff | Working |
| SDLPPX (Trados Package) | .sdlppx | Planned |
| CafeTran XLIFF | .xliff | Planned |
| Phrase XLIFF | .xliff | Planned |

## Architecture

```
Tauri 2.x
├── Rust backend     – File I/O, XML parsing, SQLite TM/termbase, fuzzy matching
└── React frontend   – AG Grid, Zustand state, LLM API calls, prompt management
```

**Key technologies:** Tauri 2.x, React 19, TypeScript, AG Grid Community, Zustand, Tailwind CSS v4, quick-xml, rusqlite, strsim

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) (stable-x86_64-pc-windows-msvc on Windows)
- [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)

### Development

```bash
npm install
npm run tauri dev
```

### Build

```bash
npm run tauri build
```

### Run Rust Tests

```bash
cd src-tauri
cargo test
```

## Project Structure

```
src/                          # React frontend
├── components/
│   ├── grid/                 # TranslationGrid, TaggedTextRenderer, StatusCellRenderer
│   ├── layout/               # AppLayout, MenuBar, PanelLayout, StatusBar
│   ├── tm/                   # TMResultsPanel
│   └── termlens/             # TermLensPanel
├── store/                    # Zustand stores (segments, project, settings, UI)
├── types/                    # TypeScript type definitions
└── services/                 # LLM clients (planned)

src-tauri/                    # Rust backend
├── src/
│   ├── commands/             # Tauri IPC command handlers
│   ├── parsers/              # XLIFF, SDLXLIFF parsers
│   ├── matching/             # Levenshtein fuzzy matching
│   └── db/                   # SQLite schema (TM, termbases)
└── Cargo.toml
```

## Related Projects

- [Supervertaler](https://github.com/michaelbeijer/Supervertaler) – The original Python/PyQt6 desktop version
- [Supervertaler for Trados](https://github.com/michaelbeijer/Supervertaler-for-Trados) – Trados Studio plugin (C#/.NET)

## Licence

MIT
