---
title: SDLXLIFF (Trados)
description: Trados Studio SDLXLIFF format support.
sidebar:
  order: 3
---

SDLXLIFF is Trados Studio's proprietary variant of XLIFF 1.2. It contains the same core structure as standard XLIFF but adds SDL-specific namespaces and metadata.

## File extensions

- `.sdlxliff`

## Key differences from standard XLIFF

| Feature | Standard XLIFF | SDLXLIFF |
|---------|---------------|----------|
| Source text | `<source>` | `<seg-source>` (preferred) or `<source>` |
| Inline formatting | `<bpt>` / `<ept>` | `<g>` tags wrapping content |
| Placeholders | `<ph>`, `<x/>` | `<x/>` with SDL-specific attributes |
| Segment IDs | Sequential numbers | UUIDs |
| Confirmation status | Not standard | `<sdl:seg-defs>` with confirmation levels |

## What is parsed

- Source text from `<seg-source>` (falls back to `<source>` if not present)
- Target text from `<target>`
- Inline tags: `<g>` (formatting groups), `<x/>` (standalone placeholders)
- Source and target language
- Empty/structural trans-units are skipped

:::tip
You do not need Trados Studio installed to open SDLXLIFF files in Supervertaler Workbench. The parser reads the XML directly.
:::
