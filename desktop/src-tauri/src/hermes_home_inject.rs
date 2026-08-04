//! Inject HERMES_HOME at spawn (PR-007)
//!
//! Computes the environment overlay that should be applied to a Hermes
//! runtime child process: `HERMES_HOME` is set to the scoped profile path
//! computed from the runtime scope context. Non-Hermes runtimes receive
//! no overlay (their env is left untouched).
//!
//! This module is intentionally a pure function — it does not spawn
//! processes, read environment variables, or touch the filesystem. The
//! caller is responsible for merging the returned overlay into the
//! child's environment before exec.
//!
//! Spec: docs/roadmap/prs/PR-007-inject-hermes-home.md

use std::path::PathBuf;

use crate::memory_scope::{HermesMemoryScope, MemoryScopeError};
use crate::profile_bootstrap::{bootstrap_profile, BootstrapError, BootstrapResult};
use crate::runtime_scope::{RuntimeKind, RuntimeScopeContext};

/// The environment variable name that controls the Hermes home directory.
pub const HERMES_HOME_VAR: &str = "HERMES_HOME";

/// Error returned by `compute_hermes_env_overlay`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InjectError {
    /// The runtime is not a Hermes alias (REQ-MEM-704).
    NotHermesRuntime { runtime: RuntimeKind },
    /// The memory scope could not be derived.
    Scope(MemoryScopeError),
    /// The profile directory bootstrap failed (REQ-MEM-706).
    Bootstrap(BootstrapError),
}

impl std::fmt::Display for InjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InjectError::NotHermesRuntime { runtime } => write!(
                f,
                "HERMES_HOME injection is only available for Hermes runtimes; got {}",
                runtime
            ),
            InjectError::Scope(e) => write!(f, "memory scope error: {}", e),
            InjectError::Bootstrap(e) => write!(f, "profile bootstrap failed: {}", e),
        }
    }
}

impl std::error::Error for InjectError {}

impl From<MemoryScopeError> for InjectError {
    fn from(e: MemoryScopeError) -> Self {
        InjectError::Scope(e)
    }
}

impl From<BootstrapError> for InjectError {
    fn from(e: BootstrapError) -> Self {
        InjectError::Bootstrap(e)
    }
}

/// The overlay to apply to a child process environment.
///
/// `None` means "do not touch the environment" (used for non-Hermes runtimes,
/// REQ-MEM-704). `Some(map)` means "set these vars; do not remove others".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvOverlay {
    /// The variable name (always `HERMES_HOME`).
    pub var: String,
    /// The scoped profile path.
    pub value: PathBuf,
    /// The bootstrap result (dir was created/exists, config copied flag).
    pub bootstrap: BootstrapResult,
}

/// Compute the environment overlay for a Hermes runtime spawn.
///
/// ## Behavior
///
/// - Non-Hermes runtimes return `Ok(None)` — their env is NOT modified,
///   and no existing `HERMES_HOME` is removed (REQ-MEM-704).
/// - Hermes runtimes derive a `HermesMemoryScope` from the context.
/// - The profile directory is bootstrapped before the spawn (REQ-MEM-701).
/// - `HERMES_HOME` is set to the profile directory (REQ-MEM-702).
/// - If the parent process already has `HERMES_HOME` set, the scoped value
///   OVERRIDES the inherited one — the caller must insert this overlay AFTER
///   inheriting from the parent (REQ-MEM-703).
/// - If bootstrap fails, the spawn MUST NOT proceed (REQ-MEM-706).
pub fn compute_hermes_env_overlay(
    ctx: &RuntimeScopeContext,
    app_data_root: &std::path::Path,
    hermes_home: Option<&std::path::Path>,
) -> Result<Option<EnvOverlay>, InjectError> {
    if ctx.runtime != RuntimeKind::Hermes {
        // REQ-MEM-704: non-Hermes runtimes receive no overlay.
        return Ok(None);
    }

    let scope = HermesMemoryScope::from_context(ctx)?;

    // REQ-MEM-701: bootstrap the profile directory before spawn.
    let bootstrap = bootstrap_profile(&scope, app_data_root, hermes_home)?;

    // REQ-MEM-702: set HERMES_HOME to the scoped profile directory.
    Ok(Some(EnvOverlay {
        var: HERMES_HOME_VAR.to_string(),
        value: bootstrap.profile_dir.clone(),
        bootstrap,
    }))
}

/// Merge an env overlay into an existing environment map.
///
/// The overlay is applied on top of the existing map — i.e., if the parent
/// process had `HERMES_HOME` set, the overlay value replaces it (REQ-MEM-703).
/// If the overlay is `None` (non-Hermes runtime), the map is returned unchanged
/// (REQ-MEM-704: no removal of existing `HERMES_HOME`).
pub fn apply_overlay(
    env: &mut Vec<(String, String)>,
    overlay: Option<&EnvOverlay>,
) {
    if let Some(o) = overlay {
        // Remove any existing entry for this var (parent inheritance).
        env.retain(|(k, _)| k != &o.var);
        // Append the scoped value — overrides the parent's setting.
        env.push((o.var.clone(), o.value.display().to_string()));
    }
    // If overlay is None, env is unchanged (non-Hermes preserves inherited env).
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_scope::{RuntimeKind, RuntimeScopeContext};
    use std::fs;
    use tempfile::TempDir;

    fn hermes_ctx() -> RuntimeScopeContext {
        RuntimeScopeContext::new(
            "wss://relay.example.com",
            "channel-1",
            "persona-1",
            RuntimeKind::Hermes,
        )
        .unwrap()
    }

    fn buzz_ctx() -> RuntimeScopeContext {
        RuntimeScopeContext::new(
            "wss://relay.example.com",
            "channel-1",
            "persona-1",
            RuntimeKind::BuzzAcp,
        )
        .unwrap()
    }

    // --- REQ-MEM-702: explicit HERMES_HOME for Hermes aliases ---

    #[test]
    fn test_hermes_runtime_gets_overlay() {
        let tmp = TempDir::new().unwrap();
        let overlay = compute_hermes_env_overlay(&hermes_ctx(), tmp.path(), None).unwrap();
        assert!(overlay.is_some());
        let o = overlay.unwrap();
        assert_eq!(o.var, "HERMES_HOME");
        assert!(o.value.is_dir(), "profile dir should exist after bootstrap");
        assert!(o.value.display().to_string().contains("hermes/profiles/v1/"));
    }

    // --- REQ-MEM-704: non-Hermes runtimes get no overlay ---

    #[test]
    fn test_non_hermes_runtime_gets_no_overlay() {
        let tmp = TempDir::new().unwrap();
        let overlay = compute_hermes_env_overlay(&buzz_ctx(), tmp.path(), None).unwrap();
        assert!(overlay.is_none(), "non-Hermes must not receive HERMES_HOME");
    }

    #[test]
    fn test_apply_overlay_none_preserves_env() {
        let mut env = vec![("HERMES_HOME".to_string(), "/parent/home".to_string())];
        apply_overlay(&mut env, None);
        assert_eq!(env.len(), 1);
        assert_eq!(env[0].1, "/parent/home", "non-Hermes must not remove HERMES_HOME");
    }

    // --- REQ-MEM-703: parent HERMES_HOME does NOT win ---

    #[test]
    fn test_apply_overlay_overrides_parent() {
        let tmp = TempDir::new().unwrap();
        let overlay = compute_hermes_env_overlay(&hermes_ctx(), tmp.path(), None).unwrap().unwrap();
        let mut env = vec![("HERMES_HOME".to_string(), "/parent/home".to_string())];
        apply_overlay(&mut env, Some(&overlay));
        assert_eq!(env.len(), 1, "should have exactly one HERMES_HOME");
        assert_eq!(env[0].0, "HERMES_HOME");
        assert_eq!(env[0].1, overlay.value.display().to_string(), "scoped value must win");
        assert_ne!(env[0].1, "/parent/home", "parent value must NOT win");
    }

    // --- Restart reuse: same context → same home ---

    #[test]
    fn test_restart_reuses_same_home() {
        let tmp = TempDir::new().unwrap();
        let a = compute_hermes_env_overlay(&hermes_ctx(), tmp.path(), None).unwrap().unwrap();
        let b = compute_hermes_env_overlay(&hermes_ctx(), tmp.path(), None).unwrap().unwrap();
        assert_eq!(a.value, b.value, "same context must produce same HERMES_HOME");
    }

    // --- Different channel/persona/relay isolate homes ---

    #[test]
    fn test_different_isolates_homes() {
        let tmp = TempDir::new().unwrap();
        let ctx_a = RuntimeScopeContext::new(
            "wss://relay.example.com", "channel-a", "persona-1", RuntimeKind::Hermes,
        ).unwrap();
        let ctx_b = RuntimeScopeContext::new(
            "wss://relay.example.com", "channel-b", "persona-1", RuntimeKind::Hermes,
        ).unwrap();
        let a = compute_hermes_env_overlay(&ctx_a, tmp.path(), None).unwrap().unwrap();
        let b = compute_hermes_env_overlay(&ctx_b, tmp.path(), None).unwrap().unwrap();
        assert_ne!(a.value, b.value, "different channels must isolate homes");
    }

    // --- REQ-MEM-706: bootstrap failure blocks spawn ---

    #[test]
    fn test_bootstrap_failure_blocks_spawn() {
        // Create a file where the profile dir would go to trigger NotADirectory.
        let tmp = TempDir::new().unwrap();
        let ctx = hermes_ctx();
        let scope = HermesMemoryScope::from_context(&ctx).unwrap();
        let profile_path = tmp.path().join(scope.profile_path());
        fs::create_dir_all(profile_path.parent().unwrap()).unwrap();
        fs::write(&profile_path, b"blocker").unwrap();

        let result = compute_hermes_env_overlay(&ctx, tmp.path(), None);
        assert!(result.is_err());
        match result.unwrap_err() {
            InjectError::Bootstrap(_) => {} // expected
            other => panic!("expected Bootstrap error, got {:?}", other),
        }
    }

    // --- Error does not leak config content ---

    #[test]
    fn test_inject_error_display_is_redacted() {
        let tmp = TempDir::new().unwrap();
        let ctx = hermes_ctx();
        let scope = HermesMemoryScope::from_context(&ctx).unwrap();
        let profile_path = tmp.path().join(scope.profile_path());
        fs::create_dir_all(profile_path.parent().unwrap()).unwrap();
        fs::write(&profile_path, b"secret-config-content-12345").unwrap();

        let err = compute_hermes_env_overlay(&ctx, tmp.path(), None).unwrap_err();
        let msg = format!("{}", err);
        // The error should mention "not a directory" but NOT leak file content.
        assert!(!msg.contains("secret-config-content-12345"), "error leaked config content: {}", msg);
    }
}
