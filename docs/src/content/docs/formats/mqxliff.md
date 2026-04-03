---
title: MQXLIFF (memoQ)
description: memoQ MQXLIFF format support.
sidebar:
  order: 4
---

MQXLIFF is memoQ's variant of XLIFF 1.2. It uses standard XLIFF inline tags (`<bpt>`, `<ept>`, `<ph>`) with RTF-style formatting codes as inner content.

## File extensions

- `.mqxliff`

## Inline tag style

memoQ XLIFF uses begin/end paired tags with RTF formatting codes inside:

```xml
<bpt id="1" ctype="bold">{\b}</bpt>Bold text<ept id="1">{\b0}</ept>
```

The `ctype` attribute often indicates the formatting type (bold, italic, etc.). When `ctype` is not present, Supervertaler Workbench analyses the inner text to detect the formatting type.

## What is parsed

- Source and target text from `<trans-unit>` elements
- Inline tags: `<bpt>` / `<ept>` (paired), `<ph>` (placeholder), `<it>` (isolated)
- Tag type detection from `ctype` attributes and RTF codes (`{\b}`, `{\i}`, `{\ul}`, etc.)
- `rid` attributes on `<ept>` tags for correct tag pairing

## Tag pairing

In memoQ XLIFF, `<ept>` elements may have a different `id` from their matching `<bpt>`. The `rid` (reference ID) attribute on the `<ept>` indicates which `<bpt>` it closes. Supervertaler Workbench uses this to display correct tag pairing in the grid.

:::tip
You do not need memoQ installed to open MQXLIFF files in Supervertaler Workbench.
:::
