---
title: Supported Formats
description: Bilingual file formats supported by Supervertaler Workbench.
sidebar:
  order: 1
---

Supervertaler Workbench opens bilingual translation files created by CAT tools. It does not import monolingual documents (like DOCX or PDF) directly – for that, use your CAT tool to create the bilingual file first.

## Currently supported

| Format | Extensions | CAT tool | Status |
|--------|-----------|----------|--------|
| XLIFF 1.2 | `.xliff`, `.xlf` | Various (CafeTran, Phrase, etc.) | Working |
| SDLXLIFF | `.sdlxliff` | Trados Studio | Working |
| MQXLIFF | `.mqxliff` | memoQ | Working |

## Planned

| Format | Extensions | CAT tool | Notes |
|--------|-----------|----------|-------|
| SDLPPX | `.sdlppx` | Trados Studio | Package format (ZIP containing SDLXLIFF) |
| CafeTran XLIFF | `.xliff` | CafeTran Espresso | XLIFF 1.2 with CafeTran-specific extensions |
| Phrase XLIFF | `.xliff` | Phrase (Memsource) | XLIFF 1.2 with Phrase-specific metadata |

:::note
Supervertaler Workbench focuses on bilingual CAT tool formats. It is designed to complement your CAT tool, not replace it. Use your CAT tool to import the original document, then open the resulting bilingual file in Supervertaler Workbench.
:::
