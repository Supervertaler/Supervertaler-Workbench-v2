---
title: AI Translation Overview
description: How AI-powered translation works in Supervertaler Workbench.
---

:::caution
AI translation is under active development. This page describes the planned functionality.
:::

Supervertaler Workbench can use large language models (LLMs) to translate, review, and rephrase segments. It connects to cloud providers or local models – you choose what works for you.

## Supported providers

| Provider | Type | Cost |
|----------|------|------|
| OpenAI (GPT-4o, GPT-4o mini) | Cloud | Pay-per-use |
| Anthropic (Claude) | Cloud | Pay-per-use |
| DeepL | Cloud | Free tier available |
| Ollama (Llama, Mistral, etc.) | Local | Free |

## How it works

1. Select a segment (or multiple segments) in the translation grid
2. Click **Translate** or press `Ctrl+T`
3. Supervertaler Workbench sends the source text, along with context (surrounding segments, TM matches, terminology), to the AI provider
4. The translation appears in the target column

## Prompts

AI translation uses configurable prompts that tell the model how to translate. Supervertaler Workbench includes built-in prompts for common tasks, and you can write your own.

Prompts use the `.svprompt` format – a simple text file with YAML metadata and a Markdown body. This format is shared with [Supervertaler for Trados](https://supervertaler.gitbook.io/trados), so prompts work in both tools.

## Privacy

- API keys are stored locally on your machine
- Source text is sent to the provider's API only when you trigger a translation
- When using Ollama, all processing happens on your machine – nothing is sent to the internet
