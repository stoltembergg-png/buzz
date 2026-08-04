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
      ['test', '--manifest-path', 'desktop/src-tauri/Cargo.toml', '--lib', 'profile_bootstrap', '--', '--nocapture'],
      { cwd: REPO_ROOT, encoding: 'utf8', timeout: 300000 }
    );
  } catch (e) {
    return e.stdout || e.stderr || String(e);
  }
}

describe('PR-006: Hermes profile bootstrap', () => {
  const output = runCargoTests();

  // @spec:AC-MEM-601 create + idempotent
  test('@spec:AC-MEM-601 creates profile dir and is idempotent', () => {
    assert.match(output, /test_bootstrap_creates_dir ... ok/);
    assert.match(output, /test_bootstrap_is_idempotent ... ok/);
  });

  // @spec:AC-MEM-602 selective copy (config.yaml only, never .env/memories/sessions/state)
  test('@spec:AC-MEM-602 copies only config.yaml, never siblings', () => {
    assert.match(output, /test_copies_config_yaml_from_hermes_home ... ok/);
    assert.match(output, /test_forbidden_siblings_not_copied ... ok/);
  });

  // @spec:AC-MEM-603 preserve existing config
  test('@spec:AC-MEM-603 existing config.yaml is preserved (not overwritten)', () => {
    assert.match(output, /test_existing_config_preserved ... ok/);
  });

  // @spec:AC-MEM-604 missing source config is OK
  test('@spec:AC-MEM-604 missing source config is accepted', () => {
    assert.match(output, /test_missing_source_config_is_ok ... ok/);
    assert.match(output, /test_none_hermes_home_is_ok ... ok/);
  });

  // @spec:AC-MEM-605 symlink in profile path refused
  test('@spec:AC-MEM-605 symlink in profile path is refused', () => {
    assert.match(output, /test_symlink_in_profile_path_rejected ... ok/);
  });

  // @spec:AC-MEM-606 file where directory expected refused
  test('@spec:AC-MEM-606 file-where-directory-expected is refused', () => {
    assert.match(output, /test_file_where_directory_expected_rejected ... ok/);
  });

  // @spec:AC-MEM-607 second init preserves first winner
  test('@spec:AC-MEM-607 second init preserves first-writer config', () => {
    assert.match(output, /test_second_bootstrap_does_not_truncate ... ok/);
  });

  // Sentinel: all tests passed, no failures
  test('cargo reports 0 failures for profile_bootstrap module', () => {
    const m = output.match(/test result: (\d+) passed; (\d+) failed/);
    if (!m) {
      throw new Error('Could not parse cargo test result.\n' + output.slice(-400));
    }
    assert.equal(m[2], '0', `expected 0 failures, got ${m[2]}\n${output.slice(-500)}`);
    assert.ok(Number(m[1]) >= 11, `expected at least 11 tests, got ${m[1]}`);
  });

  // Errors do not leak config content (API contract)
  test('@spec:AC-MEM-608 errors do not leak config content', () => {
    assert.match(output, /test_error_messages_do_not_contain_config_content ... ok/);
  });
});
