export type LLMProvider =
  | 'openai'
  | 'anthropic'
  | 'google'
  | 'mistral'
  | 'xai'
  | 'openrouter'
  | 'ollama';

export interface TranslateRequest {
  sourceText: string;
  sourceLanguage: string;
  targetLanguage: string;
  context?: string;
  systemPrompt: string;
  provider: LLMProvider;
  model: string;
  apiKey: string;
}

export interface LLMResponse {
  text: string;
  provider: LLMProvider;
  model: string;
  tokensUsed?: number;
}

/** Model definition for UI dropdowns */
export interface ModelDef {
  id: string;
  label: string;
  provider: LLMProvider;
}

/** Available models per provider */
export const MODELS: ModelDef[] = [
  // Anthropic
  { id: 'claude-sonnet-4-20250514', label: 'Claude Sonnet 4', provider: 'anthropic' },
  { id: 'claude-haiku-4-20250414', label: 'Claude Haiku 4', provider: 'anthropic' },
  // OpenAI
  { id: 'gpt-4.1', label: 'GPT-4.1', provider: 'openai' },
  { id: 'gpt-4.1-mini', label: 'GPT-4.1 Mini', provider: 'openai' },
  { id: 'gpt-4.1-nano', label: 'GPT-4.1 Nano', provider: 'openai' },
  { id: 'o3-mini', label: 'o3-mini', provider: 'openai' },
  // Google
  { id: 'gemini-2.5-flash', label: 'Gemini 2.5 Flash', provider: 'google' },
  { id: 'gemini-2.5-pro', label: 'Gemini 2.5 Pro', provider: 'google' },
  { id: 'gemini-3.1-pro-preview', label: 'Gemini 3.1 Pro (Preview)', provider: 'google' },
  { id: 'gemma-4-31b-it', label: 'Gemma 4 31B', provider: 'google' },
  { id: 'gemma-4-26b-a4b-it', label: 'Gemma 4 26B MoE', provider: 'google' },
  // xAI
  { id: 'grok-3', label: 'Grok 3', provider: 'xai' },
  { id: 'grok-3-mini', label: 'Grok 3 Mini', provider: 'xai' },
  // Mistral
  { id: 'mistral-large-latest', label: 'Mistral Large', provider: 'mistral' },
  { id: 'mistral-small-latest', label: 'Mistral Small', provider: 'mistral' },
  // OpenRouter
  { id: 'anthropic/claude-sonnet-4', label: 'Claude Sonnet 4 (via OpenRouter)', provider: 'openrouter' },
  { id: 'openai/gpt-4.1', label: 'GPT-4.1 (via OpenRouter)', provider: 'openrouter' },
  { id: 'google/gemini-2.5-flash', label: 'Gemini 2.5 Flash (via OpenRouter)', provider: 'openrouter' },
];

/** Provider display labels */
export const PROVIDER_LABELS: Record<LLMProvider, string> = {
  openai: 'OpenAI',
  anthropic: 'Anthropic',
  google: 'Google',
  mistral: 'Mistral',
  xai: 'xAI (Grok)',
  openrouter: 'OpenRouter',
  ollama: 'Ollama',
};

/** Providers that need an API key (Ollama is local) */
export const PROVIDERS_NEEDING_KEY: LLMProvider[] = [
  'openai',
  'anthropic',
  'google',
  'mistral',
  'xai',
  'openrouter',
];
