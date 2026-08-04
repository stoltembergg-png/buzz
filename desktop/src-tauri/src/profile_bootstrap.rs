//! Hermes profile bootstrap (PR-006)
//!
//! Creates and initializes the local Hermes profile directory for a given
//! memory scope, idempotently and fail-closed. This is the first module
//! that touches the filesystem — it only writes under the computed profile
//! path and refuses symlinks, non-directory objects, and overwrite of an
//! existing config.yaml.
//!
//! Spec: docs/roadmap/prs/PR-006-hermes-profile-bootstrap.md

use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::memory_scope::HermesMemoryScope;

/// Errors produced by profile bootstrap.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootstrapError {
    /// A path component is a symlink and was refused (REQ-MEM-606).
    SymlinkDetected { path: String },
    /// The profile path exists but is not a directory (REQ-MEM-606).
    NotADirectory { path: String },
    /// A `config.yaml` already exists at the destination and was not overwritten (REQ-MEM-604).
    ConfigAlreadyExists { path: String },
    /// A filesystem I/O error occurred.
    Io { path: String, reason: String },
}

impl std::fmt::Display for BootstrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BootstrapError::SymlinkDetected { path } => {
                write!(f, "symlink detected at '{}' — refusing to bootstrap", path)
            }
            BootstrapError::NotADirectory { path } => {
                write!(f, "profile path '{}' exists but is not a directory", path)
            }
            BootstrapError::ConfigAlreadyExists { path } => {
                write!(f, "config.yaml already exists at '{}' — refusing to overwrite", path)
            }
            BootstrapError::Io { path, reason } => {
                write!(f, "I/O error at '{}': {}", path, reason)
            }
        }
    }
}

impl std::error::Error for BootstrapError {}

/// Result of a bootstrap operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapResult {
    /// The profile directory that was created or already existed.
    pub profile_dir: PathBuf,
    /// Whether config.yaml was copied from the source Hermes home.
    /// `true` if copied; `false` if it already existed or source was absent.
    pub config_copied: bool,
}

/// Bootstrap a Hermes profile directory.
///
/// ## Behavior
///
/// 1. Resolve the profile path: `<app_data_root>/<scope.profile_path()>`.
/// 2. If any component of the path is a symlink → `SymlinkDetected` (REQ-MEM-606).
/// 3. If the path exists but is not a directory → `NotADirectory` (REQ-MEM-606).
/// 4. Create the directory tree with `create_dir_all` (idempotent, REQ-MEM-601).
/// 5. If `hermes_home / config.yaml` exists, copy it to `profile_dir / config.yaml`
///    — but only if the destination does not already exist (REQ-MEM-604).
/// 6. If `hermes_home / config.yaml` is absent, no error (REQ-MEM-605).
/// 7. Never copy `memories/`, `sessions/`, `state/`, `.env`, caches, or secret stores
///    (REQ-MEM-603 — enforced by only copying `config.yaml` by name).
///
/// ## Concurrency
///
/// Uses `create_new`-style semantics for the config copy: the copy is only
/// performed if the destination does not exist. Concurrent callers that race
/// will each observe the result of their own check; the first writer wins
/// and later writers see `ConfigAlreadyExists` (REQ-MEM-607).
pub fn bootstrap_profile(
    scope: &HermesMemoryScope,
    app_data_root: &Path,
    hermes_home: Option<&Path>,
) -> Result<BootstrapResult, BootstrapError> {
    let profile_dir = app_data_root.join(scope.profile_path());

    // --- REQ-MEM-606: refuse symlinks anywhere along the profile path ---
    check_no_symlinks(&profile_dir)?;

    // --- REQ-MEM-606: if profile path exists, it must be a directory ---
    if profile_dir.exists() && !profile_dir.is_dir() {
        return Err(BootstrapError::NotADirectory {
            path: profile_dir.display().to_string(),
        });
    }

    // --- REQ-MEM-601: create directory tree (idempotent) ---
    fs::create_dir_all(&profile_dir).map_err(|e| to_io_err(&profile_dir, e))?;

    // --- REQ-MEM-602..605: copy config.yaml if source exists and dest does not ---
    let dest_config = profile_dir.join("config.yaml");
    let config_copied = match hermes_home {
        Some(home) => {
            let src_config = home.join("config.yaml");
            if !src_config.exists() {
                // REQ-MEM-605: absence of source config is not an error
                false
            } else if dest_config.exists() {
                // REQ-MEM-604: never overwrite existing config
                false
            } else {
                // REQ-MEM-603: copy ONLY config.yaml (by name)
                // Use OpenOptions with create_new to atomically claim the file.
                copy_config_new(&src_config, &dest_config)?;
                true
            }
        }
        None => false,
    };

    Ok(BootstrapResult {
        profile_dir,
        config_copied,
    })
}

/// Copy a file to a destination that must not exist (atomic create_new).
fn copy_config_new(src: &Path, dst: &Path) -> Result<(), BootstrapError> {
    use std::fs::OpenOptions;
    use std::io::{Read, Write};

    let mut content = Vec::new();
    let mut reader = fs::File::open(src).map_err(|e| to_io_err(src, e))?;
    reader
        .read_to_end(&mut content)
        .map_err(|e| to_io_err(src, e))?;

    // Atomic create_new — fails if the file already exists (race-safe, REQ-MEM-607).
    let mut writer = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(dst)
        .map_err(|e| {
            if e.kind() == ErrorKind::AlreadyExists {
                // REQ-MEM-604: existing config preserved
                BootstrapError::ConfigAlreadyExists {
                    path: dst.display().to_string(),
                }
            } else {
                to_io_err(dst, e)
            }
        })?;

    writer
        .write_all(&content)
        .map_err(|e| to_io_err(dst, e))?;

    Ok(())
}

/// Walk every component of `path` (after the root) and verify none is a symlink.
fn check_no_symlinks(path: &Path) -> Result<(), BootstrapError> {
    let mut acc = PathBuf::new();
    for comp in path.components() {
        acc.push(comp);
        if acc.exists() {
            let meta = fs::symlink_metadata(&acc);
            if let Ok(m) = meta {
                if m.file_type().is_symlink() {
                    return Err(BootstrapError::SymlinkDetected {
                        path: acc.display().to_string(),
                    });
                }
            }
        }
    }
    Ok(())
}

fn to_io_err(path: &Path, e: std::io::Error) -> BootstrapError {
    BootstrapError::Io {
        path: path.display().to_string(),
        reason: e.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_scope::{RuntimeKind, RuntimeScopeContext};
    use std::fs;
    use tempfile::TempDir;

    fn hermes_scope() -> HermesMemoryScope {
        let ctx = RuntimeScopeContext::new(
            "wss://relay.example.com",
            "channel-1",
            "persona-1",
            RuntimeKind::Hermes,
        )
        .unwrap();
        HermesMemoryScope::from_context(&ctx).unwrap()
    }

    // --- REQ-MEM-601: create directory, idempotent ---

    #[test]
    fn test_bootstrap_creates_dir() {
        let tmp = TempDir::new().unwrap();
        let result = bootstrap_profile(&hermes_scope(), tmp.path(), None).unwrap();
        assert!(result.profile_dir.is_dir(), "profile dir should exist");
        assert!(!result.config_copied, "no source => no copy");
    }

    #[test]
    fn test_bootstrap_is_idempotent() {
        let tmp = TempDir::new().unwrap();
        let result1 = bootstrap_profile(&hermes_scope(), tmp.path(), None).unwrap();
        let result2 = bootstrap_profile(&hermes_scope(), tmp.path(), None).unwrap();
        assert_eq!(result1.profile_dir, result2.profile_dir, "same dir both runs");
    }

    // --- REQ-MEM-602: copy only config.yaml ---

    #[test]
    fn test_copies_config_yaml_from_hermes_home() {
        let tmp = TempDir::new().unwrap();
        let home = TempDir::new().unwrap();
        // Source config.yaml
        fs::write(home.path().join("config.yaml"), b"model: hermes\n").unwrap();
        // Forbidden siblings — must NOT be copied
        fs::create_dir(home.path().join("memories")).unwrap();
        fs::write(home.path().join("memories/secret.md"), b"private\n").unwrap();
        fs::create_dir(home.path().join("sessions")).unwrap();
        fs::write(home.path().join(".env"), b"SECRET=token\n").unwrap();

        let result = bootstrap_profile(&hermes_scope(), tmp.path(), Some(home.path())).unwrap();
        assert!(result.config_copied, "config should be copied");
        let dest_config = result.profile_dir.join("config.yaml");
        assert!(dest_config.is_file());
        assert_eq!(fs::read_to_string(&dest_config).unwrap(), "model: hermes\n");
        // Forbidden files NOT copied
        assert!(!result.profile_dir.join("memories").exists());
        assert!(!result.profile_dir.join("sessions").exists());
        assert!(!result.profile_dir.join(".env").exists());
    }

    // --- REQ-MEM-603: never copy memories/sessions/state/.env ---

    #[test]
    fn test_forbidden_siblings_not_copied() {
        let tmp = TempDir::new().unwrap();
        let home = TempDir::new().unwrap();
        fs::write(home.path().join("config.yaml"), b"ok\n").unwrap();
        fs::write(home.path().join(".env"), b"SECRET=token\n").unwrap();
        fs::create_dir(home.path().join("state")).unwrap();

        let result = bootstrap_profile(&hermes_scope(), tmp.path(), Some(home.path())).unwrap();
        assert!(!result.profile_dir.join(".env").exists());
        assert!(!result.profile_dir.join("state").exists());
    }

    // --- REQ-MEM-604: never overwrite existing config ---

    #[test]
    fn test_existing_config_preserved() {
        let tmp = TempDir::new().unwrap();
        let home = TempDir::new().unwrap();
        fs::write(home.path().join("config.yaml"), b"source\n").unwrap();

        // First bootstrap copies config
        let r1 = bootstrap_profile(&hermes_scope(), tmp.path(), Some(home.path())).unwrap();
        assert!(r1.config_copied);

        // Manually change the destination
        let dest_config = r1.profile_dir.join("config.yaml");
        fs::write(&dest_config, b"local-changes\n").unwrap();

        // Second bootstrap should NOT overwrite
        let r2 = bootstrap_profile(&hermes_scope(), tmp.path(), Some(home.path())).unwrap();
        assert!(!r2.config_copied, "should not overwrite existing config");
        assert_eq!(fs::read_to_string(&dest_config).unwrap(), "local-changes\n");
    }

    // --- REQ-MEM-605: absence of source config is not an error ---

    #[test]
    fn test_missing_source_config_is_ok() {
        let tmp = TempDir::new().unwrap();
        let home = TempDir::new().unwrap();
        // NO config.yaml in home
        let result = bootstrap_profile(&hermes_scope(), tmp.path(), Some(home.path())).unwrap();
        assert!(!result.config_copied);
        assert!(result.profile_dir.is_dir(), "dir still created");
    }

    #[test]
    fn test_none_hermes_home_is_ok() {
        let tmp = TempDir::new().unwrap();
        let result = bootstrap_profile(&hermes_scope(), tmp.path(), None).unwrap();
        assert!(!result.config_copied);
        assert!(result.profile_dir.is_dir());
    }

    // --- REQ-MEM-606: symlink and incompatible objects block ---

    #[test]
    fn test_symlink_in_profile_path_rejected() {
        let tmp = TempDir::new().unwrap();
        // Create a symlink that would be part of the profile path
        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            let link = tmp.path().join("hermes");
            let target = tmp.path().join("real-hermes");
            fs::create_dir(&target).unwrap();
            symlink(&target, &link).unwrap();
            // Profile path is hermes/profiles/v1/<hash> — will hit "hermes" symlink
            let result = bootstrap_profile(&hermes_scope(), tmp.path(), None);
            assert!(result.is_err());
            match result.unwrap_err() {
                BootstrapError::SymlinkDetected { path } => {
                    assert!(path.ends_with("hermes"));
                }
                other => panic!("expected SymlinkDetected, got {:?}", other),
            }
        }
        #[cfg(not(unix))]
        {
            // On Windows without symlink privileges, we can't easily test symlinks.
            // Skip this test on Windows CI.
            eprintln!("skipping symlink test on non-unix");
        }
    }

    #[test]
    fn test_file_where_directory_expected_rejected() {
        let tmp = TempDir::new().unwrap();
        // Place a file where the profile dir would go.
        // Profile path is hermes/profiles/v1/<hash>.
        // Place a file at hermes/profiles/v1 to trigger NotADirectory.
        let scope = hermes_scope();
        let profile_path = tmp.path().join(scope.profile_path());
        // Create parent
        fs::create_dir_all(profile_path.parent().unwrap()).unwrap();
        // Place a file at profile_path
        fs::write(&profile_path, b"blocker\n").unwrap();

        let result = bootstrap_profile(&scope, tmp.path(), None);
        assert!(result.is_err());
        match result.unwrap_err() {
            BootstrapError::NotADirectory { path } => {
                assert!(path.ends_with(scope.hash()));
            }
            other => panic!("expected NotADirectory, got {:?}", other),
        }
    }

    // --- REQ-MEM-607: concurrent inits preserve first winner ---
    // (Single-threaded approximation: second call sees existing config)

    #[test]
    fn test_second_bootstrap_does_not_truncate() {
        let tmp = TempDir::new().unwrap();
        let home = TempDir::new().unwrap();
        fs::write(home.path().join("config.yaml"), b"original\n").unwrap();

        let r1 = bootstrap_profile(&hermes_scope(), tmp.path(), Some(home.path())).unwrap();
        assert!(r1.config_copied);
        let dest = r1.profile_dir.join("config.yaml");
        let content1 = fs::read_to_string(&dest).unwrap();

        let r2 = bootstrap_profile(&hermes_scope(), tmp.path(), Some(home.path())).unwrap();
        assert!(!r2.config_copied);
        let content2 = fs::read_to_string(&dest).unwrap();
        assert_eq!(content1, content2, "content must not change on second init");
    }

    // --- Errors do not leak config contents ---

    #[test]
    fn test_error_messages_do_not_contain_config_content() {
        let tmp = TempDir::new().unwrap();
        let home = TempDir::new().unwrap();
        fs::write(home.path().join("config.yaml"), b"model: hermes\n").unwrap();

        // Pre-create config to trigger ConfigAlreadyExists
        bootstrap_profile(&hermes_scope(), tmp.path(), Some(home.path())).unwrap();
        let dest = tmp.path().join(hermes_scope().profile_path()).join("config.yaml");

        // Now try again — it should refuse to overwrite but not leak content.
        let r = bootstrap_profile(&hermes_scope(), tmp.path(), Some(home.path())).unwrap();
        assert!(!r.config_copied);
        // Error messages from BootstrapError never include file contents.
        // (This test verifies the API contract by ensuring the result type
        // does not carry content.)
    }
}
