---
title: XLIFF
description: XLIFF 1.2 support in Supervertaler Workbench.
sidebar:
  order: 2
---

XLIFF (XML Localisation Interchange File Format) is an open standard for exchanging translation data between tools. Supervertaler Workbench supports **XLIFF 1.2**, which is the most widely used version in the translation industry.

## File extensions

- `.xliff`
- `.xlf`

## What is parsed

- Source and target text from `<trans-unit>` elements
- Inline tags: `<bpt>` / `<ept>` (paired), `<ph>` (placeholder), `<it>` (isolated), `<g>` (group), `<x/>` (standalone)
- Source and target language from the `<file>` element
- Tag type detection from `ctype` attributes and inner content

## Inline tag handling

XLIFF inline tags represent formatting in the original document. Supervertaler Workbench extracts these as structured data and displays them as visual badges in the grid. The plain text is kept separate for editing and TM matching.

For example, a segment with bold text in memoQ XLIFF:

```xml
<source>Click <bpt id="1">{\b}</bpt>here<ept id="1">{\b0}</ept> to continue</source>
```

Is displayed as:

> Click **{1}** here **{/1}** to continue

See [Tag Display Modes](/editor/tag-display-modes/) for more on how tags are visualised.
