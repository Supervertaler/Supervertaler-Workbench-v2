---
title: Introduction
description: What is Supervertaler Workbench and who is it for?
sidebar:
  order: 1
---

Supervertaler Workbench 2.0 is a free, open-source translation workbench for professional translators. It is designed to **complement** your existing CAT tool – not replace it.

## What does it do?

Supervertaler Workbench opens bilingual translation files created by CAT tools like Trados Studio, memoQ, and CafeTran Espresso. You can:

- **Edit translations** in a fast, keyboard-friendly grid
- **Use AI** to translate, review, or rephrase segments (OpenAI, Anthropic, DeepL, or local models)
- **Look up terminology** from your glossaries as you translate
- **Search translation memory** for fuzzy matches from previous projects
- **Save back** to the original file format so your CAT tool can read the result

## Who is it for?

Supervertaler Workbench is built for translators who:

- Want AI assistance while translating, without paying for expensive CAT tool add-ons
- Work across multiple CAT tools and want a single, familiar editing environment
- Need a lightweight editor for quick translation tasks outside their main CAT tool
- Prefer open-source tools they can inspect, modify, and trust

## How is it different from the original Supervertaler?

The original Supervertaler Workbench (v1) was built with Python and PyQt6. Version 2.0 is a complete rewrite using Tauri, React, and Rust – resulting in a much faster grid, smaller install size, and a more modern interface.

:::note
Supervertaler Workbench is a companion to CAT tools, not a replacement. It focuses on bilingual file formats (XLIFF, SDLXLIFF, MQXLIFF) and does not handle monolingual document import/export.
:::

## Next steps

- [Installation](/getting-started/installation/) – Download and install
- [Opening a File](/getting-started/opening-a-file/) – Open your first translation file
