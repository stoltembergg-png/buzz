use std::path::{Path, PathBuf};

use super::types::RuntimeFileConfig;

/// Read only Hermes' non-secret model configuration from `config.yaml`.
/// Credentials live in Hermes' secret stores and are deliberately not read here.
pub(super) fn read_config_file() -> Option<RuntimeFileConfig> {
    let path = hermes_config_path()?;
    let raw = std::fs::read_to_string(path).ok()?;
    parse_hermes_config(&raw)
}

pub(crate) fn hermes_config_path() -> Option<PathBuf> {
    let hermes_home = std::env::var("HERMES_HOME").ok();
    let home_dir = dirs::home_dir();
    config_path_for(hermes_home.as_deref(), home_dir.as_deref())
}

fn config_path_for(hermes_home: Option<&str>, home_dir: Option<&Path>) -> Option<PathBuf> {
    if let Some(home) = hermes_home.map(str::trim).filter(|home| !home.is_empty()) {
        return Some(PathBuf::from(home).join("config.yaml"));
    }
    home_dir.map(|home| home.join(".hermes").join("config.yaml"))
}

fn parse_hermes_config(yaml_str: &str) -> Option<RuntimeFileConfig> {
    let root: serde_yaml::Value = serde_yaml::from_str(yaml_str).ok()?;
    let model_value = root.get("model");

    let (provider, model) = match model_value {
        Some(serde_yaml::Value::Mapping(model_map)) => (
            mapping_string(model_map, "provider"),
            mapping_string(model_map, "default")
                .or_else(|| mapping_string(model_map, "model")),
        ),
        Some(value) => (None, yaml_string(value)),
        None => (None, None),
    };

    // Do not walk arbitrary YAML into `extra`: Hermes config can contain
    // credentials, OAuth metadata, or provider-specific secret material.
    Some(RuntimeFileConfig {
        model,
        provider,
        ..RuntimeFileConfig::default()
    })
}

fn yaml_string(value: &serde_yaml::Value) -> Option<String> {
    value
        .as_str()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn mapping_string(map: &serde_yaml::Mapping, key: &str) -> Option<String> {
    map.get(serde_yaml::Value::String(key.to_owned()))
        .and_then(yaml_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_nested_model_provider_and_default() {
        let yaml = r#"
model:
  provider: openrouter
  default: anthropic/claude-sonnet-4.6
"#;
        let cfg = parse_hermes_config(yaml).expect("valid Hermes YAML");
        assert_eq!(cfg.provider.as_deref(), Some("openrouter"));
        assert_eq!(cfg.model.as_deref(), Some("anthropic/claude-sonnet-4.6"));
        assert!(cfg.extra.is_empty());
    }

    #[test]
    fn parses_scalar_model_without_inventing_provider() {
        let yaml = "model: anthropic/claude-sonnet-4.6\n";
        let cfg = parse_hermes_config(yaml).expect("valid Hermes YAML");
        assert_eq!(cfg.provider, None);
        assert_eq!(cfg.model.as_deref(), Some("anthropic/claude-sonnet-4.6"));
    }

    #[test]
    fn accepts_legacy_model_key_when_default_is_absent() {
        let yaml = r#"
model:
  provider: nous
  model: claude-sonnet-4.6
"#;
        let cfg = parse_hermes_config(yaml).expect("valid Hermes YAML");
        assert_eq!(cfg.provider.as_deref(), Some("nous"));
        assert_eq!(cfg.model.as_deref(), Some("claude-sonnet-4.6"));
    }

    #[test]
    fn invalid_yaml_returns_none() {
        assert!(parse_hermes_config("model: [not: valid").is_none());
    }

    #[test]
    fn credential_fields_are_not_surfaceable() {
        let yaml = r#"
model:
  provider: openrouter
  default: anthropic/claude-sonnet-4.6
api_key: should-not-leak
access_token: should-not-leak
oauth:
  refresh_token: should-not-leak
"#;
        let cfg = parse_hermes_config(yaml).expect("valid Hermes YAML");
        assert!(cfg.extra.is_empty());
        let serialized = format!("{cfg:?}");
        assert!(!serialized.contains("should-not-leak"));
    }

    #[test]
    fn config_path_prefers_hermes_home() {
        assert_eq!(
            config_path_for(Some("/tmp/hermes-agent"), Some(Path::new("/Users/test"))),
            Some(PathBuf::from("/tmp/hermes-agent/config.yaml"))
        );
    }

    #[test]
    fn config_path_uses_default_home_when_hermes_home_is_absent() {
        assert_eq!(
            config_path_for(None, Some(Path::new("/Users/test"))),
            Some(PathBuf::from("/Users/test/.hermes/config.yaml"))
        );
    }
}
