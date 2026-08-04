import { test, describe } from 'node:test';
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = resolve(fileURLToPath(import.meta.url), '..');
const REPO_ROOT = resolve(__dirname, '../../../../');

// Run `cargo test --lib memory_scope` and capture the output.
function runCargoTests() {
  try {
    return execFileSync(
      'cargo',
      ['test', '--manifest-path', 'desktop/src-tauri/Cargo.toml', '--lib', 'memory_scope', '--', '--nocapture'],
      { cwd: REPO_ROOT, encoding: 'utf8', timeout: 300000 }
    );
  } catch (e) {
    return e.stdout || e.stderr || String(e);
  }
}

describe('PR-005: HermesMemoryScope derived from RuntimeScopeContext', () => {
  const output = runCargoTests();

  // @spec:AC-MEM-501 Same context produces same hash/path (determinism)
  test('@spec:AC-MEM-501 same context → same hash and path', () => {
    assert.match(output, /test_scope_derived_from_context ... ok/);
    assert.match(output, /test_same_context_produces_same_hash ... ok/);
  });

  // @spec:AC-MEM-502 Different relay/channel/persona → different paths
  test('@spec:AC-MEM-502 different relay/channel/persona → different paths', () => {
    assert.match(output, /test_different_relay_produces_different_path ... ok/);
    assert.match(output, /test_different_channel_produces_different_path ... ok/);
    assert.match(output, /test_different_persona_produces_different_path ... ok/);
  });

  // @spec:AC-MEM-503 Display names do not affect hash
  test('@spec:AC-MEM-503 display names excluded from hash', () => {
    assert.match(output, /test_display_names_do_not_affect_scope ... ok/);
  });

  // @spec:AC-MEM-504 Traversal / Unicode / reserved names stay in hash
  test('@spec:AC-MEM-504 traversal and unicode do not escape hash', () => {
    assert.match(output, /test_traversal_inputs_stay_in_hash ... ok/);
    assert.match(output, /test_unicode_inputs_do_not_break_path ... ok/);
  });

  // @spec:AC-MEM-505 All Hermes aliases resolve to same scope
  test('@spec:AC-MEM-505 hermes / hermes-agent / hermes-acp resolve to same scope', () => {
    assert.match(output, /test_all_hermes_aliases_produce_same_hash ... ok/);
  });

  // @spec:AC-MEM-506 Non-Hermes runtime returns typed error
  test('@spec:AC-MEM-506 non-Hermes runtime → NotHermesRuntime error', () => {
    assert.match(output, /test_non_hermes_runtime_returns_error ... ok/);
    assert.match(output, /test_error_display_for_non_hermes ... ok/);
  });

  // @spec:AC-MEM-507 Hash is 64 lowercase hex characters
  test('@spec:AC-MEM-507 hash is 64 lowercase hex chars', () => {
    assert.match(output, /test_hash_is_64_hex_chars ... ok/);
    assert.match(output, /test_path_is_prefix_plus_hash_only ... ok/);
  });

  // @spec:AC-MEM-508 within() joins path without touching filesystem
  test('@spec:AC-MEM-508 within() joins with app-data root', () => {
    assert.match(output, /test_within_joins_with_app_data_root ... ok/);
    assert.match(output, /test_within_strips_trailing_separators ... ok/);
  });

  // Sentinel: cargo must report all tests passed (no failures)
  test('cargo reports 0 failures for memory_scope module', () => {
    const m = output.match(/test result: (\d+) passed; (\d+) failed/);
    if (!m) {
      throw new Error('Could not parse cargo test result line.\n' + output.slice(-500));
    }
    assert.equal(m[2], '0', `expected 0 failures, got ${m[2]}\n${output.slice(-800)}`);
    assert.ok(Number(m[1]) >= 16, `expected at least 16 tests, got ${m[1]}`);
  });
});
