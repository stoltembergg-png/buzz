//! Provider capability contract (PR-009)
//!
//! Versioned internal types for providers, models, and capabilities. No UI,
//! no discovery, no selection — purely the data contract that downstream
//! PRs (PR-010 discovery, PR-011 selection) will consume.
//!
//! Design rules (from spec):
//! - Provider/model identity is stable (id), NOT tied to display label.
//! - Capabilities cover context window, reasoning, tools, images, structured
//!   output, cost, and availability.
//! - Unknown fields are tolerated (serde `#[serde(default)]` + `#[serde(deny)]`
//!   off), per documented rule (REQ-PROV-903).
//! - No secrets make part of the contract (REQ-PROV-904).
//! - Rust and TypeScript share equivalent semantics (REQ-PROV-905).
//!
//! Spec: docs/roadmap/prs/PR-009-provider-capability-contract.md

use serde::{Deserialize, Serialize};

/// Schema version tag, embedded in serialized payloads.
pub const PROVIDER_CONTRACT_VERSION: u32 = 1;

/// A provider identity (stable — label may change without invalidating id).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderId {
    /// Stable identifier (e.g. "openai", "anthropic", "hermes").
    pub id: String,
    /// Human-readable label (display only — never identity).
    #[serde(default)]
    pub label: String,
}

/// A model identity within a provider.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelId {
    /// Stable model identifier (e.g. "gpt-4o", "claude-3-opus").
    pub id: String,
    /// Parent provider id.
    pub provider_id: String,
    /// Human-readable label (display only — never identity).
    #[serde(default)]
    pub label: String,
}

/// Cost tier (relative — not exact dollar amounts per REQ-PROV-904).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CostTier {
    Free,
    Low,
    Medium,
    High,
    Unknown,
}

/// Availability state of a model at discovery time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelAvailability {
    Available,
    Degraded,
    Unavailable,
    Unknown,
}

/// Model capability set (REQ-PROV-902).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelCapabilities {
    /// Maximum context window in tokens.
    #[serde(default)]
    pub max_context_tokens: Option<u32>,
    /// Whether the model supports reasoning/chain-of-thought.
    #[serde(default)]
    pub supports_reasoning: bool,
    /// Whether tool-use / function-calling is supported.
    #[serde(default)]
    pub supports_tools: bool,
    /// Whether image inputs are supported.
    #[serde(default)]
    pub supports_images: bool,
    /// Whether structured / JSON-mode output is supported.
    #[serde(default)]
    pub supports_structured_output: bool,
    /// Relative cost tier (never exact amounts — REQ-PROV-904).
    #[serde(default)]
    pub cost_tier: CostTier,
    /// Availability at discovery time.
    #[serde(default)]
    pub availability: ModelAvailability,
}

impl Default for ModelCapabilities {
    fn default() -> Self {
        Self {
            max_context_tokens: None,
            supports_reasoning: false,
            supports_tools: false,
            supports_images: false,
            supports_structured_output: false,
            cost_tier: CostTier::Unknown,
            availability: ModelAvailability::Unknown,
        }
    }
}

/// A complete model entry: identity + capabilities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelEntry {
    #[serde(flatten)]
    pub id: ModelId,
    #[serde(default)]
    pub capabilities: ModelCapabilities,
}

/// A provider entry: identity + its models.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEntry {
    #[serde(flatten)]
    pub id: ProviderId,
    #[serde(default)]
    pub models: Vec<ModelEntry>,
}

/// The contract payload — a versioned catalog of providers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderCatalog {
    pub schema: u32,
    pub providers: Vec<ProviderEntry>,
}

impl ProviderCatalog {
    /// Create an empty catalog with the current schema version.
    pub fn new() -> Self {
        Self {
            schema: PROVIDER_CONTRACT_VERSION,
            providers: Vec::new(),
        }
    }
}

impl Default for ProviderCatalog {
    fn default() -> Self {
        Self::new()
    }
}

/// Verify that a serialized catalog contains no secret-like fields.
///
/// Returns `Ok(())` if the JSON string is clean, or `Err(message)` listing
/// the offending field names. This is a defensive guard used by tests and
/// by PR-010 before storing a catalog (REQ-PROV-904).
pub fn assert_no_secrets_in_json(json: &str) -> Result<(), String> {
    const FORBIDDEN_KEYS: &[&str] = &[
        "api_key", "apikey", "secret", "token", "password", "credential",
        "private_key", "auth_header", "bearer",
    ];
    let lower = json.to_lowercase();
    for key in FORBIDDEN_KEYS {
        if lower.contains(&format!("\"{}", key")) {
            return Err(format!("secret-like field detected in catalog JSON: '{}'", key));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- REQ-PROV-901: stable identity separate from label ---

    #[test]
    fn test_provider_id_stability_across_label_change() {
        let a = ProviderId { id: "openai".into(), label: "OpenAI".into() };
        let b = ProviderId { id: "openai".into(), label: "OpenAI Inc".into() };
        assert_eq!(a.id, b.id, "provider id is stable despite label change");
    }

    #[test]
    fn test_model_id_stability_across_label_change() {
        let a = ModelId { id: "gpt-4o".into(), provider_id: "openai".into(), label: "GPT-4o".into() };
        let b = ModelId { id: "gpt-4o".into(), provider_id: "openai".into(), label: "GPT-4o (2024)".into() };
        assert_eq!(a.id, b.id, "model id is stable despite label change");
    }

    // --- REQ-PROV-902: capabilities cover all required dimensions ---

    #[test]
    fn test_model_capabilities_full_roundtrip() {
        let caps = ModelCapabilities {
            max_context_tokens: Some(128000),
            supports_reasoning: true,
            supports_tools: true,
            supports_images: true,
            supports_structured_output: true,
            cost_tier: CostTier::Medium,
            availability: ModelAvailability::Available,
        };
        let json = serde_json::to_string(&caps).unwrap();
        let back: ModelCapabilities = serde_json::from_str(&json).unwrap();
        assert_eq!(caps, back);
    }

    #[test]
    fn test_capabilities_have_all_required_fields() {
        let caps = ModelCapabilities::default();
        // All fields present (defaults)
        assert!(caps.max_context_tokens.is_none());
        assert!(!caps.supports_reasoning);
        assert!(!caps.supports_tools);
        assert!(!caps.supports_images);
        assert!(!caps.supports_structured_output);
        assert_eq!(caps.cost_tier, CostTier::Unknown);
        assert_eq!(caps.availability, ModelAvailability::Unknown);
    }

    // --- REQ-PROV-903: unknown fields tolerated ---

    #[test]
    fn test_unknown_fields_tolerated() {
        let json = r#"{
            "schema": 1,
            "providers": [{
                "id": "openai",
                "label": "OpenAI",
                "future_field": "ignored",
                "another_unknown": 42
            }]
        }"#;
        let cat: ProviderCatalog = serde_json::from_str(json).unwrap();
        assert_eq!(cat.providers.len(), 1);
        assert_eq!(cat.providers[0].id.id, "openai");
    }

    #[test]
    fn test_missing_optional_fields_default() {
        let json = r#"{
            "schema": 1,
            "providers": [{
                "id": "anthropic",
                "models": [{
                    "id": "claude-3-opus",
                    "provider_id": "anthropic"
                }]
            }]
        }"#;
        let cat: ProviderCatalog = serde_json::from_str(json).unwrap();
        let m = &cat.providers[0].models[0];
        assert_eq!(m.id.label, "", "missing label defaults to empty");
        assert!(!m.capabilities.supports_tools, "missing capability defaults to false");
    }

    // --- REQ-PROV-904: no secrets in contract ---

    #[test]
    fn test_catalog_serialization_has_no_secrets() {
        let cat = ProviderCatalog {
            schema: 1,
            providers: vec![ProviderEntry {
                id: ProviderId { id: "openai".into(), label: "OpenAI".into() },
                models: vec![ModelEntry {
                    id: ModelId { id: "gpt-4o".into(), provider_id: "openai".into(), label: "GPT-4o".into() },
                    capabilities: ModelCapabilities {
                        max_context_tokens: Some(128000),
                        supports_tools: true,
                        ..Default::default()
                    },
                }],
            }],
        };
        let json = serde_json::to_string(&cat).unwrap();
        assert_no_secrets_in_json(&json).unwrap();
    }

    #[test]
    fn test_assert_no_secrets_detects_offending_fields() {
        let json = r#"{"api_key": "sk-xxx"}"#;
        let result = assert_no_secrets_in_json(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("api_key"));
    }

    #[test]
    fn test_assert_no_secrets_passes_clean_json() {
        let json = r#"{"id": "openai", "label": "OpenAI"}"#;
        assert_no_secrets_in_json(json).unwrap();
    }

    // --- Round-trip serialization ---

    #[test]
    fn test_catalog_full_roundtrip() {
        let cat = ProviderCatalog {
            schema: 1,
            providers: vec![
                ProviderEntry {
                    id: ProviderId { id: "openai".into(), label: "OpenAI".into() },
                    models: vec![ModelEntry {
                        id: ModelId { id: "gpt-4o".into(), provider_id: "openai".into(), label: "GPT-4o".into() },
                        capabilities: ModelCapabilities {
                            max_context_tokens: Some(128000),
                            supports_tools: true,
                            supports_images: true,
                            supports_structured_output: true,
                            cost_tier: CostTier::Medium,
                            availability: ModelAvailability::Available,
                            ..Default::default()
                        },
                    }],
                },
                ProviderEntry {
                    id: ProviderId { id: "anthropic".into(), label: "Anthropic".into() },
                    models: vec![ModelEntry {
                        id: ModelId { id: "claude-3-opus".into(), provider_id: "anthropic".into(), label: "Claude 3 Opus".into() },
                        capabilities: ModelCapabilities {
                            max_context_tokens: Some(200000),
                            supports_reasoning: true,
                            supports_tools: true,
                            cost_tier: CostTier::High,
                            availability: ModelAvailability::Available,
                            ..Default::default()
                        },
                    }],
                },
            ],
        };
        let json = serde_json::to_string(&cat).unwrap();
        let back: ProviderCatalog = serde_json::from_str(&json).unwrap();
        assert_eq!(cat, back, "round-trip must preserve all fields");
    }

    #[test]
    fn test_empty_catalog_roundtrip() {
        let cat = ProviderCatalog::new();
        let json = serde_json::to_string(&cat).unwrap();
        let back: ProviderCatalog = serde_json::from_str(&json).unwrap();
        assert_eq!(cat, back);
    }

    // --- Duplicate IDs and mutable labels ---

    #[test]
    fn test_duplicate_provider_ids_remain_equal() {
        let a = ProviderId { id: "openai".into(), label: "OpenAI".into() };
        let b = ProviderId { id: "openai".into(), label: "Different".into() };
        // IDs match, labels differ — identity is the id, not the label.
        assert_eq!(a.id, b.id);
        assert_ne!(a.label, b.label);
    }

    // --- Schema version is documented and stable ---

    #[test]
    fn test_schema_version_embedded_in_catalog() {
        let cat = ProviderCatalog::new();
        assert_eq!(cat.schema, PROVIDER_CONTRACT_VERSION);
        let json = serde_json::to_string(&cat).unwrap();
        assert!(json.contains("\"schema\":1"));
    }

    // --- CostTier and ModelAvailability serialize as kebab-case ---

    #[test]
    fn test_cost_tier_serializes_kebab_case() {
        let json = serde_json::to_string(&CostTier::Medium).unwrap();
        assert_eq!(json, "\"medium\"");
        let json = serde_json::to_string(&ModelAvailability::Available).unwrap();
        assert_eq!(json, "\"available\"");
    }
}
