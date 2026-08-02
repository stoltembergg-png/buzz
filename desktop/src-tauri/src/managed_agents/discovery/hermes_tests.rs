use super::super::{known_acp_runtime, preset_harness_ids};

#[test]
fn hermes_is_a_known_runtime_for_all_supported_entrypoints() {
    let by_acp = known_acp_runtime("hermes-acp").expect("hermes-acp should resolve");
    let by_cli = known_acp_runtime("hermes").expect("hermes should resolve");
    let by_alias = known_acp_runtime("hermes-agent").expect("hermes-agent alias should resolve");

    assert_eq!(by_acp.id, "hermes");
    assert_eq!(by_cli.id, "hermes");
    assert_eq!(by_alias.id, "hermes");
    assert!(by_acp.supports_acp_model_switching);
    assert_eq!(by_acp.config_file_path, Some("~/.hermes/config.yaml"));
    assert_eq!(by_acp.config_file_format, Some("yaml"));
    assert!(by_acp.model_env_var.is_none());
    assert!(by_acp.provider_env_var.is_none());
    assert!(by_acp
        .default_env
        .iter()
        .any(|(key, value)| *key == "HERMES_ACP_SKIP_CONFIGURED_MCP" && *value == "1"));
}

#[test]
fn hermes_is_not_registered_as_a_second_layer_preset() {
    assert!(!preset_harness_ids().contains(&"hermes"));
}

#[test]
fn hermes_cli_fallback_gets_acp_subcommand() {
    assert_eq!(
        normalize_agent_args("hermes", Vec::new()),
        vec!["acp".to_string()]
    );
}
