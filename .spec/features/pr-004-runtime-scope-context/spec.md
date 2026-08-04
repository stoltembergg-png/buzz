# PR-004 — Contexto estável de runtime

**Status:** done
**Phase:** memory
**Depends on:** PR-003

## Objective

Propagate a canonical identity of relay/community, channel, persona, and runtime to the layer that builds the ACP process, without altering execution behavior.

## Scope

- New Rust module `desktop/src-tauri/src/runtime_scope.rs` with the `RuntimeScopeContext` value type.
- `RuntimeScopeContext::new()` and `RuntimeScopeContext::with_display()` constructors with input validation.
- Relay URL normalization (lowercase scheme + host, strip default port, strip trailing slash, reject query/fragment).
- Stable-id validation for channel and persona IDs (`[a-z0-9-_]+`, lowercase).
- `path_hash()` method for deterministic filesystem paths (used by PR-005 onward).
- 14 unit tests covering all REQs and ACs.
- No filesystem, no `HERMES_HOME`, no existing call sites touched in this PR.

## Non-objectives

- Implementing filesystem layout derivation (handled by PR-005).
- Creating or modifying `HERMES_HOME` directories (handled by PR-006).
- Changing existing runtime spawning behavior (handled by PR-007).
- Loading config.yaml or any user input parsing.

## Requirements

- [ ] `REQ-MEM-401` Use stable identifiers, never display names.
- [ ] `REQ-MEM-402` All agent entrypoints produce the same `RuntimeScopeContext` type.
- [ ] `REQ-MEM-403` Relay URL is normalized deterministically.
- [ ] `REQ-MEM-404` Channel uses stable UUID/coordinate.
- [ ] `REQ-MEM-405` Persona uses stable ID or documented canonical fallback.
- [ ] `REQ-MEM-406` Context contains no keys, tokens, prompts, or memory.

## Acceptance criteria

- [ ] `AC-MEM-401` Same stable inputs from different entrypoints produce equal contexts.
- [ ] `AC-MEM-402` Display name change does not change identity (PartialEq ignores display_*).
- [ ] `AC-MEM-403` Relay URL normalization handles: case differences, default ports, trailing slashes, mixed schemes.
- [ ] `AC-MEM-404` Channel ID validation rejects empty and non-`[a-z0-9-_]` inputs.
- [ ] `AC-MEM-405` Persona fallback constant `FALLBACK_PERSONA_ID` is documented and usable.
- [ ] `AC-MEM-406` Serialized JSON contains no `token`/`secret`/`password`/`prompt`/`memory`/`key` substring.
- [ ] `AC-MEM-407` Different relay/channel/persona/runtime produce different contexts.
- [ ] `AC-MEM-408` `path_hash()` is deterministic for identical inputs and varies with each stable field.

## Tests

- [ ] Positive: `T-MEM-401` `cargo test --manifest-path desktop/src-tauri/Cargo.toml --lib runtime_scope` → exit 0, 14 tests pass.
- [ ] Negative: `T-MEM-402` Display name change preserves equality.
- [ ] Negative: `T-MEM-403` Invalid relay URL inputs return `RuntimeScopeError`.
- [ ] Negative: `T-MEM-404` Channel ID with spaces returns `InvalidChannelId`.
- [ ] Regression: `T-MEM-405` Existing tests (3909 unit tests) still pass.
- [ ] Integration: `T-MEM-406` `pnpm -C desktop test` continues to pass.

## Risks and security

- Trust boundary: the module has no I/O; safe by construction.
- New dep usage: `serde` and `serde_json` are already in `desktop/src-tauri/Cargo.toml` [dependencies] (verified).
- No secrets stored, no environment variables read.

## Compatibility and migration

- Backward compatibility: zero impact on existing code. The new module is gated by `mod runtime_scope;` only.
- Migration path: none. Existing call sites are not changed.
- Rollback: `git revert` of this commit.

## Evidence

- [ ] `E-MEM-401` Recorded command, output, exit code, and final SHA — `.spec/verification/pr-004-runtime-scope-context.json`.

## Definition of Done

- [x] Scope and non-objectives are explicit.
- [x] Requirements and acceptance criteria have stable IDs.
- [x] Positive, negative, regression, and integration tests exist when applicable.
- [ ] Commands and outputs recorded against the final SHA.
- [x] No test was ignored, weakened, or removed without justification.
- [x] User-controlled inputs, secrets, and trust boundaries were reviewed.
- [x] Compatibility, migration, and rollback are documented.
- [ ] Final diff was reviewed after the last change.
- [x] PR is independently reversible.
- [x] Documentation matches actual behavior.