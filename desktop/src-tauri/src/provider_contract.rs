//! PR-009 — Provider capability contract.
//!
//! Versioned internal types for providers, models, and capabilities.
//! No discovery, UI, or selection logic lives here — this module is purely
//! the data contract that downstream PRs (PR-010..014) build on.
//!
//! # Requirements
//! - REQ-PROV-901: Stable provider/model identity separate from display label.
//! - REQ-PROV-902: Capabilities include context window, reasoning, tools,
//!   images, structured output, cost, and availability.
//! - REQ-PROV-903: Unknown fields are tolerated per documented rule.
//! - REQ-PROV-904: No secrets are part of the contract.
//! - REQ-PROV-905: Rust and TypeScript share equivalent semantics.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Schema version for the provider contract. Incremented on breaking changes.
/// Unknown fields from future versions are kept (REQ-PROV-903).
pub const PROVIDER_CONTRACT_VERSION: u32 = 1;

// ─── Identity ───────────────────────────────────────────────────────

/// Stable, machine-readable provider identifier (REQ-PROV-901).
///
/// Lowercase, kebab-case. Never a display label. Examples: `"openai"`,
/// `"anthropic"`, `"nvidia"`, `"openrouter"`, `"minimax"`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProviderId(String);

impl ProviderId {
    pub fn new(id: impl Into<String>) -> Self {
        let id = id.into();
        // Normalize: lowercase, trimmed
        Self(id.trim().to_lowercase())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    /// Validate that the ID is non-empty and kebab-case.
    pub fn is_valid(&self) -> bool {
        !self.0.is_empty()
            && self.0.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    }
}

impl std::fmt::Display for ProviderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ProviderId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

/// Stable, machine-readable model identifier (REQ-PROV-901).
///
/// Examples: `"gpt-4o"`, `"claude-sonnet-4-20250514"`, `"z-ai/glm-5.2"`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModelId(String);

impl ModelId {
    pub fn new(id: impl Into<String>) -> Self {
        let id = id.into();
        Self(id.trim().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn is_valid(&self) -> bool {
        !self.0.is_empty()
    }
}

impl std::fmt::Display for ModelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ModelId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

// ─── Capabilities ───────────────────────────────────────────────────

/// Model capabilities (REQ-PROV-902).
///
/// All fields are `Option` to tolerate partial data and future unknown
/// fields (REQ-PROV-903). A `None` means "unknown", not "unsupported".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelCapabilities {
    /// Maximum context window in tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_window: Option<u32>,

    /// Whether the model supports reasoning / thinking tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<bool>,

    /// Whether the model supports tool/function calling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<bool>,

    /// Whether the model supports image input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<bool>,

    /// Whether the model supports structured output (JSON mode).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_output: Option<bool>,

    /// Cost per 1M input tokens in USD cents (or provider currency unit).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_per_1m_input: Option<u64>,

    /// Cost per 1M output tokens in USD cents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_per_1m_output: Option<u64>,

    /// Whether the model is currently available (may change at runtime).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
}

impl Default for ModelCapabilities {
    fn default() -> Self {
        Self {
            context_window: None,
            reasoning: None,
            tools: None,
            images: None,
            structured_output: None,
            cost_per_1m_input: None,
            cost_per_1m_output: None,
            available: None,
        }
    }
}

// ─── Provider Entry ─────────────────────────────────────────────────

/// A provider entry in the catalog — identity, display metadata, and
/// the models it offers. No secrets (REQ-PROV-904).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProviderEntry {
    /// Stable provider identity (REQ-PROV-901).
    pub id: ProviderId,

    /// Human-readable label for UI display. Mutable, not used for matching.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Base URL for API calls (no API keys — REQ-PROV-904).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,

    /// Models offered by this provider.
    #[serde(default)]
    pub models: Vec<ModelEntry>,
}

/// A model entry — identity + capabilities + optional display label.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelEntry {
    /// Stable model identity (REQ-PROV-901).
    pub id: ModelId,

    /// Human-readable label for UI display.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Capabilities (REQ-PROV-902).
    #[serde(default)]
    pub capabilities: ModelCapabilities,
}

// ─── Provider Catalog ────────────────────────────────────────────────

/// The full provider catalog — a versioned collection of providers.
/// Unknown top-level fields are preserved for forward compatibility
/// (REQ-PROV-903) via `#[serde(flatten)]`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProviderCatalog {
    /// Schema version.
    pub version: u32,

    /// Provider entries.
    #[serde(default)]
    pub providers: Vec<ProviderEntry>,

    /// Unknown fields from future versions (REQ-PROV-903).
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

impl ProviderCatalog {
    /// Create an empty catalog at the current contract version.
    pub fn empty() -> Self {
        Self {
            version: PROVIDER_CONTRACT_VERSION,
            providers: Vec::new(),
            extra: BTreeMap::new(),
        }
    }

    /// Find a provider by ID.
    pub fn find_provider(&self, id: &ProviderId) -> Option<&ProviderEntry> {
        self.providers.iter().find(|p| &p.id == id)
    }

    /// Find a model within a provider.
    pub fn find_model(&self, provider: &ProviderId, model: &ModelId) -> Option<&ModelEntry> {
        self.find_provider(provider)
            .and_then(|p| p.models.iter().find(|m| &m.id == model))
    }

    /// Check for duplicate provider IDs.
    pub fn has_duplicate_providers(&self) -> bool {
        let mut seen = std::collections::HashSet::new();
        self.providers.iter().any(|p| !seen.insert(&p.id))
    }
}

impl Default for ProviderCatalog {
    fn default() -> Self {
        Self::empty()
    }
}

// ─── Secret Redaction (REQ-PROV-904) ────────────────────────────────

/// Assert that a serialized JSON value contains no secret-like fields.
/// Checks for common secret field names: `api_key`, `secret`, `token`,
/// `password`, `credential`, `private_key`.
pub fn assert_no_secrets_in_json(value: &serde_json::Value) -> Result<(), String> {
    let secret_patterns = [
        "api_key",
        "apikey",
        "secret",
        "token",
        "password",
        "credential",
        "private_key",
        "access_key",
    ];

    fn check_object(
        obj: &serde_json::Map<String, serde_json::Value>,
        patterns: &[&str],
    ) -> Result<(), String> {
        for (key, val) in obj {
            let key_lower = key.to_lowercase();
            if patterns.iter().any(|p| key_lower.contains(p)) {
                return Err(format!(
                    "secret-like field '{}' found in serialized contract — \
                     REQ-PROV-904 violation",
                    key
                ));
            }
            // Recurse into nested objects and arrays
            if let Some(nested) = val.as_object() {
                check_object(nested, patterns)?;
            }
            if let Some(arr) = val.as_array() {
                for item in arr {
                    if let Some(nested) = item.as_object() {
                        check_object(nested, patterns)?;
                    }
                }
            }
        }
        Ok(())
    }

    if let Some(obj) = value.as_object() {
        check_object(obj, &secret_patterns)?;
    }
    if let Some(arr) = value.as_array() {
        for item in arr {
            if let Some(nested) = item.as_object() {
                check_object(nested, &secret_patterns)?;
            }
        }
    }

    Ok(())
}

// ─── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // REQ-PROV-901: Identity

    #[test]
    fn test_provider_id_normalizes() {
        let id = ProviderId::new("  OpenAI  ");
        assert_eq!(id.as_str(), "openai");
        assert!(id.is_valid());
    }

    #[test]
    fn test_provider_id_rejects_uppercase() {
        // ProviderId normalizes to lowercase — "OpenAI" becomes "openai" which is valid.
        // But spaces and non-kebab chars should fail.
        let id = ProviderId::new("Open AI!");
        assert!(!id.is_valid(), "non-kebab chars should fail validation");
    }

    #[test]
    fn test_provider_id_rejects_empty() {
        let id = ProviderId::new("");
        assert!(!id.is_valid());
    }

    #[test]
    fn test_model_id_preserves_case() {
        // Model IDs can have mixed case (e.g., "z-ai/glm-5.2")
        let id = ModelId::new("z-ai/glm-5.2");
        assert_eq!(id.as_str(), "z-ai/glm-5.2");
        assert!(id.is_valid());
    }

    #[test]
    fn test_model_id_rejects_empty() {
        let id = ModelId::new("");
        assert!(!id.is_valid());
    }

    #[test]
    fn test_provider_id_eq_hash() {
        let a = ProviderId::new("openai");
        let b = ProviderId::new("openai");
        let c = ProviderId::new("anthropic");
        let mut set = std::collections::HashSet::new();
        set.insert(a);
        assert!(set.contains(&b));
        assert!(!set.contains(&c));
    }

    // REQ-PROV-902: Capabilities

    #[test]
    fn test_capabilities_default_all_none() {
        let caps = ModelCapabilities::default();
        assert!(caps.context_window.is_none());
        assert!(caps.reasoning.is_none());
        assert!(caps.tools.is_none());
        assert!(caps.images.is_none());
    }

    #[test]
    fn test_capabilities_round_trip() {
        let caps = ModelCapabilities {
            context_window: Some(200_000),
            reasoning: Some(true),
            tools: Some(true),
            images: Some(true),
            structured_output: Some(true),
            cost_per_1m_input: Some(500),
            cost_per_1m_output: Some(1500),
            available: Some(true),
        };
        let json = serde_json::to_value(&caps).unwrap();
        let back: ModelCapabilities = serde_json::from_value(json).unwrap();
        assert_eq!(caps, back);
    }

    // REQ-PROV-903: Unknown field tolerance

    #[test]
    fn test_unknown_top_level_fields_preserved() {
        let json = serde_json::json!({
            "version": 1,
            "providers": [],
            "future_field": "value"
        });
        let catalog: ProviderCatalog = serde_json::from_value(json).unwrap();
        assert_eq!(catalog.version, 1);
        assert!(catalog.providers.is_empty());
        assert!(catalog.extra.contains_key("future_field"));
    }

    #[test]
    fn test_unknown_model_fields_dropped_silently() {
        // serde drops unknown fields inside structs without `deny_unknown_fields`
        let json = serde_json::json!({
            "id": "gpt-4o",
            "label": "GPT-4o",
            "capabilities": {},
            "future_field": "value"
        });
        let entry: ModelEntry = serde_json::from_value(json).unwrap();
        assert_eq!(entry.id.as_str(), "gpt-4o");
    }

    // REQ-PROV-904: No secrets

    #[test]
    fn test_provider_entry_has_no_secrets() {
        let entry = ProviderEntry {
            id: ProviderId::new("openai"),
            label: Some("OpenAI".into()),
            base_url: Some("https://api.openai.com/v1".into()),
            models: vec![],
        };
        let json = serde_json::to_value(&entry).unwrap();
        assert!(assert_no_secrets_in_json(&json).is_ok());
    }

    #[test]
    fn test_catalog_with_secret_field_detected() {
        let json = serde_json::json!({
            "version": 1,
            "providers": [],
            "api_key": "sk-1234"
        });
        // The catalog will accept it in `extra`, but our redaction check catches it
        let result = assert_no_secrets_in_json(&json);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("api_key"));
    }

    #[test]
    fn test_nested_secret_in_model_detected() {
        let json = serde_json::json!({
            "id": "openai",
            "models": [
                {
                    "id": "gpt-4o",
                    "capabilities": {},
                    "secret": "hidden"
                }
            ]
        });
        let result = assert_no_secrets_in_json(&json);
        assert!(result.is_err());
    }

    // REQ-PROV-905: Rust/TS equivalent semantics

    #[test]
    fn test_catalog_round_trip() {
        let catalog = ProviderCatalog {
            version: PROVIDER_CONTRACT_VERSION,
            providers: vec![ProviderEntry {
                id: ProviderId::new("openai"),
                label: Some("OpenAI".into()),
                base_url: Some("https://api.openai.com/v1".into()),
                models: vec![ModelEntry {
                    id: ModelId::new("gpt-4o"),
                    label: Some("GPT-4o".into()),
                    capabilities: ModelCapabilities {
                        context_window: Some(128_000),
                        reasoning: Some(false),
                        tools: Some(true),
                        images: Some(true),
                        structured_output: Some(true),
                        ..Default::default()
                    },
                }],
            }],
            extra: BTreeMap::new(),
        };
        let json = serde_json::to_value(&catalog).unwrap();
        let back: ProviderCatalog = serde_json::from_value(json).unwrap();
        assert_eq!(catalog, back);
    }

    #[test]
    fn test_empty_catalog() {
        let catalog = ProviderCatalog::empty();
        assert_eq!(catalog.version, PROVIDER_CONTRACT_VERSION);
        assert!(catalog.providers.is_empty());
    }

    #[test]
    fn test_find_provider_and_model() {
        let catalog = ProviderCatalog {
            version: 1,
            providers: vec![ProviderEntry {
                id: ProviderId::new("anthropic"),
                label: None,
                base_url: None,
                models: vec![ModelEntry {
                    id: ModelId::new("claude-sonnet-4"),
                    label: None,
                    capabilities: ModelCapabilities::default(),
                }],
            }],
            extra: BTreeMap::new(),
        };
        let provider_id = ProviderId::new("anthropic");
        let model_id = ModelId::new("claude-sonnet-4");
        assert!(catalog.find_provider(&provider_id).is_some());
        assert!(catalog.find_model(&provider_id, &model_id).is_some());
        assert!(catalog.find_model(&provider_id, &ModelId::new("nonexistent")).is_none());
        assert!(catalog.find_provider(&ProviderId::new("openai")).is_none());
    }

    #[test]
    fn test_duplicate_provider_ids_detected() {
        let catalog = ProviderCatalog {
            version: 1,
            providers: vec![
                ProviderEntry {
                    id: ProviderId::new("openai"),
                    label: None,
                    base_url: None,
                    models: vec![],
                },
                ProviderEntry {
                    id: ProviderId::new("openai"),
                    label: None,
                    base_url: None,
                    models: vec![],
                },
            ],
            extra: BTreeMap::new(),
        };
        assert!(catalog.has_duplicate_providers());
    }

    #[test]
    fn test_no_duplicate_providers() {
        let catalog = ProviderCatalog {
            version: 1,
            providers: vec![
                ProviderEntry {
                    id: ProviderId::new("openai"),
                    label: None,
                    base_url: None,
                    models: vec![],
                },
                ProviderEntry {
                    id: ProviderId::new("anthropic"),
                    label: None,
                    base_url: None,
                    models: vec![],
                },
            ],
            extra: BTreeMap::new(),
        };
        assert!(!catalog.has_duplicate_providers());
    }
}
