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
      ['test', '--manifest-path', 'desktop/src-tauri/Cargo.toml', '--lib', 'persistence_harness', '--', '--nocapture'],
      { cwd: REPO_ROOT, encoding: 'utf8', timeout: 300000 }
    );
  } catch (e) {
    return e.stdout || e.stderr || String(e);
  }
}

describe('PR-008: Memory persistence harness', () => {
  const output = runCargoTests();

  // @spec:AC-MEM-801 Single command runs all scenarios
  test('@spec:AC-MEM-801 harness runs all 5 scenarios', () => {
    assert.match(output, /test_harness_runs_all_scenarios ... ok/);
  });

  // @spec:AC-MEM-802 Random marker written under first scope
  test('@spec:AC-MEM-802 marker written in first scope (implied by restart scenario)', () => {
    // Write is exercised as the first scenario; recovery in #803 confirms it landed.
    assert.match(output, /test_restart_same_scope_recovers_marker ... ok/);
  });

  // @spec:AC-MEM-803 Restart in same scope recovers marker
  test('@spec:AC-MEM-803 restart in same scope recovers marker', () => {
    assert.match(output, /test_restart_same_scope_recovers_marker ... ok/);
  });

  // @spec:AC-MEM-804 Different channel/persona/relay do not recover
  test('@spec:AC-MEM-804 different scopes do not recover marker', () => {
    assert.match(output, /test_different_channel_no_recovery ... ok/);
    assert.match(output, /test_different_persona_no_recovery ... ok/);
    assert.match(output, /test_different_relay_no_recovery ... ok/);
  });

  // @spec:AC-MEM-805 Report contains required fields
  test('@spec:AC-MEM-805 report has schema, os, arch, scenarios, all_passed', () => {
    assert.match(output, /test_report_has_required_fields ... ok/);
  });

  // @spec:AC-MEM-806 Marker content is redacted (≤8 chars)
  test('@spec:AC-MEM-806 marker content redacted in report', () => {
    assert.match(output, /test_report_redacts_marker_content ... ok/);
  });

  // @spec:AC-MEM-807 JSON report is parseable
  test('@spec:AC-MEM-807 report JSON is parseable', () => {
    assert.match(output, /test_report_json_is_parseable ... ok/);
  });

  // Sentinel: all scenarios pass
  test('cargo reports 0 failures and all scenarios pass', () => {
    assert.match(output, /test_all_scenarios_pass ... ok/);
    const m = output.match(/test result: (\d+) passed; (\d+) failed/);
    if (!m) {
      throw new Error('Could not parse cargo test result.\n' + output.slice(-400));
    }
    assert.equal(m[2], '0', `expected 0 failures, got ${m[2]}\n${output.slice(-500)}`);
    assert.ok(Number(m[1]) >= 9, `expected at least 9 tests, got ${m[1]}`);
  });
});
