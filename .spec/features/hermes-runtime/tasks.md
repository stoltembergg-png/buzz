# Tasks: Hermes runtime

> feature: hermes-runtime

## T-001 — Runtime registry [concluida]
- Refs: US-001, AC-001, AC-002, AC-006
- Arquivos: desktop/src-tauri/src/managed_agents/discovery.rs, desktop/src-tauri/src/managed_agents/discovery/hermes.rs, desktop/src-tauri/src/managed_agents/discovery/tests.rs, desktop/src-tauri/src/managed_agents/discovery/hermes_tests.rs, crates/buzz-acp/src/config.rs
- Notas: Metadata compilada, aliases, fallback ACP e teste de não duplicação.

## T-002 — Safe config bridge [concluida]
- Refs: US-002, US-003, AC-003, AC-004, AC-005
- Arquivos: desktop/src-tauri/src/managed_agents/config_bridge/hermes.rs, desktop/src-tauri/src/managed_agents/config_bridge/mod.rs, desktop/src-tauri/src/managed_agents/config_bridge/reader.rs
- Notas: Ler somente provider/model de config.yaml; nunca ler .env ou stores OAuth.

## T-003 — Catalog migration [concluida]
- Refs: US-001, AC-006
- Arquivos: desktop/src-tauri/src/managed_agents/discovery/presets.rs, desktop/src/features/onboarding/ui/RuntimeIcon.tsx
- Notas: Hermes sai da segunda camada de presets e o mapa de logos não mantém entrada órfã.

## T-004 — Verification [concluida]
- Refs: AC-001, AC-002, AC-003, AC-004, AC-005, AC-006
- Arquivos: .spec/features/hermes-runtime/contract.test.mjs, onpspec.config.json
- Notas: Evidência TAP anotada por AC, suíte Rust/Desktop e auditoria ONP.
