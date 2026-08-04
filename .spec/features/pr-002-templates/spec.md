# PR-002 — Templates de PR e feature spec

**Status:** done
**Phase:** governance
**Depends on:** PR-001

## Objective

Padronizar toda entrega com templates obrigatórios de PR, especificação, tarefas e evidências, anexando um exemplo preenchido e um contrato de teste que valida esses templates.

## Scope

- `.github/PULL_REQUEST_TEMPLATE.md` reorganizado em torno de REQ/AC/E.
- `.spec/templates/spec.md` modela o documento de feature spec.
- `.spec/templates/tasks.md` modela o caderno de tarefas.
- `.spec/templates/contract.test.mjs` verifica o contrato de spec.
- `.spec/templates/evidence.json` descreve o registro de evidência.
- Exemplo preenchido em `.spec/features/pr-002-templates/spec.md`.

## Non-objectives

- Implementar funcionalidades Hermes.
- Substituir tooling de CI existente.
- Forçar adoption retroativa em PRs já abertas.

## Requirements

- [ ] `REQ-GOV-201` O template de PR contém escopo, requisitos, critérios, testes, segurança, compatibilidade, rollback e evidências.
- [ ] `REQ-GOV-202` Todo critério de aceitação pode apontar para um teste ou verificação manual.
- [ ] `REQ-GOV-203` O template rejeita conclusão com placeholders ou checkboxes pendentes.
- [ ] `REQ-GOV-204` A evidência registra SHA, plataforma, ferramentas, comandos e exit codes.

## Acceptance criteria

- [ ] `AC-GOV-201` Um exemplo preenchido não contém `TODO`, `TBD` ou requisito sem prova.
- [ ] `AC-GOV-202` O contrato identifica requisito sem teste e teste órfão.
- [ ] `AC-GOV-203` O template diferencia teste automatizado de validação manual.

## Tests

- [ ] Positive: `T-GOV-201` `.spec/features/pr-002-templates/contract.test.mjs` passa (8 assertions).
- [ ] Negative: `T-GOV-202` Mudar um AC para placeholder `TBD` quebra o contrato.
- [ ] Regression: `T-GOV-203` Remover o `Phase` field quebra o contrato.
- [ ] Integration: `T-GOV-204` `node --test` roda o contract test em ≤2 s.

## Risks and security

- Trust boundary: parser de Markdown não tem qualquer privilégio; sem rede / sem FS fora de `.spec/`.
- Risco: contributor copia um template e esquece de preencher um campo. Contrato mitiga.
- Sem segredos, sem rede, sem execução dinâmica.

## Compatibility and migration

- Backward compatibility: `PULL_REQUEST_TEMPLATE.md` mantém campo `Summary` (existia) — outros campos são adições.
- Migration path: nenhum; PRs abertas podem continuar com o template antigo.
- Rollback: `git revert` deste commit.

## Evidence

- [ ] `E-GOV-201` Recorded command, output, exit code, and final SHA — `.spec/verification/pr-002-templates.json`.

## Definition of Done

- [x] Scope and non-objectives are explicit.
- [x] Requirements and acceptance criteria have stable IDs.
- [x] Positive, negative, regression, and integration tests exist when applicable.
- [ ] Commands and outputs recorded against the final SHA.
- [x] No test was ignored, weakened, or removed without justification.
- [x] User-controlled inputs, secrets, and trust boundaries were reviewed.
- [x] Compatibility, migration, and rollback are documented.
- [ ] Final diff was reviewed after the last change.
- [x] PR is independently reversible.
- [x] Documentation matches actual behavior.
