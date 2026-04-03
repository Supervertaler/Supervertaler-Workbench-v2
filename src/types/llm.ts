export type LLMProvider =
  | 'openai'
  | 'anthropic'
  | 'google'
  | 'mistral'
  | 'xai'
  | 'ollama';

export interface TranslateRequest {
  sourceText: string;
  sourceLanguage: string;
  targetLanguage: string;
  context?: string;
  prompt?: string;
  provider: LLMProvider;
  model: string;
}

export interface LLMClient {
  translate(request: TranslateRequest): AsyncIterable<string>;
}
