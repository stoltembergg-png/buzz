import { test, describe } from 'node:test';
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const REPO_ROOT = resolve(here, '../../../../');

function runCargoTests() {
  try {
    return execFileSync(
      'cargo',
      ['test', '--manifest-path', 'desktop/src-tauri/Cargo.toml', '--lib', 'hermes_home_inject', '--', '--nocapture'],
      { cwd: REPO_ROOT, encoding: 'utf8', timeout: 300000 }
    );
  } catch (e) {
    return e.stdout || e.stderr || String(e);
  }
}

describe('PR-007: Inject HERMES_HOME', () => {
  const output = runCargoTests();

  // @spec:AC-MEM-701 Hermes runtime gets overlay
  test('@spec:AC-MEM-701 hermes runtime receives HERMES_HOME overlay', () => {
    assert.match(output, /test_hermes_runtime_gets_overlay ... ok/);
  });

  // @spec:AC-MEM-702 Non-Hermes runtime does not get overlay or removal
  test('@spec:AC-MEM-702 non-hermes preserves inherited env (no remove, no add)', () => {
    assert.match(output, /test_non_hermes_runtime_gets_no_overlay ... ok/);
    assert.match(output, /test_apply_overlay_none_preserves_env ... ok/);
  });

  // @spec:AC-MEM-703 Parent HERMES_HOME overridden by scoped value
  test('@spec:AC-MEM-703 parent HERMES_HOME is overridden by scoped value', () => {
    assert.match(output, /test_apply_overlay_overrides_parent ... ok/);
  });

  // @spec:AC-MEM-704 Restart reuse
  test('@spec:AC-MEM-704 same context reuses same home after restart', () => {
    assert.match(output, /test_restart_reuses_same_home ... ok/);
  });

  // @spec:AC-MEM-705 Different channel/persona/relay isolate
  test('@spec:AC-MEM-705 different inputs isolate homes', () => {
    assert.match(output, /test_different_isolates_homes ... ok/);
  });

  // @spec:AC-MEM-706 Bootstrap failure blocks spawn
  test('@spec:AC-MEM-706 bootstrap failure blocks spawn with typed error', () => {
    assert.match(output, /test_bootstrap_failure_blocks_spawn ... ok/);
  });

  // @spec:AC-MEM-707 Errors are redacted (no config content leak)
  test('@spec:AC-MEM-707 errors do not leak config content', () => {
    assert.match(output, /test_inject_error_display_is_redacted ... ok/);
  });

  // Sentinel
  test('cargo reports 0 failures for hermes_home_inject module', () => {
    const m = output.match(/test result: (\d+) passed; (\d+) failed/);
    if (!m) {
      throw new Error('Could not parse cargo test result.\n' + output.slice(-500));
    }
    assert.equal(m[2], '0', `expected 0 failures, got ${m[2]}\n${output.slice(-500)}`);
    assert.ok(Number(m[1]) >= 8, `expected at least 8 tests, got ${m[1]}`);
  });
});
