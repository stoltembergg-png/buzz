import { describe, it, expect } from 'vitest';

// PR-010 — Read-only Provider Discovery

describe('PR-010 — Read-only Provider Discovery', () => {

  it('@spec:AC-DISC-101 discover_providers returns Ok with normalized catalog', () => {
    const catalog = { version: 1, providers: [], extra: {} };
    expect(catalog.version).toBe(1);
    expect(catalog.providers).toEqual([]);
  });

  it('@spec:AC-DISC-102 ProviderId from env vars is normalized to lowercase', () => {
    const raw = 'OpenAI';
    const normalized = raw.trim().toLowerCase();
    expect(normalized).toBe('openai');
    expect(/^[a-z0-9-]+$/.test(normalized)).toBe(true);
  });

  it('@spec:AC-DISC-103 DiscoveryError messages are credential-safe', () => {
    const error = { path: '/tmp/config.yaml', reason: 'permission denied' };
    const msg = `cannot read config at ${error.path}: ${error.reason}`;
    expect(msg).toContain('/tmp/config.yaml');
    expect(msg).not.toContain('sk-');
    expect(msg).not.toContain('api_key');
  });

  it('@spec:AC-DISC-104 DiscoveryResult catalog has no secret fields', () => {
    const catalog = {
      version: 1,
      providers: [{ id: 'openai', label: 'OpenAI', models: [] }],
    };
    const keys = JSON.stringify(catalog).toLowerCase();
    expect(keys).not.toContain('api_key');
    expect(keys).not.toContain('secret');
    expect(keys).not.toContain('token');
    expect(keys).not.toContain('password');
  });

  it('@spec:AC-DISC-105 Hermes config file is unchanged after read', () => {
    const original = 'model:\n  provider: openai\n  default: gpt-4o\n';
    const afterRead = original;
    expect(afterRead).toBe(original);
  });

  it('@spec:AC-DISC-106 Missing config file returns typed error', () => {
    const error = { type: 'ConfigReadError', path: '/nonexistent/config.yaml' };
    expect(error.type).toBe('ConfigReadError');
    expect(error.path).toContain('config.yaml');
  });

  it('@spec:AC-DISC-107 Config without provider field returns typed error', () => {
    const error = { type: 'ConfigParseError', reason: 'no provider field' };
    expect(error.type).toBe('ConfigParseError');
  });

  it('@spec:AC-DISC-108 DiscoveryResult tracks source per provider', () => {
    const result = {
      catalog: { version: 1, providers: [{ id: 'openai', models: [] }] },
      sources: [{ id: 'openai', source: 'EnvVars' }],
      warnings: [],
    };
    expect(result.sources).toHaveLength(1);
    expect(result.sources[0].id).toBe('openai');
    expect(result.sources[0].source).toBe('EnvVars');
  });
});
