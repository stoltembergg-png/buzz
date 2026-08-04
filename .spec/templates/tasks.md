<!--
  Tasks Template — granular breakdown of work for a PR or feature spec.

  Save as: `.spec/features/<feature>/tasks.md` or `.spec/tasks/<PR-XXX>.md`.

  Each T-NNN is a single unit of work that one contributor can complete in
  a single commit batch. Evidence links to either an automated test or a
  manual verification file under `.spec/verification/`.

  Placeholders are not allowed in checked items.
-->

# Tasks — PR-XXX

| ID | Task | Owner | Status | Evidence | Depends on |
|----|------|-------|--------|----------|------------|
| T-XXX-001 | <description> | <name> | todo \| doing \| done | `E-XXX-001` (test or evidence) | — |
| T-XXX-002 | <description> | <name> | todo \| doing \| done | `E-XXX-002` | T-XXX-001 |
| T-XXX-003 | <description> | <name> | todo \| doing \| done | `E-XXX-003` | T-XXX-001, T-XXX-002 |

## Definition of Done recap

- [ ] All T-NNN marked done.
- [ ] Each done task has an Evidence entry referencing a passing test or verification file.
- [ ] No T-NNN was silently added or removed after the PR opened.
- [ ] Final SHA recorded in the spec's Evidence section.

## Out-of-scope tasks

(Tracking only — these ship in other PRs.)

- [ ] T-XXX-OOS-001 <description> — handled by PR-YYY.
