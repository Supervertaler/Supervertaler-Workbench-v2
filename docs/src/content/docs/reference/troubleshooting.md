---
title: Troubleshooting
description: Solutions to common issues with Supervertaler Workbench.
---

## File won't open

**"Unsupported file format"**

Supervertaler Workbench only opens bilingual CAT tool formats. Check that your file has one of the supported extensions: `.xliff`, `.xlf`, `.sdlxliff`, `.mqxliff`.

If you have a monolingual document (`.docx`, `.pdf`, etc.), open it in your CAT tool first to create a bilingual file.

**"Failed to read file"**

- Check that the file exists and is not locked by another application
- Make sure the file is valid XML (not corrupted)

## Grid shows no segments

If the grid is empty after opening a file:

- Check the status bar – does it show a segment count?
- Some XLIFF files contain structural trans-units with no text. These are skipped.
- If the file has 0 segments, it may not contain any translation units

## Tags look wrong

If tags display as raw text (e.g. `{}` instead of `{1}`):

- Make sure you're running the latest version
- Try cycling the tag display mode using the **Tags** button in the toolbar

## Windows SmartScreen warning

When running the installer, Windows may show a "Windows protected your PC" message. This happens because the application is not yet code-signed. Click **More info** → **Run anyway** to proceed.

## Reporting issues

If you encounter a bug, please report it on GitHub:

**[Report an issue](https://github.com/Supervertaler/Supervertaler-Workbench-v2/issues/new)**

Include:
- What you were doing when the problem occurred
- The file format you were working with
- Any error messages shown
- Your operating system and Supervertaler Workbench version
