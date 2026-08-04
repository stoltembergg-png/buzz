//! Runtime scope context — stable identity propagation (PR-004)
//!
//! Defines `RuntimeScopeContext`, a value-type carrying the canonical
//! identity of an agent runtime: relay/community, channel, persona, and
//! runtime kind. Stable identifiers (UUIDs, coordinates, normalized URLs)
//! are used instead of display names so the same logical scope produces
//! the same identity across entrypoints.
//!
//! This module is intentionally side-effect-free: it does not read
//! environment variables, files, or the network. Construction validates
//! inputs and serializes with a deterministic JSON form that excludes
//! sensitive fields.
//!
//! Spec: docs/roadmap/prs/PR-004-stable-runtime-context.md

use serde::{Deserialize, Serialize};
use std::fmt;

/// Canonical identity of an agent's runtime scope.
///
/// # Invariants
///
/// - `relay_url` is normalized: scheme lowercased, host lowercased, default
///   port stripped, trailing slash removed, no query/fragment.
/// - `channel_id` is a stable UUID or coordinate (lowercased ASCII).
/// - `persona_id` is a stable UUID or fallback canônico documentado.
/// - `runtime` is one of the known runtime kinds.
/// - `display_*` fields are human-readable labels, NOT used for identity.
///
/// Equality is derived from the stable fields only. Two contexts built with
/// different display names but identical stable fields compare equal.
#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct RuntimeScopeContext {
    /// Normalized relay/community URL (e.g. `wss://relay.example.com`).
    pub relay_url: String,
    /// Stable channel identifier (UUID or coordinate).
    pub channel_id: String,
    /// Stable persona identifier (UUID or canonical fallback).
    pub persona_id: String,
    /// Runtime kind.
    pub runtime: RuntimeKind,
    /// Human-readable display name for the channel (not used for identity).
    #[serde(default)]
    pub display_channel: String,
    /// Human-readable display name for the persona (not used for identity).
    #[serde(default)]
    pub display_persona: String,
}

// PartialEq compares stable identity fields only — display names are NOT identity.
impl PartialEq for RuntimeScopeContext {
    fn eq(&self, other: &Self) -> bool {
        self.relay_url == other.relay_url
            && self.channel_id == other.channel_id
            && self.persona_id == other.persona_id
            && self.runtime == other.runtime
    }
}

impl std::hash::Hash for RuntimeScopeContext {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.relay_url.hash(state);
        self.channel_id.hash(state);
        self.persona_id.hash(state);
        self.runtime.hash(state);
    }
}

/// Supported runtime kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RuntimeKind {
    /// The native Buzz ACP runtime.
    BuzzAcp,
    /// The Hermes ACP runtime (hermes-acp / hermes / hermes-agent).
    Hermes,
}

impl fmt::Display for RuntimeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeKind::BuzzAcp => f.write_str("buzz-acp"),
            RuntimeKind::Hermes => f.write_str("hermes"),
        }
    }
}

/// Error produced when constructing a `RuntimeScopeContext` from invalid inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeScopeError {
    /// The relay URL could not be parsed as a valid URL with scheme.
    InvalidRelayUrl { url: String, reason: String },
    /// The channel ID is empty or contains characters outside [a-z0-9-].
    InvalidChannelId { id: String, reason: String },
    /// The persona ID is empty or contains characters outside [a-z0-9-].
    InvalidPersonaId { id: String, reason: String },
}

impl fmt::Display for RuntimeScopeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeScopeError::InvalidRelayUrl { url, reason } =>
                write!(f, "invalid relay URL '{}': {}", url, reason),
            RuntimeScopeError::InvalidChannelId { id, reason } =>
                write!(f, "invalid channel ID '{}': {}", id, reason),
            RuntimeScopeError::InvalidPersonaId { id, reason } =>
                write!(f, "invalid persona ID '{}': {}", id, reason),
        }
    }
}

impl std::error::Error for RuntimeScopeError {}

/// Normalize a relay URL: lowercase scheme + host, strip default port,
/// strip trailing slash, reject query/fragment.
fn normalize_relay_url(raw: &str) -> Result<String, RuntimeScopeError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(RuntimeScopeError::InvalidRelayUrl {
            url: raw.to_string(),
            reason: "empty".to_string(),
        });
    }
    if trimmed.contains('?') || trimmed.contains('#') {
        return Err(RuntimeScopeError::InvalidRelayUrl {
            url: raw.to_string(),
            reason: "query/fragment not allowed".to_string(),
        });
    }

    let (scheme, rest) = trimmed.split_once("://").ok_or_else(|| RuntimeScopeError::InvalidRelayUrl {
        url: raw.to_string(),
        reason: "missing scheme".to_string(),
    })?;

    let scheme_lower = scheme.to_ascii_lowercase();
    if scheme_lower != "wss" && scheme_lower != "ws" && scheme_lower != "https" && scheme_lower != "http" {
        return Err(RuntimeScopeError::InvalidRelayUrl {
            url: raw.to_string(),
            reason: format!("unsupported scheme '{}'", scheme_lower),
        });
    }

    // Strip trailing slash
    let rest_no_slash = rest.trim_end_matches('/');

    // Split host:port and path
    let (authority, path) = match rest_no_slash.find('/') {
        Some(i) => (&rest_no_slash[..i], &rest_no_slash[i..]),
        None => (rest_no_slash, ""),
    };

    let (host, port) = match authority.rfind(':') {
        Some(i) if authority[i+1..].chars().all(|c| c.is_ascii_digit()) =>
            (&authority[..i], Some(&authority[i+1..])),
        _ => (authority, None),
    };

    // Strip default port
    let default_port = match scheme_lower.as_str() {
        "wss" | "https" => Some("443"),
        "ws" | "http" => Some("80"),
        _ => None,
    };
    let port_stripped = match (port, default_port) {
        (Some(p), Some(d)) if p == d => String::new(),
        (Some(p), _) => format!(":{}", p),
        (None, _) => String::new(),
    };

    let host_lower = host.to_ascii_lowercase();
    if host_lower.is_empty() {
        return Err(RuntimeScopeError::InvalidRelayUrl {
            url: raw.to_string(),
            reason: "empty host".to_string(),
        });
    }

    Ok(format!("{}://{}{}{}", scheme_lower, host_lower, port_stripped, path))
}

/// Validate and normalize a stable identifier (UUID or coordinate).
fn validate_stable_id(field: &str, raw: &str) -> Result<String, RuntimeScopeError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(match field {
            "channel" => RuntimeScopeError::InvalidChannelId {
                id: raw.to_string(),
                reason: "empty".to_string(),
            },
            "persona" => RuntimeScopeError::InvalidPersonaId {
                id: raw.to_string(),
                reason: "empty".to_string(),
            },
            _ => RuntimeScopeError::InvalidRelayUrl {
                url: raw.to_string(),
                reason: "empty".to_string(),
            },
        });
    }
    let lower = trimmed.to_ascii_lowercase();
    if !lower.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        return Err(match field {
            "channel" => RuntimeScopeError::InvalidChannelId {
                id: raw.to_string(),
                reason: "contains characters outside [a-z0-9-_]".to_string(),
            },
            "persona" => RuntimeScopeError::InvalidPersonaId {
                id: raw.to_string(),
                reason: "contains characters outside [a-z0-9-_]".to_string(),
            },
            _ => RuntimeScopeError::InvalidRelayUrl {
                url: raw.to_string(),
                reason: "invalid id".to_string(),
            },
        });
    }
    Ok(lower)
}

impl RuntimeScopeContext {
    /// Canonical fallback persona ID used when a persona is not yet chosen.
    pub const FALLBACK_PERSONA_ID: &'static str = "default-persona";

    /// Construct a context with explicit display names. Display names are
    /// stored for human inspection but do not affect equality.
    pub fn new(
        relay_url: &str,
        channel_id: &str,
        persona_id: &str,
        runtime: RuntimeKind,
    ) -> Result<Self, RuntimeScopeError> {
        Self::with_display(relay_url, channel_id, persona_id, runtime, "", "")
    }

    /// Construct a context with display names.
    pub fn with_display(
        relay_url: &str,
        channel_id: &str,
        persona_id: &str,
        runtime: RuntimeKind,
        display_channel: &str,
        display_persona: &str,
    ) -> Result<Self, RuntimeScopeError> {
        let relay = normalize_relay_url(relay_url)?;
        let channel = validate_stable_id("channel", channel_id)?;
        let persona = validate_stable_id("persona", persona_id)?;
        Ok(Self {
            relay_url: relay,
            channel_id: channel,
            persona_id: persona,
            runtime,
            display_channel: display_channel.to_string(),
            display_persona: display_persona.to_string(),
        })
    }

    /// Stable hash suitable for filesystem paths (REQ-MEM-405 canonical fallback).
    pub fn path_hash(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        self.relay_url.hash(&mut h);
        self.channel_id.hash(&mut h);
        self.persona_id.hash(&mut h);
        self.runtime.hash(&mut h);
        format!("{:016x}", h.finish())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- REQ-MEM-401: stable identifiers, not display names ---

    #[test]
    fn test_display_name_does_not_change_identity() {
        let a = RuntimeScopeContext::with_display(
            "wss://relay.example.com/",
            "11111111-1111-1111-1111-111111111111",
            "22222222-2222-2222-2222-222222222222",
            RuntimeKind::BuzzAcp,
            "Display Channel A",
            "Display Persona A",
        ).unwrap();
        let b = RuntimeScopeContext::with_display(
            "WSS://Relay.Example.com",
            "11111111-1111-1111-1111-111111111111",
            "22222222-2222-2222-2222-222222222222",
            RuntimeKind::BuzzAcp,
            "Display Channel B (different!)",
            "Display Persona B (different!)",
        ).unwrap();
        assert_eq!(a, b, "REQ-MEM-401 violated: display name altered identity");
    }

    // --- REQ-MEM-402: same context from different entrypoints ---

    #[test]
    fn test_same_inputs_equal_across_entrypoints() {
        let from_onboarding = RuntimeScopeContext::new(
            "wss://relay.example.com",
            "channel-001",
            "persona-001",
            RuntimeKind::Hermes,
        ).unwrap();
        let from_restart = RuntimeScopeContext::new(
            "wss://relay.example.com",
            "channel-001",
            "persona-001",
            RuntimeKind::Hermes,
        ).unwrap();
        assert_eq!(from_onboarding, from_restart);
    }

    // --- REQ-MEM-403: relay URL is normalized ---

    #[test]
    fn test_relay_url_normalization() {
        let cases = [
            ("WSS://Relay.Example.com", "wss://relay.example.com"),
            ("wss://relay.example.com:443", "wss://relay.example.com"),
            ("wss://relay.example.com/", "wss://relay.example.com"),
            ("HTTPS://Relay.Example.COM/path", "https://relay.example.com/path"),
            ("ws://relay.example.com:80", "ws://relay.example.com"),
        ];
        for (input, expected) in cases {
            let ctx = RuntimeScopeContext::new(
                input, "c", "p", RuntimeKind::BuzzAcp,
            ).unwrap();
            assert_eq!(ctx.relay_url, expected, "failed for input '{}'", input);
        }
    }

    #[test]
    fn test_relay_url_rejects_invalid() {
        let bad = ["", "no-scheme", "ftp://relay", "wss://relay?q=1", "wss://relay#frag"];
        for input in bad {
            let result = RuntimeScopeContext::new(input, "c", "p", RuntimeKind::BuzzAcp);
            assert!(result.is_err(), "should have rejected '{}'", input);
        }
    }

    // --- REQ-MEM-404: channel ID is stable UUID/coordinate ---

    #[test]
    fn test_channel_id_validation() {
        let ok = RuntimeScopeContext::new(
            "wss://relay", "abc-123", "p", RuntimeKind::BuzzAcp,
        );
        assert!(ok.is_ok());

        let bad_chars = RuntimeScopeContext::new(
            "wss://relay", "abc 123", "p", RuntimeKind::BuzzAcp,
        );
        assert!(matches!(bad_chars, Err(RuntimeScopeError::InvalidChannelId { .. })));

        let empty = RuntimeScopeContext::new(
            "wss://relay", "", "p", RuntimeKind::BuzzAcp,
        );
        assert!(matches!(empty, Err(RuntimeScopeError::InvalidChannelId { .. })));
    }

    // --- REQ-MEM-405: persona fallback is canonical ---

    #[test]
    fn test_fallback_persona() {
        let ctx = RuntimeScopeContext::new(
            "wss://relay", "c", RuntimeScopeContext::FALLBACK_PERSONA_ID, RuntimeKind::BuzzAcp,
        ).unwrap();
        assert_eq!(ctx.persona_id, RuntimeScopeContext::FALLBACK_PERSONA_ID);
    }

    // --- REQ-MEM-406: context does not contain secrets/prompts/memory ---

    #[test]
    fn test_serialized_context_has_no_secret_fields() {
        let ctx = RuntimeScopeContext::with_display(
            "wss://relay.example.com",
            "channel-001",
            "persona-001",
            RuntimeKind::Hermes,
            "Display Channel",
            "Display Persona",
        ).unwrap();
        let json = serde_json::to_string(&ctx).unwrap();
        // Forbid known sensitive keys in the JSON representation.
        for forbidden in ["token", "secret", "password", "api_key", "prompt", "memory", "key"] {
            assert!(
                !json.to_lowercase().contains(forbidden),
                "serialized context leaked forbidden field: {}",
                forbidden
            );
        }
        // The serialization only contains the stable identity fields.
        assert!(json.contains("relay_url"));
        assert!(json.contains("channel_id"));
        assert!(json.contains("persona_id"));
        assert!(json.contains("runtime"));
    }

    // --- Different stable inputs change the identity correctly ---

    #[test]
    fn test_different_relay_changes_identity() {
        let a = RuntimeScopeContext::new("wss://a.example", "c", "p", RuntimeKind::BuzzAcp).unwrap();
        let b = RuntimeScopeContext::new("wss://b.example", "c", "p", RuntimeKind::BuzzAcp).unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn test_different_channel_changes_identity() {
        let a = RuntimeScopeContext::new("wss://relay", "channel-a", "p", RuntimeKind::BuzzAcp).unwrap();
        let b = RuntimeScopeContext::new("wss://relay", "channel-b", "p", RuntimeKind::BuzzAcp).unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn test_different_persona_changes_identity() {
        let a = RuntimeScopeContext::new("wss://relay", "c", "persona-a", RuntimeKind::BuzzAcp).unwrap();
        let b = RuntimeScopeContext::new("wss://relay", "c", "persona-b", RuntimeKind::BuzzAcp).unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn test_different_runtime_changes_identity() {
        let a = RuntimeScopeContext::new("wss://relay", "c", "p", RuntimeKind::BuzzAcp).unwrap();
        let b = RuntimeScopeContext::new("wss://relay", "c", "p", RuntimeKind::Hermes).unwrap();
        assert_ne!(a, b);
    }

    // --- path_hash is deterministic ---

    #[test]
    fn test_path_hash_is_deterministic_and_changes_with_inputs() {
        let a = RuntimeScopeContext::new("wss://relay", "c", "p", RuntimeKind::BuzzAcp).unwrap();
        let b = RuntimeScopeContext::new("wss://relay", "c", "p", RuntimeKind::BuzzAcp).unwrap();
        assert_eq!(a.path_hash(), b.path_hash());

        let c = RuntimeScopeContext::new("wss://relay", "c2", "p", RuntimeKind::BuzzAcp).unwrap();
        assert_ne!(a.path_hash(), c.path_hash());
    }
}