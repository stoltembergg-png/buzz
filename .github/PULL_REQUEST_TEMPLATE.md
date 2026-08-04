<!--
  PR template — aligned with PR-001 (Definition of Done) and PR-002 (governance templates).

  Required sections (per REQ-GOV-201):
    Summary, Scope, Non-objectives, Requirements, Acceptance criteria,
    Tests, Security, Compatibility and migration, Rollback, Evidence.

  Placeholders (TODO / TBD / ??? / XXX) are not allowed in checked items.
  Every Acceptance Criterion must cite a test or evidence file.
-->

## Summary

<!-- One paragraph: what this PR does and why. -->

### Related issue

<!-- Link the issue, or write "none found" after a duplicate search. -->

### Roadmap PR

<!-- Reference the roadmap PR number this work implements, e.g. PR-XXX. -->

## Scope

- Bullet 1 (concrete deliverable).
- Bullet 2.

## Non-objectives

- Explicitly out of scope.

## Requirements

- [ ] `REQ-XXX-NNN` <requirement>
- [ ] `REQ-XXX-NNN` <requirement>

## Acceptance criteria

- [ ] `AC-XXX-NNN` <criterion → test id or evidence file>
- [ ] `AC-XXX-NNN` <criterion>

## Tests

- [ ] Positive: <name + test id or evidence file>
- [ ] Negative: <name + test id or evidence file>
- [ ] Regression: <name + test id or evidence file>
- [ ] Integration: <name + test id or evidence file> (only if cross-component)

## Security

- Trust boundaries touched (filesystem, network, user input, secrets).
- Risks and mitigations.
- Threats and mitigations.

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
