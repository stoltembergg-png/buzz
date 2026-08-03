# Hermes Persona + Channel Local Memory Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Give every Hermes agent a persistent local `HERMES_HOME` scoped by relay/community, channel, and persona identity, without changing relay protocol or other ACP runtimes.

**Architecture:** Add a focused Rust module in `buzz-acp` that canonicalizes the scope tuple, hashes it into a versioned app-data path, and safely bootstraps the scoped Hermes home from only the primary `config.yaml`. Wire the resulting `HERMES_HOME` into the existing Hermes-only environment assembly before process spawn. Stable scope inputs are passed through the existing agent launch configuration; no memory content or absolute path is serialized to relay events or UI metadata.

**Tech Stack:** Rust, Tokio filesystem/process APIs, SHA-256 via the workspace's existing hashing dependency, Tauri/Buzz Desktop launch configuration, ACP process spawning, Rust unit and temporary-filesystem tests.

## Global Constraints

- Memory is local-only and never published to relay events.
- Scope identity is `relay/community identity + channel ID + persona identity`.
- Raw scope values never become filesystem path fragments.
- Path format is `<Buzz app data>/hermes/profiles/v1/<lowercase-hex-hash>/`.
- Bootstrap copies only `config.yaml`; it never copies memories, sessions, state, caches, OAuth stores, `.env`, or secret-store material.
- Existing scoped configuration is never overwritten.
- Missing primary Hermes configuration is not an error.
- Symlink and non-directory conflicts fail closed.
- Only normalized Hermes runtimes receive the scoped `HERMES_HOME`.
- Existing Hermes credential and reasoning-effort bridges remain unchanged.
- Non-Hermes runtimes must neither receive nor have `HERMES_HOME` altered.

---

### Task 1: Define scope identity and deterministic path derivation

**Files:**
- Create: `crates/buzz-acp/src/hermes_memory.rs`
- Modify: `crates/buzz-acp/src/lib.rs`
- Test: `crates/buzz-acp/src/hermes_memory.rs` (`#[cfg(test)]` module)

**Interfaces:**
- Produces: `pub(crate) struct HermesMemoryScope { relay_scope: String, channel_id: String, persona_id: String }`
- Produces: `pub(crate) fn is_hermes_command(command: &str) -> bool`
- Produces: `pub(crate) fn scope_digest(scope: &HermesMemoryScope) -> String`
- Produces: `pub(crate) fn scoped_home(app_data_dir: &Path, scope: &HermesMemoryScope) -> PathBuf`
- Consumes: existing `normalize_agent_command_identity` behavior from `config.rs` through a `pub(crate)` helper or equivalent shared normalization.

- [ ] **Step 1: Write failing tests for stable and isolated scope hashes**

Add tests asserting:

```rust
#[test]
fn same_scope_is_stable() {
    let scope = HermesMemoryScope::new("wss://relay.example", "channel-a", "persona-a");
    assert_eq!(scope_digest(&scope), scope_digest(&scope));
}

#[test]
fn each_scope_component_isolated() {
    let base = HermesMemoryScope::new("wss://relay.example", "channel-a", "persona-a");
    assert_ne!(scope_digest(&base), scope_digest(&HermesMemoryScope::new("wss://other.example", "channel-a", "persona-a")));
    assert_ne!(scope_digest(&base), scope_digest(&HermesMemoryScope::new("wss://relay.example", "channel-b", "persona-a")));
    assert_ne!(scope_digest(&base), scope_digest(&HermesMemoryScope::new("wss://relay.example", "channel-a", "persona-b")));
}

#[test]
fn raw_identifiers_never_enter_the_path() {
    let scope = HermesMemoryScope::new("../../relay", "../channel", "persona/../../secret");
    let path = scoped_home(Path::new("/tmp/buzz"), &scope);
    let rendered = path.to_string_lossy();
    assert!(!rendered.contains("../"));
    assert!(!rendered.contains("persona"));
    assert!(rendered.ends_with(scope_digest(&scope)));
}
```

- [ ] **Step 2: Run focused tests and confirm failure**

Run:

```bash
cargo test -p buzz-acp hermes_memory --no-fail-fast
```

Expected: compilation failure because `hermes_memory` types/functions do not exist.

- [ ] **Step 3: Implement canonical hashing and path layout**

Implement a canonical byte string exactly as:

```rust
format!(
    "buzz-hermes-memory-v1\n{}\n{}\n{}",
    scope.relay_scope, scope.channel_id, scope.persona_id
)
```

Hash it using SHA-256 and lowercase hex encoding. Return:

```rust
app_data_dir
    .join("hermes")
    .join("profiles")
    .join("v1")
    .join(scope_digest(scope))
```

Keep all fields private and avoid deriving `Debug` if it would expose identifiers in logs.

- [ ] **Step 4: Run focused tests**

Run:

```bash
cargo test -p buzz-acp hermes_memory --no-fail-fast
```

Expected: all scope/path tests pass.

- [ ] **Step 5: Commit**

```bash
git add crates/buzz-acp/src/hermes_memory.rs crates/buzz-acp/src/lib.rs
git commit -m "feat(buzz-acp): derive scoped Hermes memory homes"
```

### Task 2: Add safe local filesystem bootstrap

**Files:**
- Modify: `crates/buzz-acp/src/hermes_memory.rs`
- Modify: `crates/buzz-acp/Cargo.toml` only if a temporary-directory dev dependency is not already available
- Test: `crates/buzz-acp/src/hermes_memory.rs`

**Interfaces:**
- Produces: `pub(crate) struct HermesHomeBootstrap { pub scoped_home: PathBuf }`
- Produces: `pub(crate) fn bootstrap_scoped_home(app_data_dir: &Path, primary_home: &Path, scope: &HermesMemoryScope) -> Result<HermesHomeBootstrap, HermesMemoryError>`
- Produces: `pub(crate) enum HermesMemoryError` with non-secret, path-redacted variants.
- Consumes: `scoped_home` from Task 1.

- [ ] **Step 1: Write failing filesystem tests**

Cover these cases using a temporary directory:

```rust
#[test]
fn creates_scope_and_copies_config_once() { /* primary/config.yaml copied */ }

#[test]
fn existing_scoped_config_is_not_overwritten() { /* second bootstrap preserves scoped contents */ }

#[test]
fn missing_primary_config_is_allowed() { /* scoped directory exists, no config */ }

#[test]
fn memories_and_sessions_are_not_copied() { /* primary children remain absent in scoped home */ }

#[cfg(unix)]
#[test]
fn symlink_config_target_fails_closed() { /* scoped config symlink causes error */ }

#[test]
fn non_directory_scope_conflict_fails_closed() { /* file at scope path causes error */ }
```

- [ ] **Step 2: Run focused tests and confirm failure**

Run:

```bash
cargo test -p buzz-acp hermes_memory --no-fail-fast
```

Expected: missing bootstrap API failures.

- [ ] **Step 3: Implement idempotent bootstrap**

Implementation rules:

1. Create `.../hermes/profiles/v1` with `create_dir_all`.
2. Reject a pre-existing non-directory at the scope path.
3. Create the scope directory with `create_dir`; treat `AlreadyExists` as a race and revalidate it is a real directory.
4. Use `symlink_metadata` for the scoped `config.yaml`; reject symlinks and non-files.
5. If the scoped file is absent and primary `config.yaml` is a regular file, open the destination using create-new semantics and copy bytes.
6. If another process wins the create race, accept the winner without overwrite.
7. Never enumerate or recursively copy the primary home.
8. Error messages include only a stable operation label and app-relative suffix, never file contents or the primary absolute path.

- [ ] **Step 4: Run focused tests**

Run:

```bash
cargo test -p buzz-acp hermes_memory --no-fail-fast
```

Expected: all filesystem tests pass.

- [ ] **Step 5: Commit**

```bash
git add crates/buzz-acp/src/hermes_memory.rs crates/buzz-acp/Cargo.toml
git commit -m "feat(buzz-acp): bootstrap isolated Hermes homes"
```

### Task 3: Carry stable persona and channel scope into agent launch configuration

**Files:**
- Modify: `crates/buzz-acp/src/config.rs`
- Modify: the Desktop agent launch command that constructs `CliArgs`/`Config` for managed agents, expected under `desktop/src-tauri/src/commands/agent_model_process.rs` or the adjacent managed-agent process module identified during implementation
- Test: `crates/buzz-acp/src/config.rs`
- Test: relevant Desktop Rust command tests beside the modified launch code

**Interfaces:**
- Produces: optional launch fields or environment inputs for `memory_relay_scope`, `memory_channel_id`, and `memory_persona_id`.
- Consumes: stable relay URL/community identity already present in `Config`.
- Consumes: stable channel UUID/coordinate from the active agent/channel launch request.
- Consumes: stable persona ID/agent definition ID from the persona snapshot, never the display name.

- [ ] **Step 1: Write failing parsing/config tests**

Add tests proving:

```rust
#[test]
fn memory_scope_requires_all_three_components() { /* partial tuples are rejected or disabled explicitly */ }

#[test]
fn memory_scope_uses_stable_persona_id_not_display_name() { /* display label changes do not alter passed ID */ }
```

In Desktop tests, construct two launch requests with the same persona ID but different display names and assert identical scope inputs; then change channel ID and assert a different channel input.

- [ ] **Step 2: Run focused tests and confirm failure**

Run:

```bash
cargo test -p buzz-acp config::tests --no-fail-fast
cargo test --manifest-path desktop/src-tauri/Cargo.toml --lib agent_model_process --no-fail-fast
```

Expected: new scope fields or helpers are missing.

- [ ] **Step 3: Implement minimal scope transport**

Use existing CLI/environment conventions rather than adding relay protocol fields. Prefer explicit host-only environment variables:

```text
BUZZ_ACP_MEMORY_RELAY_SCOPE
BUZZ_ACP_MEMORY_CHANNEL_ID
BUZZ_ACP_MEMORY_PERSONA_ID
BUZZ_ACP_APP_DATA_DIR
```

Requirements:

- Desktop sets them only for managed Hermes launches.
- Values come from stable IDs.
- `Config::from_args` groups them into `Option<HermesMemoryScope>` only when all identity components and app-data root are present.
- Partial scope input returns a clear configuration error rather than silently merging identities.
- These values are not forwarded to arbitrary tool subprocesses.

- [ ] **Step 4: Run focused tests**

Run:

```bash
cargo test -p buzz-acp config::tests --no-fail-fast
cargo test --manifest-path desktop/src-tauri/Cargo.toml --lib agent_model_process --no-fail-fast
```

Expected: scope transport tests pass.

- [ ] **Step 5: Commit**

```bash
git add crates/buzz-acp/src/config.rs desktop/src-tauri/src/commands/agent_model_process.rs desktop/src-tauri/src/commands/*test*
git commit -m "feat(desktop): pass stable Hermes memory scope"
```

### Task 4: Inject the scoped home into Hermes process spawn

**Files:**
- Modify: `crates/buzz-acp/src/config.rs`
- Modify: `crates/buzz-acp/src/acp.rs`
- Modify: `crates/buzz-acp/src/pool.rs` if spawn environment is assembled there
- Modify: `crates/buzz-acp/src/hermes_memory.rs`
- Test: `crates/buzz-acp/src/acp.rs`
- Test: `crates/buzz-acp/src/config.rs`

**Interfaces:**
- Produces: `pub(crate) fn hermes_memory_env(command: &str, app_data_dir: Option<&Path>, primary_home: &Path, scope: Option<&HermesMemoryScope>) -> Result<Vec<(String, String)>, HermesMemoryError>`
- Consumes: bootstrap API from Task 2 and parsed scope from Task 3.
- Produces exactly one environment pair for valid Hermes scope: `("HERMES_HOME", scoped_path)`.

- [ ] **Step 1: Write failing environment and spawn tests**

Add tests proving:

```rust
#[test]
fn hermes_receives_scoped_home() { /* aliases hermes/hermes-agent/hermes-acp */ }

#[test]
fn non_hermes_receives_no_memory_home() { /* codex/claude/goose unchanged */ }

#[test]
fn scoped_home_overrides_inherited_primary_home_for_hermes() { /* explicit child env wins */ }
```

Extend the existing `/bin/sh` spawn tests to print `${HERMES_HOME-unset}` and verify the scoped value for Hermes and unchanged/unset behavior for non-Hermes.

- [ ] **Step 2: Run focused tests and confirm failure**

Run:

```bash
cargo test -p buzz-acp hermes_memory_env --no-fail-fast
cargo test -p buzz-acp acp::tests::spawn --no-fail-fast
```

Expected: environment injection is absent.

- [ ] **Step 3: Implement Hermes-only environment assembly**

Before creating `AcpClient`, resolve the primary Hermes home from the parent `HERMES_HOME` or the same default used by the existing Hermes config bridge. Bootstrap the scoped home and append `HERMES_HOME` to the explicit persona environment.

In `AcpClient::spawn`, ensure explicit `HERMES_HOME` for a Hermes command overwrites inherited parent state. For non-Hermes commands, preserve current behavior and do not add or remove `HERMES_HOME` solely because this feature exists.

Do not merge this logic with `_HERMES_FORCE_BUZZ_*` credential variables or `BUZZ_ACP_REASONING_EFFORT`; keep separate helpers and tests.

- [ ] **Step 4: Run focused tests**

Run:

```bash
cargo test -p buzz-acp hermes_memory --no-fail-fast
cargo test -p buzz-acp acp::tests --no-fail-fast
cargo test -p buzz-acp config::tests --no-fail-fast
```

Expected: all focused tests pass.

- [ ] **Step 5: Commit**

```bash
git add crates/buzz-acp/src/hermes_memory.rs crates/buzz-acp/src/config.rs crates/buzz-acp/src/acp.rs crates/buzz-acp/src/pool.rs
git commit -m "feat(buzz-acp): launch Hermes with scoped local memory"
```

### Task 5: Add contract documentation and regression verification

**Files:**
- Create: `.spec/features/hermes-scoped-memory/spec.md`
- Create: `.spec/features/hermes-scoped-memory/tasks.md`
- Create: `.spec/features/hermes-scoped-memory/contract.test.mjs`
- Modify: `CHANGELOG.md`
- Modify: `docs/superpowers/specs/2026-08-03-hermes-persona-channel-memory-design.md` only if implementation discoveries require a factual correction

**Interfaces:**
- Consumes all implementation behavior from Tasks 1–4.
- Produces executable contract references for deterministic isolation, local-only persistence, safe bootstrap, and Hermes-only spawn behavior.

- [ ] **Step 1: Add contract tests invoking exact Rust test filters**

The Node contract should run:

```text
cargo test -p buzz-acp hermes_memory
cargo test -p buzz-acp non_hermes_receives_no_memory_home
cargo test --manifest-path desktop/src-tauri/Cargo.toml --lib hermes_memory
```

Each acceptance criterion must map to at least one exact test filter.

- [ ] **Step 2: Add changelog entry**

Document:

- local memory isolated by relay/channel/persona;
- reuse of Hermes-native memory through scoped `HERMES_HOME`;
- no relay synchronization or global-persona sharing;
- primary global Hermes memory remains untouched.

Do not claim semantic search, memory UI, or cross-device synchronization.

- [ ] **Step 3: Run format, focused suites, and contract verification**

Run:

```bash
cargo fmt --all -- --check
cargo test -p buzz-acp hermes_memory --no-fail-fast
cargo test -p buzz-acp acp::tests --no-fail-fast
cargo test -p buzz-acp config::tests --no-fail-fast
cargo test --manifest-path desktop/src-tauri/Cargo.toml --lib hermes_memory --no-fail-fast
pnpm run check
npx onp-spec verify hermes-scoped-memory
npx onp-spec audit --ci
```

Expected: all commands pass. If an unrelated pre-existing failure occurs, reproduce it on the branch base and document the exact command and matching failure in the PR.

- [ ] **Step 4: Review for privacy regressions**

Search changed code and logs:

```bash
git diff main...HEAD | rg "HERMES_HOME|memory|config.yaml|tracing::|println!|dbg!"
```

Verify no memory content, config content, primary absolute path, or raw scope tuple is logged or serialized.

- [ ] **Step 5: Commit**

```bash
git add .spec/features/hermes-scoped-memory CHANGELOG.md docs/superpowers/specs/2026-08-03-hermes-persona-channel-memory-design.md
git commit -m "docs: specify Hermes scoped local memory"
```

### Task 6: Final verification and Pull Request

**Files:**
- No intended source changes; only fix issues found by verification.

**Interfaces:**
- Produces a reviewable branch and Pull Request against `main`.

- [ ] **Step 1: Inspect final diff and commit history**

Run:

```bash
git status --short
git diff --check
git log --oneline main..HEAD
git diff --stat main...HEAD
```

Expected: clean tree, no whitespace errors, focused commits only.

- [ ] **Step 2: Run final required verification**

Run:

```bash
cargo fmt --all -- --check
cargo test -p buzz-acp hermes_memory --no-fail-fast
cargo test -p buzz-acp acp::tests --no-fail-fast
cargo test -p buzz-acp config::tests --no-fail-fast
cargo test --manifest-path desktop/src-tauri/Cargo.toml --lib hermes_memory --no-fail-fast
pnpm run check
```

Expected: all pass or only explicitly proven pre-existing failures.

- [ ] **Step 3: Open the Pull Request**

Title:

```text
feat(desktop): isolate Hermes memory by persona and channel
```

PR body must include:

- summary of scope and architecture;
- security/privacy properties;
- exact tests executed and results;
- explicit non-goals;
- migration note that existing global Hermes memory remains untouched;
- statement that no relay protocol changes are included.

- [ ] **Step 4: Fetch the PR and verify metadata**

Confirm base `main`, head `feature/hermes-persona-channel-memory`, draft state as intended, and changed files limited to the scoped feature.
