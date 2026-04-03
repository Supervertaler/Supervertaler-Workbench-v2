---
title: Changelog
description: Version history for Supervertaler Workbench 2.0.
---

## 2.0.0-alpha.1 – 2026-04-03

### Added

- Translation grid with AG Grid-based editor
- XLIFF 1.2 parser with inline tag extraction
- SDLXLIFF parser (Trados Studio format)
- MQXLIFF parser (memoQ format)
- Inline tag visualization with three display modes (No Tag Text, Partial Tag Text, Full Tag Text)
- Stack-based tag pairing for correct visual numbering
- Tag type detection with colour-coded badges
- File open dialog with format filters
- UTF-8 BOM handling
- Segment status indicators
- Settings store with tag display mode, font size, theme preferences
- Zustand state management
- Tauri 2.x backend with Rust XML parsing

### Technical

- Built with Tauri 2.x, React 19, TypeScript, Vite
- AG Grid Community v35 with themeQuartz
- Tailwind CSS v4
- Rust: quick-xml, rusqlite, strsim
