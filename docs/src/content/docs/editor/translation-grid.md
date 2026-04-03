---
title: Translation Grid
description: The main translation editor in Supervertaler Workbench.
sidebar:
  order: 1
---

The translation grid is the main workspace in Supervertaler Workbench. It shows all segments from your bilingual file in a table with source text, target text, status, and match information.

## Grid columns

| Column | Description |
|--------|-------------|
| **#** | Segment number |
| **Status** | Colour-coded dot showing the segment's translation status |
| **Source** | The original text (read-only) |
| **Target** | The translation (editable) |
| **%** | Match percentage from translation memory |

## Segment statuses

Each segment has a status shown as a coloured dot:

| Colour | Status | Meaning |
|--------|--------|---------|
| Grey | New | No translation yet |
| Yellow | Draft | Translation started but not finished |
| Blue | Translated | Has a translation |
| Green | Confirmed | Translation reviewed and confirmed |
| Dark green | Approved | Translation approved (final) |
| Red | Rejected | Translation rejected during review |
| Dark grey | Locked | Segment is locked and cannot be edited |

## Editing a segment

1. Click on a cell in the **Target** column
2. Type your translation
3. Press `Enter` or `Tab` to move to the next segment

When you start typing in an empty segment, its status automatically changes from **New** to **Draft**.

## Right panel

When enabled, the right side of the screen shows:

- **TM Results** – Translation memory matches for the active segment
- **TermLens** – Terminology matches from your glossaries

You can show or hide these panels from the **View** menu.
