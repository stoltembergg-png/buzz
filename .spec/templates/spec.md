<!--
  Feature Spec Template — aligned with PR-001 (Definition of Done) and PR-002 (templates contract).

  Save as: `.spec/features/<feature-name>.md` (or <PR-XXX>.md if several features ship together).

  Conventions:
  - REQ-XXX-NNN = requirement identifier (stable, never reused)
  - AC-XXX-NNN  = acceptance criterion (must point to a test or manual evidence)
  - T-XXX-NNN   = task identifier (cell of work)
  - E-XXX-NNN   = evidence identifier (recorded SHA + commands + exit codes)

  Every checkbox must be either checked or removed before "Done".
  Placeholders like TODO / TBD / ??? are not allowed in checked items.
-->

# PR-XXX — <one-line title>

**Status:** planned | in-progress | done
**Phase:** <governance | memory | providers | context | skills | mcp | plugins | observability | eval | self-improvement | security | data | release | maintenance>
**Depends on:** <PR-XXX [, PR-XXX, ...] | none>

## Objective

<!-- One-paragraph description of the capability produced for the user. -->

## Scope

- Bullet 1 (concrete deliverable).
- Bullet 2.
- Bullet 3.

## Non-objectives

- Explicitly out of scope.
- Cross-cutting feature that ships in another PR.

## Requirements

- [ ] `REQ-XXX-NNN` <requirement statement — verifiable, no "should", no "good">
- [ ] `REQ-XXX-NNN` <requirement>
- [ ] `REQ-XXX-NNN` <requirement>

## Acceptance criteria

- [ ] `AC-XXX-NNN` <criterion pointing to a test id or manual evidence file>
- [ ] `AC-XXX-NNN` <criterion>
- [ ] `AC-XXX-NNN` <criterion>

## Tests

- [ ] Positive: `T-XXX-NNN` <name> — automated test ID or manual evidence file
- [ ] Negative: `T-XXX-NNN` <name>
- [ ] Regression: `T-XXX-NNN` <name>
- [ ] Integration: `T-XXX-NNN` <name> (only if cross-component)

## Risks and security

- Trust boundaries touched (filesystem, network, user input, secrets).
- Risk: <specific risk + mitigation>.
- Threat: <specific threat + mitigation>.

## Compatibility and migration

- Backward compatibility: <yes / partial / no + reason>.
- Migration path: <script / manual / none>.
- Rollback: <git revert / feature flag / data restore>.

## Evidence

- [ ] `E-XXX-NNN` Recorded command, output, exit code, and final SHA — `.spec/verification/<feature>.json`.

## Definition of Done

- [ ] Scope and non-objectives are explicit.
- [ ] Requirements and acceptance criteria have stable IDs.
- [ ] Positive, negative, regression, and integration tests exist when applicable.
- [ ] Commands and outputs recorded against the final SHA.
- [ ] No test was ignored, weakened, or removed without justification.
- [ ] User-controlled inputs, secrets, and trust boundaries were reviewed.
- [ ] Compatibility, migration, and rollback are documented.
- [ ] Final diff was reviewed after the last change.
- [ ] PR is independently reversible.
- [ ] Documentation matches actual behavior.
