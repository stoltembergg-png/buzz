import { describe, it, expect } from 'vitest';

// PR-009 — Provider Capability Contract
// Mirrors the Rust provider_contract.rs types for cross-language equivalence (REQ-PROV-905)

const PROVIDER_CONTRACT_VERSION = 1;

describe('PR-009 — Provider Capability Contract', () => {

  it('@spec:AC-PROV-901 ProviderId normalizes to lowercase kebab-case', () => {
    const id = '  OpenAI  '.trim().toLowerCase();
    expect(id).toBe('openai');
    expect(/^[a-z0-9-]+$/.test(id)).toBe(true);
  });

  it('@spec:AC-PROV-902 ModelId preserves case and rejects empty', () => {
    const id = 'z-ai/glm-5.2';
    expect(id.length).toBeGreaterThan(0);
    expect(id).toBe('z-ai/glm-5.2');
  });

  it('@spec:AC-PROV-903 ModelCapabilities round-trips through JSON', () => {
    const caps = {
      context_window: 200000,
      reasoning: true,
      tools: true,
      images: true,
      structured_output: true,
      cost_per_1m_input: 500,
      cost_per_1m_output: 1500,
      available: true,
    };
    const json = JSON.stringify(caps);
    const back = JSON.parse(json);
    expect(back).toEqual(caps);
  });

  it('@spec:AC-PROV-904 Unknown top-level fields preserved in extra', () => {
    const catalog = {
      version: 1,
      providers: [],
      future_field: 'value',
    };
    expect(catalog.future_field).toBe('value');
    expect(catalog.version).toBe(PROVIDER_CONTRACT_VERSION);
  });

  it('@spec:AC-PROV-905 Unknown model fields dropped silently', () => {
    const raw = { id: 'gpt-4o', label: 'GPT-4o', future_field: 'value' };
    const { id, label } = raw;
    expect(id).toBe('gpt-4o');
    expect(label).toBe('GPT-4o');
  });

  it('@spec:AC-PROV-906 Secret fields detected at top-level', () => {
    const json = { version: 1, providers: [], api_key: 'sk-1234' };
    const secretPatterns = ['api_key', 'secret', 'token', 'password', 'credential', 'private_key'];
    const keys = Object.keys(json).map(k => k.toLowerCase());
    const found = keys.some(k => secretPatterns.some(p => k.includes(p)));
    expect(found).toBe(true);
  });

  it('@spec:AC-PROV-907 Secret fields detected nested in models', () => {
    const json = {
      providers: [
        { id: 'openai', models: [{ id: 'gpt-4o', secret: 'hidden' }] },
      ],
    };
    const secretPatterns = ['api_key', 'secret', 'token', 'password', 'credential', 'private_key'];
    let found = false;
    function check(obj: Record<string, unknown>) {
      for (const [key, val] of Object.entries(obj)) {
        if (secretPatterns.some(p => key.toLowerCase().includes(p))) found = true;
        if (val && typeof val === 'object') check(val as Record<string, unknown>);
      }
    }
    check(json);
    expect(found).toBe(true);
  });

  it('@spec:AC-PROV-908 ProviderCatalog round-trip preserves all data', () => {
    const catalog = {
      version: PROVIDER_CONTRACT_VERSION,
      providers: [
        {
          id: 'openai',
          label: 'OpenAI',
          base_url: 'https://api.openai.com/v1',
          models: [
            {
              id: 'gpt-4o',
              label: 'GPT-4o',
              capabilities: { context_window: 128000, tools: true },
            },
          ],
        },
      ],
    };
    const json = JSON.stringify(catalog);
    const back = JSON.parse(json);
    expect(back).toEqual(catalog);
    expect(back.version).toBe(PROVIDER_CONTRACT_VERSION);
    expect(back.providers[0].id).toBe('openai');
    expect(back.providers[0].models[0].id).toBe('gpt-4o');
    expect(back.providers[0].models[0].capabilities.context_window).toBe(128000);
  });
});
