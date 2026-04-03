---
title: Tag Display Modes
description: How inline formatting tags are shown in the translation grid.
sidebar:
  order: 2
---

Bilingual translation files often contain **inline tags** – markers for formatting like bold, italic, underline, links, and placeholders. These tags must be preserved in the translation to maintain the original document's formatting.

Supervertaler Workbench shows these tags as visual badges in the grid, with three display modes to choose from.

## The three modes

### No Tag Text

Tags are shown as minimal coloured markers with no text:

- ▸ Opening tag
- ◂ Closing tag
- ◆ Standalone tag (placeholder)

This mode gives you the cleanest view of the text, with the smallest visual footprint for tags.

### Partial Tag Text

Tags are shown as numbered badges:

- `{1}` Opening tag 1
- `{/1}` Closing tag 1
- `{2}` Standalone tag 2

The numbers indicate **pairing**: `{1}` and `{/1}` are a matched pair (e.g. bold on/off). This makes it easy to see which tags belong together, especially with nested formatting.

**This is the default mode.**

### Full Tag Text

Tags show their raw display text, such as the original markup:

- `{\b}` (RTF bold on)
- `{\b0}` (RTF bold off)
- `<g id="1">` (XLIFF group tag)

This mode is useful for troubleshooting tag issues or when you need to see exactly what each tag represents.

## Switching modes

Click the **Tags** button in the toolbar to cycle through the three modes:

**No Tag Text** → **Partial Tag Text** → **Full Tag Text** → **No Tag Text** → ...

The current mode is shown on the button label.

## Tag colours

Tags are colour-coded by formatting type:

| Colour | Tag type |
|--------|----------|
| Purple | Bold |
| Blue | Italic |
| Green | Underline |
| Amber | Superscript / subscript |
| Cyan | Link |
| Red | Placeholder |
| Grey | Other formatting |

## How tag pairing works

In **Partial Tag Text** mode, tag numbers are assigned based on nesting order. Each opening tag gets the next number, and each closing tag gets the number of the most recently opened tag. This ensures correct visual pairing:

```
{1} {2} Bold and italic text {/2} {/1}
```

Here, `{1}` and `{/1}` are the outer pair (e.g. bold), and `{2}` and `{/2}` are the inner pair (e.g. italic).

:::note
The visual tag numbers are for display only. When you save the file, the original tag structure is preserved exactly as it was in the source file.
:::
