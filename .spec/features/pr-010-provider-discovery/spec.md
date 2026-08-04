# PR-010 — Read-only Provider Discovery

**Status:** implemented
**Depende de:** PR-009

## Requisitos
- [x] REQ-DISC-1001: Cancelable read-only discovery bridge.
- [x] REQ-DISC-1002: Contract-normalized catalog output.
- [x] REQ-DISC-1003: Typed empty/partial/error states.
- [x] REQ-DISC-1004: Credential-safe diagnostics (no secrets in errors/logs).
- [x] REQ-DISC-1005: Hermes config is never written.

## Acceptance Criteria
- @spec:AC-DISC-101: discover_providers returns Ok with normalized catalog
- @spec:AC-DISC-102: ProviderId from env vars is normalized to lowercase
- @spec:AC-DISC-103: DiscoveryError messages are credential-safe
- @spec:AC-DISC-104: DiscoveryResult catalog has no secret fields
- @spec:AC-DISC-105: Hermes config file is unchanged after read
- @spec:AC-DISC-106: Missing config file returns typed error
- @spec:AC-DISC-107: Config without provider field returns typed error
- @spec:AC-DISC-108: DiscoveryResult tracks source per provider
