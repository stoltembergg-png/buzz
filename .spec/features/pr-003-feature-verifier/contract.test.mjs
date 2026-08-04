// Contract test for PR-003 — verify-feature.sh
// Tests the shell script against existing features and temporary fixtures
// created within the repo's .spec/features/ directory.

import { test } from "node:test";
import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { existsSync, mkdirSync, rmSync, writeFileSync } from "node:fs";

const REPO_ROOT = resolve(fileURLToPath(import.meta.url), "../../../../");
const SCRIPT = join(REPO_ROOT, "scripts/verify-feature.sh");

// Detect bash path — works on MSYS, Cygwin, Git Bash, WSL, Linux
function findBash() {
  const candidates = [
    "sh",                      // MSYS/Cygwin often links sh -> bash (WORKS)
    process.env.BASH,
    "bash",                    // PATH lookup (often WSL, fails)
    "/usr/bin/bash",           // Cygwin/MSYS
    "/bin/bash",               // Linux/WSL
    "/usr/bin/sh",             // MSYS/Cygwin fallback
    "/bin/sh",                 // Linux fallback
    "C:/Program Files/Git/bin/bash.exe", // Git for Windows
    "C:/Program Files/Git/usr/bin/bash.exe",
    "C:/msys64/usr/bin/bash.exe",
  ];
  for (const cmd of candidates) {
    if (!cmd) continue;
    try {
      const r = spawnSync(cmd, ["--version"], { encoding: "utf8", timeout: 2000 });
      if (r.status === 0) return cmd;
    } catch {}
  }
  throw new Error("bash/sh not found — cannot run verify-feature.sh tests");
}

const BASH = findBash();

function toUnixPath(winPath) {
  // Convert C:\Users\... -> /c/Users/...
  return winPath
    .replace(/\\/g, "/")
    .replace(/^([A-Za-z]):/, (_, drive) => `/${drive.toLowerCase()}`);
}

const SCRIPT_UNIX = toUnixPath(join(REPO_ROOT, "scripts/verify-feature.sh"));

function runVerifyFeature(feature, args = []) {
  try {
    const out = execFileSync(BASH, [SCRIPT_UNIX, feature, ...args], {
      cwd: REPO_ROOT,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "pipe"],
      timeout: 10000,
    });
    return { exit: 0, stdout: out, stderr: "" };
  } catch (e) {
    return { exit: e.status, stdout: e.stdout || "", stderr: e.stderr || "" };
  }
}

function createTestFeature(name, specContent, testContent) {
  const featureDir = join(REPO_ROOT, ".spec/features", name);
  if (existsSync(featureDir)) rmSync(featureDir, { recursive: true, force: true });
  mkdirSync(featureDir, { recursive: true });
  writeFileSync(join(featureDir, "spec.md"), specContent);
  writeFileSync(join(featureDir, "contract.test.mjs"), testContent);
  return featureDir;
}

function cleanupTestFeature(name) {
  const featureDir = join(REPO_ROOT, ".spec/features", name);
  if (existsSync(featureDir)) rmSync(featureDir, { recursive: true, force: true });
}

// Positive: existing features
test("@spec:AC-GOV-301 @spec:AC-GOV-306 T-GOV-301 valid feature pr-002-templates returns exit 0", () => {
  const r = runVerifyFeature("pr-002-templates");
  assert.strictEqual(r.exit, 0, `expected exit 0, got ${r.exit}: ${r.stderr}`);
});

test("@spec:AC-GOV-301 @spec:AC-GOV-306 T-GOV-301 valid feature hermes-runtime returns exit 0", () => {
  const r = runVerifyFeature("hermes-runtime");
  assert.strictEqual(r.exit, 0, `expected exit 0, got ${r.exit}: ${r.stderr}`);
});

// Negative: nonexistent feature → exit 1
test("@spec:AC-GOV-301 T-GOV-302 nonexistent feature returns exit 1", () => {
  const r = runVerifyFeature("nonexistent-feature-xyz-123");
  assert.strictEqual(r.exit, 1, `expected exit 1, got ${r.exit}: ${r.stdout}`);
  assert.match(r.stderr, /feature directory not found/);
});

// Negative: missing spec.md → exit 2
test("@spec:AC-GOV-302 T-GOV-303 missing spec.md returns exit 2", () => {
  const name = "test-missing-spec-" + Date.now();
  const featureDir = join(REPO_ROOT, ".spec/features", name);
  mkdirSync(featureDir, { recursive: true });
  writeFileSync(join(featureDir, "contract.test.mjs"), 'import { test } from "node:test"; test("@spec:AC-001 ok", () => {});');
  try {
    const r = runVerifyFeature(name);
    assert.strictEqual(r.exit, 2, `expected exit 2, got ${r.exit}: ${r.stdout}`);
    assert.match(r.stderr, /spec.md missing/);
  } finally {
    rmSync(featureDir, { recursive: true, force: true });
  }
});

// Negative: missing contract.test.mjs → exit 2
test("@spec:AC-GOV-302 T-GOV-304 missing contract.test.mjs returns exit 2", () => {
  const name = "test-missing-test-" + Date.now();
  const featureDir = join(REPO_ROOT, ".spec/features", name);
  mkdirSync(featureDir, { recursive: true });
  writeFileSync(join(featureDir, "spec.md"), "# spec\n\n#### AC-001 — test\n\nText.");
  try {
    const r = runVerifyFeature(name);
    assert.strictEqual(r.exit, 2, `expected exit 2, got ${r.exit}: ${r.stdout}`);
    assert.match(r.stderr, /contract.test.mjs missing/);
  } finally {
    rmSync(featureDir, { recursive: true, force: true });
  }
});

// Negative: duplicate AC in spec → exit 3
test("@spec:AC-GOV-303 T-GOV-305 duplicate AC in spec returns exit 3", () => {
  const name = "test-dup-spec-" + Date.now();
  createTestFeature(name,
    "# spec\n\n#### AC-001 — test\n\nText.\n\n#### AC-001 — duplicate\n\nText.",
    'import { test } from "node:test"; test("@spec:AC-001 ok", () => {});'
  );
  try {
    const r = runVerifyFeature(name);
    assert.strictEqual(r.exit, 3, `expected exit 3, got ${r.exit}: ${r.stdout}`);
    assert.match(r.stderr, /duplicate AC IDs in spec/);
  } finally {
    cleanupTestFeature(name);
  }
});

// Negative: duplicate @spec:AC in test is now ALLOWED (multiple tests per AC)
test("@spec:AC-GOV-303 T-GOV-306 duplicate @spec:AC in test is allowed (no longer an error)", () => {
  const name = "test-dup-test-" + Date.now();
  createTestFeature(name,
    "# spec\n\n#### AC-001 — test\n\nText.",
    'import { test } from "node:test"; test("@spec:AC-001 first", () => {}); test("@spec:AC-001 second", () => {});'
  );
  try {
    const r = runVerifyFeature(name);
    // Should succeed (exit 0) since duplicate @spec:AC in tests is now allowed
    assert.strictEqual(r.exit, 0, `expected exit 0 (allowed), got ${r.exit}: ${r.stdout}`);
  } finally {
    cleanupTestFeature(name);
  }
});

// Negative: AC in spec with no test → exit 4
test("@spec:AC-GOV-302 T-GOV-307 AC in spec without test returns exit 4", () => {
  const name = "test-orphan-req-" + Date.now();
  createTestFeature(name,
    "# spec\n\n#### AC-001 — has test\n\nText.\n\n#### AC-999 — no test\n\nText.",
    'import { test } from "node:test"; test("@spec:AC-001 ok", () => {});'
  );
  try {
    const r = runVerifyFeature(name);
    assert.strictEqual(r.exit, 4, `expected exit 4, got ${r.exit}: ${r.stdout}`);
    assert.match(r.stderr, /AC.*in spec have no test/);
  } finally {
    cleanupTestFeature(name);
  }
});

// Negative: @spec:AC in test with no spec → exit 5
test("@spec:AC-GOV-303 T-GOV-308 @spec:AC in test without spec returns exit 5", () => {
  const name = "test-orphan-test-" + Date.now();
  createTestFeature(name,
    "# spec\n\n#### AC-001 — has test\n\nText.",
    'import { test } from "node:test"; test("@spec:AC-001 ok", () => {}); test("@spec:AC-999 orphan", () => {});'
  );
  try {
    const r = runVerifyFeature(name);
    assert.strictEqual(r.exit, 5, `expected exit 5, got ${r.exit}: ${r.stdout}`);
    assert.match(r.stderr, /test tags reference unknown AC/);
  } finally {
    cleanupTestFeature(name);
  }
});

// Negative: task done without evidence → exit 6
test("@spec:AC-GOV-304 T-GOV-309 task done without evidence returns exit 6", () => {
  const name = "test-task-no-evid-" + Date.now();
  const featureDir = createTestFeature(name,
    "# spec\n\n#### AC-001 — has test\n\nText.",
    'import { test } from "node:test"; test("@spec:AC-001 ok", () => {});'
  );
  // Add a tasks.md with a done task without evidence
  writeFileSync(join(featureDir, "tasks.md"), "| ID | Task | Owner | Status | Evidence |\n|----|------|-------|--------|----------|\n| T-XXX-001 | dummy | me | done | |\n");
  try {
    const r = runVerifyFeature(name);
    assert.strictEqual(r.exit, 6, `expected exit 6, got ${r.exit}: ${r.stdout}`);
    assert.match(r.stderr, /task marked done without evidence ref/);
  } finally {
    cleanupTestFeature(name);
  }
});

// Negative: path traversal → exit 8
test("@spec:AC-GOV-305 T-GOV-311 path traversal returns exit 8", () => {
  const r = runVerifyFeature("../etc/passwd");
  assert.strictEqual(r.exit, 8, `expected exit 8, got ${r.exit}: ${r.stdout}`);
  assert.match(r.stderr, /unsafe characters or traversal/);
});

// Negative: slash in name → exit 8
test("@spec:AC-GOV-305 T-GOV-311 slash in feature name returns exit 8", () => {
  const r = runVerifyFeature("feature/with/slash");
  assert.strictEqual(r.exit, 8, `expected exit 8, got ${r.exit}: ${r.stdout}`);
  assert.match(r.stderr, /unsafe characters or traversal/);
});

// Negative: semicolon injection → exit 8
test("@spec:AC-GOV-305 T-GOV-311 semicolon injection returns exit 8", () => {
  const r = runVerifyFeature("feature;evil");
  assert.strictEqual(r.exit, 8, `expected exit 8, got ${r.exit}: ${r.stdout}`);
  assert.match(r.stderr, /unsafe characters or traversal/);
});

// Integration: --json output
test("@spec:AC-GOV-305 T-GOV-301 --json flag produces valid JSON", () => {
  const r = runVerifyFeature("pr-002-templates", ["--json"]);
  assert.strictEqual(r.exit, 0, `expected exit 0, got ${r.exit}: ${r.stderr}`);
  const parsed = JSON.parse(r.stdout.trim());
  assert.ok(parsed.feature === "pr-002-templates");
  assert.ok(parsed.gitRev);
  assert.ok(parsed.platform);
  assert.ok(parsed.tools);
  assert.ok(Array.isArray(parsed.specACs));
  assert.ok(Array.isArray(parsed.testACs));
  assert.ok(parsed.results);
});

// Regression: runtime ≤ 3s
test("@spec:AC-GOV-307 T-GOV-313 valid feature completes within 3 seconds", () => {
  const start = Date.now();
  const r = runVerifyFeature("pr-002-templates");
  const elapsed = Date.now() - start;
  assert.strictEqual(r.exit, 0);
  assert.ok(elapsed < 3000, `took ${elapsed}ms, expected < 3000ms`);
});