---
title: Setting Up API Keys
description: How to connect Supervertaler Workbench to AI translation providers.
sidebar:
  order: 4
---

To use AI-powered translation, you need an API key from at least one provider. Supervertaler Workbench supports multiple providers – you only need to set up the ones you want to use.

## Supported providers

| Provider | Models | Pricing |
|----------|--------|---------|
| [OpenAI](https://platform.openai.com/) | GPT-4o, GPT-4o mini | Pay-per-use |
| [Anthropic](https://console.anthropic.com/) | Claude Sonnet, Claude Haiku | Pay-per-use |
| [DeepL](https://www.deepl.com/pro-api) | DeepL Pro | Free tier available |
| [Ollama](https://ollama.com/) | Llama, Mistral, Gemma, etc. | Free (runs locally) |

:::tip
If you want to try AI translation without spending money, **Ollama** lets you run open-source language models locally on your own machine. See [AI Translation Overview](/ai-translation/overview/) for setup instructions.
:::

## Adding an API key

1. Click **Settings** in the toolbar
2. Go to the **API Keys** tab
3. Enter your API key for each provider you want to use
4. Click **Save**

API keys are stored locally on your machine. They are never sent anywhere except to the provider's own API.

## Getting an API key

### OpenAI

1. Go to [platform.openai.com](https://platform.openai.com/)
2. Sign up or log in
3. Navigate to **API Keys** and create a new key
4. Copy the key and paste it into Supervertaler Workbench

### Anthropic

1. Go to [console.anthropic.com](https://console.anthropic.com/)
2. Sign up or log in
3. Navigate to **API Keys** and create a new key
4. Copy the key and paste it into Supervertaler Workbench

### DeepL

1. Go to [deepl.com/pro-api](https://www.deepl.com/pro-api)
2. Sign up for a DeepL API plan (free tier translates up to 500,000 characters/month)
3. Copy your authentication key from the account settings
4. Paste it into Supervertaler Workbench

## Next steps

- [AI Translation Overview](/ai-translation/overview/) – How AI translation works in Supervertaler Workbench
