# Supervertaler Workbench 2.0 – Rewrite Architecture & Specification

> **Purpose:** This document is a comprehensive brief for Claude Code to scaffold and implement a full rewrite of Supervertaler Workbench. It contains the technology decision rationale, architectural design, feature mapping from the existing Python/PyQt6 codebase, and implementation guidance.
>
> **Author:** Michael Beijer, with research assistance from Claude (Anthropic)
> **Date:** April 2026
> **Target lifespan:** 10+ years

---

## 1. Executive Summary

Supervertaler Workbench is an open-source, cross-platform CAT (Computer-Assisted Translation) tool currently built with Python 3.12 + PyQt6. It has ~1,700 commits, 370+ releases, and a rich feature set including multi-LLM AI translation, translation memory, glossary management, Superlookup concordance, voice commands, and an Okapi Framework sidecar for industrial-strength file extraction.

**The problem:** Python's GIL, interpreted execution, and PyQt6's rendering model create a performance ceiling for the translation grid – the core UI where translators spend 90%+ of their time. Segment navigation, grid re-rendering, and concurrent operations (TM lookup, terminology highlighting, API calls) cause perceptible lag that degrades the professional workflow.

**The solution:** Rewrite the application using **Tauri 2.x + TypeScript + React** with a Rust backend for performance-critical operations. This provides native-level performance, tiny bundle sizes (under 20 MB vs 300+ MB PyInstaller bundles), and access to the world's largest UI component ecosystem.

---

## 2. Technology Decision: Why Tauri + TypeScript + React

### 2.1 Candidates Evaluated

| Framework | Language | Grid Perf | Ecosystem | Cross-platform | Vibe-coding | Bundle Size | Verdict |
|-----------|----------|-----------|-----------|----------------|-------------|-------------|---------|
| Python + PyQt6 (current) | Python | Poor (GIL) | Medium | Yes | Good | ~300 MB | Reject – ceiling reached |
| Electron + React | TypeScript | Excellent | Massive | Yes | Excellent | ~120 MB | Strong – but bloated |
| **Tauri 2.x + React** | **TS + Rust** | **Excellent** | **Massive** | **Yes** | **Excellent** | **~10–20 MB** | **Winner** |
| C# + Avalonia UI | C# | Very Good | Growing | Yes | Good | ~40 MB | Runner-up |
| C++ + Qt6 | C++ | Native | Good | Yes | Poor | ~30 MB | Reject – dev speed |

### 2.2 Why Tauri over Electron

- **Memory:** Tauri apps idle at ~30–40 MB vs Electron's 200–300 MB
- **Bundle size:** Under 10 MB vs 100+ MB (no bundled Chromium)
- **Startup time:** Under 0.5s vs 1–2s
- **Security:** Opt-in permissions model (locked down by default)
- **Rust backend:** Native file I/O, process management (Okapi sidecar), SQLite – all at compiled speed
- **Mobile potential:** Tauri 2.0 supports iOS/Android (future option)
- **Tauri 2.0 is production-ready** as of October 2024 (stable release) with a completed external security audit, plugin architecture, and rewritten IPC layer

### 2.3 Why Tauri over Avalonia (C#)

Avalonia was the runner-up and has real merits – especially given the existing C# codebase for Supervertaler for Trados. However:

- **Ecosystem:** The web/React ecosystem for grids, editors, and UI components vastly exceeds what's available in Avalonia. AG Grid alone has more features than Avalonia's DataGrid.
- **Vibe-coding:** Claude is strongest in TypeScript/React. C# is good but TS is better for rapid AI-assisted development.
- **UI flexibility:** Web technologies (HTML/CSS/JS) offer unlimited styling and layout control. XAML is powerful but more constrained.
- **Component reuse:** The same web frontend could potentially run as a web app in future (SaaS version).
- **DataGrid maturity:** Avalonia's DataGrid is functional but immature compared to AG Grid, RevoGrid, or TanStack Table.

### 2.4 Acknowledged Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| WebView inconsistencies across platforms | Tauri on Windows uses WebView2 (Chromium-based); macOS/Linux use WebKit. For a translation grid, WebKit differences are minimal. Test on all three platforms in CI. |
| Rust learning curve | Keep Rust usage minimal – just the backend commands for file I/O, Okapi sidecar management, and SQLite. Most logic stays in TypeScript. Claude is adequate at Rust for this scope. |
| Tauri ecosystem maturity | Tauri 2.0 is stable; sidecar support exists; plugin system is extensible. Pin dependencies to specific versions. |
| AG Grid licensing | AG Grid Community (MIT) covers all needed features. Enterprise license only needed for pivot tables etc. which are not relevant here. Alternative: RevoGrid (MIT, handles 400k+ rows) or TanStack Table (MIT, headless). |

---

## 3. Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                    TAURI SHELL (Rust)                     │
│                                                          │
│  ┌────────────┐  ┌────────────┐  ┌───────────────────┐  │
│  │ File I/O   │  │ SQLite DB  │  │ Okapi Sidecar     │  │
│  │ Commands   │  │ (rusqlite) │  │ Process Manager   │  │
│  └─────┬──────┘  └─────┬──────┘  └────────┬──────────┘  │
│        │               │                  │              │
│        └───────────┬───┘──────────────────┘              │
│                    │ IPC (invoke / events)                │
│  ┌─────────────────┴───────────────────────────────────┐ │
│  │              WEBVIEW (React + TypeScript)             │ │
│  │                                                      │ │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌────────┐ │ │
│  │  │ Grid     │ │ TermLens │ │ LLM      │ │ TM     │ │ │
│  │  │ (AG Grid)│ │ Panel    │ │ Clients  │ │ Engine │ │ │
│  │  └──────────┘ └──────────┘ └──────────┘ └────────┘ │ │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌────────┐ │ │
│  │  │ Settings │ │ Prompt   │ │ Super    │ │ QA     │ │ │
│  │  │ Manager  │ │ Library  │ │ Lookup   │ │ Checks │ │ │
│  │  └──────────┘ └──────────┘ └──────────┘ └────────┘ │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                          │
│  ┌──────────────────────────────────────────────────────┐ │
│  │              SIDECAR: Okapi Framework (Java)          │ │
│  │              (Bundled minimal JRE, ~44 MB)            │ │
│  └──────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

### 3.1 Layer Responsibilities

**Rust Backend (Tauri Core Process):**
- File system operations (read/write XLIFF, SDLXLIFF, SDLPPX, DOCX, TMX, TBX, TSV, SVProj)
- SQLite database management (TM, termbase, settings, project state) via `rusqlite`
- Okapi sidecar process lifecycle management (start/stop/health-check the Java process)
- System-level operations: clipboard, global hotkeys, tray icon, auto-updater
- Heavy data operations: TM fuzzy matching (Levenshtein/edit distance in Rust = blazing fast)

**TypeScript/React Frontend (WebView):**
- All UI rendering (grid, panels, dialogs, settings)
- LLM API client calls (OpenAI, Anthropic, Google, Mistral, xAI, Ollama – all REST/HTTP)
- Prompt management and template rendering
- State management (Zustand or Jotai – lightweight, performant)
- Keyboard shortcut handling (in-app shortcuts)

**Okapi Sidecar (Java, unchanged):**
- File extraction and merging (DOCX, PPTX, XLSX, HTML, etc.)
- REST API on localhost (current architecture carries over directly)

### 3.2 IPC Design

Tauri 2.0's IPC uses custom protocols (not JSON serialisation), supporting raw payloads for large data transfers. Use:

```rust
// Rust side – define commands
#[tauri::command]
async fn load_project(path: String) -> Result<ProjectData, String> { ... }

#[tauri::command]
async fn get_tm_matches(source: String, threshold: f32) -> Result<Vec<TmMatch>, String> { ... }

#[tauri::command]
async fn save_segment(segment_id: u64, target: String) -> Result<(), String> { ... }
```

```typescript
// TypeScript side – call commands
import { invoke } from '@tauri-apps/api/core';

const project = await invoke<ProjectData>('load_project', { path: '/path/to/file.xliff' });
const matches = await invoke<TmMatch[]>('get_tm_matches', { source: 'Hello world', threshold: 0.7 });
await invoke('save_segment', { segmentId: 42, target: 'Hallo wereld' });
```

Use Tauri's **event system** for real-time updates (batch translation progress, Okapi sidecar status):

```rust
// Rust side – emit events
app_handle.emit("batch-progress", BatchProgress { current: 5, total: 100 })?;
```

```typescript
// TypeScript side – listen
import { listen } from '@tauri-apps/api/event';
listen<BatchProgress>('batch-progress', (event) => {
  updateProgressBar(event.payload.current, event.payload.total);
});
```

---

## 4. Feature Map: Python → Tauri/React

This maps every major feature from the current Python codebase to its implementation strategy in the new stack.

### 4.1 Core Translation Grid

**Current (Python):** `Supervertaler.py` (monolithic, very large file) + PyQt6 QTableWidget with custom cell widgets. Performance bottleneck: QTableWidget re-renders entire visible area on segment change; GIL blocks UI thread during TM lookup and API calls.

**New (React):** AG Grid Community (MIT) or RevoGrid (MIT).

Key requirements:
- Virtual scrolling (only render visible rows – handles 10,000+ segments effortlessly)
- Editable cells with rich text (inline tags like `<b>`, `<i>`, `<cf>`)
- Custom cell renderers (source column read-only, target column editable, status icons, match percentage)
- Row selection (single + multi-select with Ctrl/Shift+Click)
- Keyboard navigation (Tab/Enter to move between segments, Ctrl+Enter to confirm)
- Variable row heights (auto-height based on content length)
- Column pinning (segment number always visible)
- Filtering and sorting by status, match percentage, segment content

**Implementation notes:**
- Use AG Grid's `cellRenderer` for custom source/target display with inline tag highlighting
- Use AG Grid's `cellEditor` for the target column with a custom rich-text editor component
- Segment status (Draft, Translated, Confirmed, Approved, Rejected, Locked) → AG Grid row styling
- TM match percentage → colour-coded badges in a dedicated column
- Performance target: < 16ms per segment navigation (one frame at 60fps)

### 4.2 TermLens (Inline Terminology Display)

**Current (Python):** `modules/termlens_widget.py` – `TermLensWidget` class. Highlights matched terms in the source segment with colour-coded priority levels (1–99). Hover tooltips show translation, priority, forbidden status. Double-click inserts term translation at cursor.

**New (React):** Custom React component rendered in a panel below or beside the grid.
- Term matching runs in Rust (SQLite FTS5 for fast lookups against the termbase)
- Matched terms sent to frontend via IPC
- Render highlighted terms using `<span>` elements with CSS classes for priority colours
- Tooltip on hover (React Tooltip or Floating UI)
- Click to insert at cursor position in the active target cell editor

### 4.3 Translation Memory

**Current (Python):** `modules/database_manager.py` – SQLite-based TM with fuzzy matching. TMX import/export.

**New (Rust backend):**
- SQLite database with `rusqlite` for storage
- Fuzzy matching algorithm implemented in Rust (edit distance / Levenshtein) – dramatically faster than Python
- TMX import/export as Rust commands
- Results sent to frontend as structured data
- Frontend: `TranslationResultsPanel` React component (equivalent to `modules/translation_results_panel.py`)

### 4.4 LLM Integration (Multi-Provider AI Translation)

**Current (Python):** `modules/llm_clients.py` – abstraction layer for OpenAI, Anthropic, Google, Mistral, xAI, Ollama.

**New (TypeScript):** Keep LLM client logic in TypeScript (frontend layer) since these are all HTTP REST APIs. Benefits:
- Streaming responses render directly in the UI without IPC overhead
- Easier to add new providers (just add a new client class)
- Shared code with potential future web version

```typescript
// LLM client abstraction
interface LLMClient {
  translate(request: TranslateRequest): AsyncIterable<string>;
  complete(request: CompletionRequest): AsyncIterable<string>;
}

class AnthropicClient implements LLMClient { ... }
class OpenAIClient implements LLMClient { ... }
class GoogleClient implements LLMClient { ... }
class OllamaClient implements LLMClient { ... }
```

Note: API keys stored securely via Tauri's secure storage or the OS keychain (not plaintext files).

### 4.5 Prompt Library

**Current (Python):** `.svprompt` files with YAML frontmatter + Markdown body. Variables: `{{SOURCE_LANGUAGE}}`, `{{TARGET_LANGUAGE}}`, `{{SOURCE_SEGMENT}}`, etc. Shared with Supervertaler for Trados.

**New (TypeScript):** Same `.svprompt` format (maintain compatibility with Trados plugin). Parse YAML frontmatter with `gray-matter`, render Markdown with `marked` or `remark`. Template variable substitution in TypeScript.

### 4.6 Superlookup (Concordance Search)

**Current (Python):** Unified search across TM, glossaries, MT engines, and web resources.

**New:** Split between Rust (TM/glossary SQLite queries) and TypeScript (MT engine HTTP calls, web resource fetching). Results aggregated in a React panel with tabbed display.

### 4.7 File Import/Export

**Current (Python):** Native importers for XLIFF, SDLXLIFF, SDLPPX/SDLRPX, bilingual DOCX/RTF, Déjà Vu RTF, CafeTran XLIFF, Phrase XLIFF, plain text, Markdown, PDF. Okapi sidecar for DOCX round-trip with formatting preservation.

**New (Rust + Okapi):**
- XML parsing (XLIFF, SDLXLIFF, SDLPPX) in Rust using `quick-xml` – much faster than Python's XML parsers
- Okapi sidecar (Java) continues to handle DOCX/PPTX/etc. extraction and merging – architecture identical to current
- Tauri's sidecar feature manages the Java process lifecycle natively
- File chooser dialogs via Tauri's built-in dialog plugin

### 4.8 Settings & Preferences

**Current (Python):** `settings/settings.json` + satellite files (`themes.json`, `shortcuts.json`, etc.).

**New:** Same JSON format (migration-compatible). Settings read/written via Rust commands. Frontend settings UI as a React modal/panel with tabs.

### 4.9 Voice Commands (OpenAI Whisper)

**Current (Python):** OpenAI Whisper API for voice dictation.

**New (TypeScript):** Browser's `MediaRecorder` API to capture audio → send to OpenAI Whisper API via HTTP. Alternative: Tauri plugin for native audio capture if WebView audio APIs are insufficient.

### 4.10 Global Hotkeys (QuickTrans, Superlookup, QuickLauncher)

**Current (Python + AHK):** `supervertaler_hotkeys.ahk` for Windows; `pynput` for macOS.

**New (Rust):** Tauri's `global-shortcut` plugin for cross-platform global hotkey registration. No more AutoHotkey dependency on Windows.

### 4.11 Spellcheck

**Current (Python):** hunspell integration.

**New:** WebView's built-in spellcheck (Chromium/WebKit both support it natively). Alternatively, use `nspell` (JavaScript hunspell port) for consistent cross-platform behaviour.

### 4.12 Superbench (Translation Quality Benchmarking)

**Current (Python):** Runs test translations across multiple LLM providers/models and compares results.

**New (TypeScript):** Direct port – this is primarily LLM API calls + result comparison/display. React component for the benchmark results table.

### 4.13 QuickTrans (System-Wide Translation Popup)

**Current (Python):** Global hotkey triggers a popup with instant translations from multiple sources.

**New (Tauri):** Tauri supports creating additional windows. Register global shortcut → read clipboard → invoke LLM/MT translations → display results in a small overlay window. Tauri's multi-window support handles this cleanly.

### 4.14 AutoHotkey Integration

**Current:** `supervertaler_hotkeys.ahk` – Windows-only AHK script for global hotkeys.

**New:** Eliminated. Tauri's `global-shortcut` plugin replaces this entirely on all platforms.

---

## 5. Project Structure

```
supervertaler-workbench-v2/
├── src-tauri/                     # Rust backend
│   ├── src/
│   │   ├── main.rs                # Tauri entry point
│   │   ├── commands/              # IPC command handlers
│   │   │   ├── mod.rs
│   │   │   ├── project.rs         # Open/save/close project
│   │   │   ├── segments.rs        # CRUD segment data
│   │   │   ├── tm.rs              # Translation memory operations
│   │   │   ├── termbase.rs        # Terminology database
│   │   │   ├── import_export.rs   # File import/export (XLIFF, SDLXLIFF, etc.)
│   │   │   ├── okapi.rs           # Okapi sidecar management
│   │   │   └── settings.rs        # Read/write settings
│   │   ├── db/                    # Database schemas and migrations
│   │   │   ├── mod.rs
│   │   │   ├── schema.rs
│   │   │   └── migrations/
│   │   ├── parsers/               # File format parsers
│   │   │   ├── xliff.rs
│   │   │   ├── sdlxliff.rs
│   │   │   ├── sdlppx.rs
│   │   │   ├── tmx.rs
│   │   │   └── tbx.rs
│   │   └── matching/              # TM fuzzy matching engine
│   │       ├── levenshtein.rs
│   │       └── tm_search.rs
│   ├── Cargo.toml
│   ├── tauri.conf.json            # Tauri configuration
│   └── icons/                     # App icons
├── src/                           # React frontend
│   ├── App.tsx                    # Root component
│   ├── main.tsx                   # Entry point
│   ├── components/
│   │   ├── grid/                  # Translation grid
│   │   │   ├── TranslationGrid.tsx
│   │   │   ├── SourceCellRenderer.tsx
│   │   │   ├── TargetCellEditor.tsx
│   │   │   ├── StatusCellRenderer.tsx
│   │   │   └── MatchPercentageBadge.tsx
│   │   ├── termlens/              # TermLens panel
│   │   │   ├── TermLensPanel.tsx
│   │   │   └── TermHighlight.tsx
│   │   ├── tm/                    # Translation memory panel
│   │   │   └── TMResultsPanel.tsx
│   │   ├── superlookup/           # Concordance search
│   │   │   └── SuperlookupPanel.tsx
│   │   ├── llm/                   # AI translation UI
│   │   │   ├── AIAssistantPanel.tsx
│   │   │   ├── BatchTranslateDialog.tsx
│   │   │   └── PromptEditor.tsx
│   │   ├── settings/              # Settings panels
│   │   │   ├── SettingsDialog.tsx
│   │   │   ├── AISettingsTab.tsx
│   │   │   ├── GeneralSettingsTab.tsx
│   │   │   └── ShortcutsTab.tsx
│   │   ├── quicktrans/            # QuickTrans popup window
│   │   │   └── QuickTransWindow.tsx
│   │   ├── superbench/            # Benchmarking UI
│   │   │   └── SuperbenchPanel.tsx
│   │   └── layout/                # App layout shell
│   │       ├── AppLayout.tsx
│   │       ├── MenuBar.tsx
│   │       ├── StatusBar.tsx
│   │       └── PanelLayout.tsx
│   ├── hooks/                     # Custom React hooks
│   │   ├── useSegments.ts
│   │   ├── useTM.ts
│   │   ├── useTermbase.ts
│   │   ├── useLLM.ts
│   │   └── useShortcuts.ts
│   ├── services/                  # Business logic
│   │   ├── llm/
│   │   │   ├── LLMClient.ts       # Interface
│   │   │   ├── AnthropicClient.ts
│   │   │   ├── OpenAIClient.ts
│   │   │   ├── GoogleClient.ts
│   │   │   ├── MistralClient.ts
│   │   │   ├── XAIClient.ts
│   │   │   └── OllamaClient.ts
│   │   ├── prompt/
│   │   │   ├── PromptParser.ts    # .svprompt YAML+MD parser
│   │   │   └── PromptRenderer.ts  # Variable substitution
│   │   └── mt/
│   │       ├── GoogleTranslate.ts
│   │       ├── DeepL.ts
│   │       └── MyMemory.ts
│   ├── store/                     # State management (Zustand)
│   │   ├── projectStore.ts
│   │   ├── segmentStore.ts
│   │   ├── settingsStore.ts
│   │   └── uiStore.ts
│   ├── types/                     # TypeScript type definitions
│   │   ├── segment.ts
│   │   ├── project.ts
│   │   ├── tm.ts
│   │   ├── termbase.ts
│   │   └── llm.ts
│   └── styles/                    # CSS / Tailwind
│       ├── globals.css
│       └── grid.css
├── okapi-sidecar/                 # Java Okapi sidecar (unchanged from current repo)
│   ├── ...
│   └── bundled-jre/
├── prompts/                       # Shared .svprompt library
├── package.json
├── tsconfig.json
├── vite.config.ts                 # Vite bundler config
├── tailwind.config.ts
└── CLAUDE.md                      # Claude Code reference (updated for new stack)
```

---

## 6. Data Model

### 6.1 Segment

```typescript
interface Segment {
  id: number;
  sourceText: string;
  targetText: string;
  status: 'new' | 'draft' | 'translated' | 'confirmed' | 'approved' | 'rejected' | 'locked';
  matchPercentage: number | null;        // TM match % (0–100)
  matchOrigin: string | null;            // TM match origin description
  sourceInlineTags: InlineTag[];         // Parsed inline formatting tags
  targetInlineTags: InlineTag[];
  segmentNumber: number;                 // Display number (1-indexed)
  notes: string | null;
  createdBy: string | null;
  modifiedBy: string | null;
  modifiedAt: string | null;
  // SDLXLIFF-specific
  sdlxliffConfirmation?: string;         // Confirmed | ApprovedTranslation | etc.
  sdlxliffComments?: Comment[];
}

interface InlineTag {
  type: 'b' | 'i' | 'u' | 's' | 'sup' | 'sub' | 'cf' | 'placeholder';
  id: string;
  content?: string;                      // For cf: colour value; for placeholder: original code
  position: number;                      // Character offset in text
}

interface Comment {
  author: string;
  date: string;
  text: string;
}
```

### 6.2 Project

```typescript
interface Project {
  path: string;                          // .svproj file path
  name: string;
  sourceLanguage: string;
  targetLanguage: string;
  segments: Segment[];
  sourceFile: SourceFile;
  tmDatabases: string[];                 // Paths to TM SQLite files
  termbases: string[];                   // Paths to termbase files
  settings: ProjectSettings;
}

interface SourceFile {
  originalPath: string;
  format: 'xliff' | 'sdlxliff' | 'sdlppx' | 'docx' | 'mqxliff' | 'dejavu_rtf' | 'text' | 'markdown';
  okapiManifest?: string;               // Path to Okapi manifest for round-trip
}
```

### 6.3 TM Entry

```typescript
interface TMEntry {
  id: number;
  sourceText: string;
  targetText: string;
  sourceLanguage: string;
  targetLanguage: string;
  createdBy: string;
  createdAt: string;
  modifiedAt: string;
  context?: string;
  origin?: string;                       // Which TM file this came from
}
```

### 6.4 Term

```typescript
interface Term {
  id: number;
  sourceTerm: string;
  targetTerm: string;
  priority: number;                      // 1–99
  forbidden: boolean;
  notes: string | null;
  domain: string | null;
  termbaseId: number;
}
```

---

## 7. Key Implementation Details

### 7.1 Grid Performance Strategy

The core performance improvement comes from three factors:

1. **DOM virtualisation:** AG Grid only renders visible rows (typically 20–40 at a time). Even with 10,000 segments, only ~40 DOM nodes exist. PyQt's QTableWidget renders all visible cell widgets.

2. **No GIL:** All Rust operations (TM lookup, file I/O, segment saving) run on native threads without blocking the UI. In Python, the GIL means TM lookups block grid rendering.

3. **Incremental updates:** AG Grid supports surgical row updates via `api.applyTransaction()`. Changing one segment's status updates only that row's DOM, not the entire grid.

```typescript
// When user confirms a segment – only that row updates
const onSegmentConfirm = (segmentId: number) => {
  gridApi.applyTransaction({
    update: [{ ...segment, status: 'confirmed' }]
  });
};
```

### 7.2 Tag Handling in the Editor

Translation segments contain inline formatting tags (`<b>`, `<i>`, `<cf color="#FF0000">`, etc.). The target cell editor must:

- Display tags as styled, non-editable badges (like memoQ/Trados)
- Allow tag insertion via keyboard shortcuts (Ctrl+B, Ctrl+I) or picking from a tag list
- Preserve tag order and pairing during editing
- Validate tags on segment confirmation

Implement using a custom contentEditable editor or a lightweight rich-text component (e.g., Tiptap with a custom tag extension). Do NOT use a full WYSIWYG editor – this needs to be fast and focused.

### 7.3 Okapi Sidecar Integration

The existing Okapi sidecar architecture carries over unchanged. Tauri provides native sidecar support:

```json
// tauri.conf.json
{
  "bundle": {
    "externalBin": ["okapi-sidecar/okapi-server"]
  }
}
```

```rust
// Rust side – manage the Okapi sidecar
use tauri::api::process::Command;

#[tauri::command]
async fn start_okapi() -> Result<(), String> {
    Command::new_sidecar("okapi-server")
        .expect("failed to create sidecar command")
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}
```

### 7.4 SQLite for TM and Termbase

Use `rusqlite` with FTS5 (Full-Text Search) for fast concordance lookups:

```sql
-- TM table with FTS5
CREATE VIRTUAL TABLE tm_fts USING fts5(
  source_text,
  target_text,
  content='tm_entries',
  content_rowid='id'
);

-- Concordance search
SELECT * FROM tm_fts WHERE tm_fts MATCH 'patent AND adhesive';
```

Fuzzy matching (Levenshtein distance) implemented in Rust for TM matching – orders of magnitude faster than Python.

### 7.5 State Management

Use **Zustand** (lightweight, TypeScript-first, no boilerplate):

```typescript
import { create } from 'zustand';

interface SegmentStore {
  segments: Segment[];
  activeSegmentId: number | null;
  setActiveSegment: (id: number) => void;
  updateSegment: (id: number, updates: Partial<Segment>) => void;
}

const useSegmentStore = create<SegmentStore>((set) => ({
  segments: [],
  activeSegmentId: null,
  setActiveSegment: (id) => set({ activeSegmentId: id }),
  updateSegment: (id, updates) => set((state) => ({
    segments: state.segments.map(s => s.id === id ? { ...s, ...updates } : s)
  })),
}));
```

### 7.6 Keyboard Shortcuts

Use a custom shortcut manager integrated with Tauri's accelerator system:

- **In-app shortcuts:** React event handlers (e.g., `Ctrl+Enter` to confirm segment)
- **Global shortcuts:** Tauri `global-shortcut` plugin (e.g., `Ctrl+Alt+L` for Superlookup from any app)
- **Configurable:** Shortcuts stored in `settings/shortcuts.json`, editable from Settings UI

---

## 8. Migration & Compatibility

### 8.1 File Format Compatibility

All existing file formats must be supported from day one:
- `.svproj` project files (same JSON format)
- `.svprompt` prompt files (same YAML+MD format, shared with Trados plugin)
- `settings.json` and satellite settings files (same structure)
- TMX import/export (same standard format)
- All supported CAT tool formats (XLIFF, SDLXLIFF, SDLPPX, etc.)

### 8.2 Prompt Library Sharing

The `.svprompt` prompt library must remain compatible with Supervertaler for Trados. Both apps read from a shared `prompts/` directory. The YAML frontmatter format and template variable names must be identical.

### 8.3 Gradual Migration Plan

1. **Phase 1 – Core grid + file import:** Scaffold Tauri app, implement XLIFF/SDLXLIFF import, build the translation grid with AG Grid, implement basic segment editing. This alone should demonstrate the performance improvement.

2. **Phase 2 – TM + Termbase:** Port SQLite TM engine to Rust, implement fuzzy matching, build TermLens panel.

3. **Phase 3 – LLM Integration:** Port all LLM clients to TypeScript, implement batch translate, build AI Assistant panel.

4. **Phase 4 – Full feature parity:** Superlookup, QuickTrans, voice commands, Superbench, Okapi sidecar, all import/export formats, settings UI.

5. **Phase 5 – Polish & release:** Auto-updater, installers (MSI/DMG/AppImage), documentation, testing.

---

## 9. Build & Distribution

### 9.1 Development

```bash
# Prerequisites
# - Node.js 20+ (for frontend)
# - Rust toolchain (rustup)
# - System WebView (WebView2 on Windows, WebKitGTK on Linux)

# Setup
npm install
cd src-tauri && cargo build  # First build compiles Rust (slow, subsequent builds are fast)

# Development with hot-reload
npm run tauri dev
```

### 9.2 Building for Distribution

```bash
# Build for current platform
npm run tauri build

# Output:
# Windows: src-tauri/target/release/bundle/msi/Supervertaler_x.y.z.msi
# macOS:   src-tauri/target/release/bundle/dmg/Supervertaler_x.y.z.dmg
# Linux:   src-tauri/target/release/bundle/appimage/Supervertaler_x.y.z.AppImage
```

Expected bundle sizes: 10–20 MB (+ ~44 MB for Okapi JRE sidecar).

### 9.3 Auto-Updater

Tauri includes a built-in updater plugin. Configure update endpoint:

```json
// tauri.conf.json
{
  "plugins": {
    "updater": {
      "endpoints": ["https://releases.supervertaler.com/{{target}}/{{arch}}/{{current_version}}"]
    }
  }
}
```

---

## 10. Key Dependencies

### Frontend (package.json)
```json
{
  "dependencies": {
    "@tauri-apps/api": "^2.x",
    "@tauri-apps/plugin-dialog": "^2.x",
    "@tauri-apps/plugin-global-shortcut": "^2.x",
    "@tauri-apps/plugin-clipboard-manager": "^2.x",
    "@tauri-apps/plugin-updater": "^2.x",
    "react": "^19.x",
    "react-dom": "^19.x",
    "ag-grid-react": "^33.x",
    "ag-grid-community": "^33.x",
    "zustand": "^5.x",
    "gray-matter": "^4.x",
    "marked": "^15.x",
    "@floating-ui/react": "^0.x"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.x",
    "typescript": "^5.x",
    "vite": "^6.x",
    "@vitejs/plugin-react": "^4.x",
    "tailwindcss": "^4.x"
  }
}
```

### Backend (Cargo.toml)
```toml
[dependencies]
tauri = { version = "2", features = ["tray-icon"] }
tauri-plugin-dialog = "2"
tauri-plugin-global-shortcut = "2"
tauri-plugin-clipboard-manager = "2"
tauri-plugin-updater = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rusqlite = { version = "0.32", features = ["bundled", "fts5"] }
quick-xml = "0.37"
tokio = { version = "1", features = ["full"] }
zip = "2"                          # For SDLPPX (ZIP archive) handling
strsim = "0.11"                    # String similarity (Levenshtein, etc.)
```

---

## 11. CLAUDE.md for the New Project

Place this as `CLAUDE.md` in the project root for Claude Code sessions:

```markdown
# Supervertaler Workbench v2 – Claude Code Reference

> Cross-platform AI translation workbench. Tauri 2.x + React + TypeScript frontend, Rust backend.

## Quick Commands
- `npm run tauri dev` – development with hot-reload
- `npm run tauri build` – production build
- `cargo test` – run Rust tests
- `npm test` – run frontend tests

## Key Paths
| What | Path |
|------|------|
| Tauri config | `src-tauri/tauri.conf.json` |
| Rust commands | `src-tauri/src/commands/` |
| React components | `src/components/` |
| LLM clients | `src/services/llm/` |
| State stores | `src/store/` |
| Type definitions | `src/types/` |
| Prompts | `prompts/` |

## Architecture
- **Rust backend:** File I/O, SQLite (TM + termbase), Okapi sidecar management, TM fuzzy matching
- **React frontend:** All UI, LLM API calls, prompt management, state management (Zustand)
- **Okapi sidecar:** Java process for DOCX/PPTX extraction (managed by Rust)

## Conventions
- Use en dashes (–) not em dashes (—) in all user-facing text
- British English for UI text
- TypeScript strict mode
- Tailwind CSS for styling
- AG Grid Community for the translation grid
- Zustand for state management
- .svprompt files use YAML frontmatter + Markdown body

## Pitfalls
1. Tauri IPC is async – always `await invoke()`
2. WebView CORS: route localhost API calls through Rust commands in production
3. AG Grid: use `applyTransaction` for row updates, not full data replacement
4. Okapi sidecar: test on all platforms (Java process paths differ)
5. Settings files must remain JSON-compatible with v1 format
```

---

## 12. Summary of Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| App framework | Tauri 2.x | Lightweight, fast, cross-platform, Rust backend, production-ready |
| Frontend framework | React 19 | Largest ecosystem, best vibe-coding support, AG Grid integration |
| Language (frontend) | TypeScript | Type safety, excellent Claude Code support |
| Language (backend) | Rust | Performance for TM matching, file parsing, SQLite; Tauri requires it |
| Grid component | AG Grid Community | Virtual scrolling, editable cells, filtering, MIT license |
| State management | Zustand | Lightweight, TypeScript-first, no boilerplate |
| Database | SQLite (rusqlite) | Same as current, with FTS5 for concordance, Rust-native performance |
| Styling | Tailwind CSS | Utility-first, rapid UI development, consistent styling |
| Bundler | Vite | Fast HMR, Tauri integration, standard for React projects |
| File parsing | quick-xml (Rust) | Fast XML parsing for XLIFF/SDLXLIFF; replaces Python's xml.etree |
| TM matching | Rust (strsim) | Levenshtein/edit distance at compiled speed |
| Okapi sidecar | Java (unchanged) | Proven architecture, Tauri sidecar support |
| Auto-updater | Tauri updater plugin | Built-in, cross-platform |
| Global hotkeys | Tauri global-shortcut plugin | Replaces AHK + pynput |
