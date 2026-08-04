// Contract test for PR-004 — RuntimeScopeContext (Rust module).
// Validates the spec ↔ module contract: AC-MEM-401..408.

import { test } from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const REPO_ROOT = resolve(fileURLToPath(import.meta.url), "../../../");
const CARGO_TOML = join(REPO_ROOT, "desktop/src-tauri/Cargo.toml");

function cargoTest(filter) {
  try {
    const out = execFileSync("cargo", [
      "test",
      "--manifest-path",
      CARGO_TOML,
      "--lib",
      filter,
      "--",
      "--nocapture",
    ], { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"], timeout: 300000 });
    return { exit: 0, stdout: out, stderr: "" };
  } catch (e) {
    return { exit: e.status, stdout: e.stdout || "", stderr: e.stderr || "" };
  }
}

test("@spec:AC-MEM-401 @spec:AC-MEM-402 @spec:AC-MEM-403 @spec:AC-MEM-404 @spec:AC-MEM-405 @spec:AC-MEM-406 @spec:AC-MEM-407 @spec:AC-MEM-408 T-MEM-401 runtime_scope tests all pass", () => {
  const r = cargoTest("runtime_scope");
  assert.strictEqual(r.exit, 0, `cargo test failed (exit ${r.exit}):\n${r.stderr.slice(-500)}\n${r.stdout.slice(-500)}`);
  // Confirm all 12 tests ran
  assert.match(r.stdout, /test result: ok\. 12 passed/);
});

test("@spec:AC-MEM-401 T-MEM-402 display name change preserves equality", () => {
  const r = cargoTest("runtime_scope::tests::test_display_name_does_not_change_identity");
  assert.strictEqual(r.exit, 0);
});

test("@spec:AC-MEM-403 T-MEM-403 invalid relay URL inputs return error", () => {
  const r = cargoTest("runtime_scope::tests::test_relay_url_rejects_invalid");
  assert.strictEqual(r.exit, 0);
});

test("@spec:AC-MEM-404 T-MEM-404 channel ID with spaces returns InvalidChannelId", () => {
  const r = cargoTest("runtime_scope::tests::test_channel_id_validation");
  assert.strictEqual(r.exit, 0);
});

test("@spec:AC-MEM-408 T-MEM-405 path_hash is deterministic", () => {
  const r = cargoTest("runtime_scope::tests::test_path_hash_is_deterministic_and_changes_with_inputs");
  assert.strictEqual(r.exit, 0);
});
