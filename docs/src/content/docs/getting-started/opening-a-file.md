---
title: Opening a File
description: How to open a bilingual translation file in Supervertaler Workbench.
sidebar:
  order: 3
---

Supervertaler Workbench opens bilingual translation files created by CAT tools. You do not need the original CAT tool installed to edit these files.

## Open a file

1. Click **Open** in the toolbar, or press `Ctrl+O`
2. Select a translation file (`.xliff`, `.sdlxliff`, `.mqxliff`, or other supported format)
3. The file's segments load into the translation grid

The status bar at the bottom shows the project name, source and target languages, and segment count.

## What happens when you open a file

Supervertaler Workbench parses the bilingual file and extracts:

- **Source text** – the original text to be translated
- **Target text** – the existing translation (if any)
- **Inline tags** – formatting markers (bold, italic, links, etc.)
- **Segment status** – whether each segment is new, translated, confirmed, etc.

Segments that already have a translation are marked as **Translated** (blue dot). Empty segments are marked as **New** (grey dot).

:::tip
Inline tags are shown as numbered badges like `{1}` and `{/1}`. You can change how tags are displayed – see [Tag Display Modes](/editor/tag-display-modes/).
:::

## Supported formats

| Format | Extensions | CAT tool |
|--------|-----------|----------|
| XLIFF 1.2 | `.xliff`, `.xlf` | Various |
| SDLXLIFF | `.sdlxliff` | Trados Studio |
| MQXLIFF | `.mqxliff` | memoQ |

For more detail on each format, see [Supported Formats](/formats/supported-formats/).

## Next steps

- [Translation Grid](/editor/translation-grid/) – Learn how the grid works
- [Setting Up API Keys](/getting-started/api-keys/) – Connect to AI translation providers
