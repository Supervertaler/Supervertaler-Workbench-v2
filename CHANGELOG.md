# Changelog

All notable changes to Supervertaler Workbench 2.0 will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0-alpha.1] – 2026-04-03

### Added

- **Translation grid** – AG Grid-based editor with source, target, status, match percentage columns
- **XLIFF 1.2 parser** – Supports standard XLIFF (.xliff, .xlf) with inline tag extraction
- **SDLXLIFF parser** – Trados Studio format with seg-source preference, g/x tag handling
- **MQXLIFF parser** – memoQ XLIFF with bpt/ept/ph inline tag support
- **Inline tag visualization** – Three display modes matching Trados Studio:
  - No Tag Text – minimal markers (▸ ◂ ◆)
  - Partial Tag Text – numbered badges ({1}, {/1})
  - Full Tag Text – raw XML markup
- **Stack-based tag pairing** – Visual numbers correctly show nesting ({1}{2}...{/2}{/1})
- **Tag type detection** – Colour-coded badges for bold, italic, underline, links, placeholders
- **Tag mode toggle** – Cycle between display modes via toolbar button
- **File open dialog** – Supports .xliff, .xlf, .sdlxliff, .mqxliff file filters
- **UTF-8 BOM handling** – Transparent stripping of byte order marks on file load
- **Zustand state management** – Stores for segments, project, settings, and UI state
- **Segment status indicators** – Colour-coded dots (new, draft, translated, confirmed, approved, rejected, locked)
- **Settings store** – Grid font size, tag display mode, theme, auto-propagate preferences
- **Flex layout** – Responsive panel layout with grid + optional right panel (TM results, TermLens)
- **Status bar** – Displays project name, languages, segment count, and progress
- **Tauri 2.x backend** – Rust-based file I/O, XML parsing (quick-xml), SQLite schema (rusqlite)
- **TM fuzzy matching engine** – Levenshtein-based matching (strsim) with configurable threshold
- **SQLite schema** – Tables for translation memory (with FTS5) and termbases

### Technical

- Tauri 2.x + React 19 + TypeScript + Vite
- AG Grid Community v35 with themeQuartz module system
- Tailwind CSS v4 via @tailwindcss/vite plugin
- Rust toolchain: stable-x86_64-pc-windows-msvc

[0.1.0-alpha.1]: https://github.com/Supervertaler/Supervertaler-Workbench-v2/releases/tag/v0.1.0-alpha.1
