---
title: TM Basics
description: How translation memory works in Supervertaler Workbench.
---

:::caution
Translation memory is under active development. This page describes the planned functionality.
:::

Supervertaler Workbench includes a built-in translation memory (TM) that stores previously translated segments and suggests matches for new segments.

## How it works

When you open a translation file, Supervertaler Workbench searches the TM for each source segment. If a similar segment was translated before, it appears in the **TM Results** panel on the right side of the screen, along with a match percentage.

## Match types

| Match | Description |
|-------|-------------|
| 100% | Exact match – identical source text |
| 75–99% | Fuzzy match – similar but not identical |
| Below 75% | Not shown by default (configurable) |

## TM storage

Translation memories are stored as local SQLite databases. They support full-text search for fast lookup, even with large TMs.
