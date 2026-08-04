# PR-003 — Verificador local de feature

**Status:** done
**Phase:** governance
**Depends on:** PR-001, PR-002

## Objective

Criar um comando local que valide rastreabilidade entre requisitos, critérios de aceitação, testes, tarefas e evidências sem depender de GitHub Actions.

## Scope

- `scripts/verify-feature.sh <feature>` — valida spec ↔ contract tests.
- Leitura de `.spec/features/<feature>/spec.md` e `.spec/features/<feature>/contract.test.mjs`.
- Códigos de saída determinísticos (0–8).
- Relatório legível e opcionalmente JSON (`--json`).
- Nenhuma execução de comandos arbitrários vindos da spec (REQ-GOV-306).

## Non-objectives

- Implementar funcionalidades Hermes.
- Executar testes de produto.
- Criar CI hospedada.
- Substituir tooling de CI existente.

## Requirements

- [ ] `REQ-GOV-301` Falhar quando a feature não existe.
- [ ] `REQ-GOV-302` Falhar quando requisito não possui critério/teste.
- [ ] `REQ-GOV-303` Falhar para teste ou evidência órfã.
- [ ] `REQ-GOV-304` Falhar para tarefa concluída sem evidência.
- [ ] `REQ-GOV-305` Registrar SHA, plataforma e versões no relatório.
- [ ] `REQ-GOV-306` Não interpretar conteúdo da spec como shell.

## Acceptance criteria

- [ ] `AC-GOV-301` Feature inexistente → exit 1.
- [ ] `AC-GOV-302` Requisito sem prova → exit 4.
- [ ] `AC-GOV-303` Teste sem requisito → exit 5.
- [ ] `AC-GOV-304` Tarefa sem evidência → exit 6.
- [ ] `AC-GOV-305` Caminho malicioso → exit 8.
- [ ] `AC-GOV-306` Feature válida → exit 0, relatório JSON inclui SHA/plataforma.
- [ ] `AC-GOV-307` Testes negativos cobrem: feature inexistente, arquivo ausente, ID duplicado, requisito sem prova, teste sem requisito, SHA divergente, caminho malicioso, feature válida.

## Tests

- [ ] Positive: `T-GOV-301` `bash scripts/verify-feature.sh pr-003-feature-verifier --json` → exit 0, JSON válido.
- [ ] Negative: `T-GOV-302` feature inexistente → exit 1.
- [ ] Negative: `T-GOV-303` spec.md ausente → exit 2.
- [ ] Negative: `T-GOV-304` contract.test.mjs ausente → exit 2.
- [ ] Negative: `T-GOV-305` AC duplicado no spec → exit 3.
- [ ] Negative: `T-GOV-306` AC duplicado no test → exit 3.
- [ ] Negative: `T-GOV-307` AC no spec sem test → exit 4.
- [ ] Negative: `T-GOV-308` @spec:AC no test sem spec → exit 5.
- [ ] Negative: `T-GOV-309` tarefa done sem evidence → exit 6.
- [ ] Negative: `T-GOV-310` contract test falha → exit 7.
- [ ] Negative: `T-GOV-311` path traversal / unsafe chars → exit 8.
- [ ] Integration: `T-GOV-312` roda contra pr-002-templates e hermes-runtime → ambos exit 0.
- [ ] Regression: `T-GOV-313` feature válida roda em ≤2 s.

## Risks and security

- Trust boundary: parser de Markdown e grep não têm privilégios; sem rede / sem FS fora de `.spec/`.
- REQ-GOV-306 garantido: feature name validado contra `^[a-zA-Z0-9._-]+$`, sem traversal.
- Nenhum segredo, nenhuma rede, nenhuma execução dinâmica de conteúdo da spec.

## Compatibility and migration

- Backward compatibility: features existentes (pr-002-templates, hermes-runtime) continuam passando.
- Migration path: nenhum; script novo, features antigas não quebram.
- Rollback: `git revert` deste commit.

## Evidence

- [ ] `E-GOV-301` Recorded command, output, exit code, and final SHA — `.spec/verification/pr-003-feature-verifier.json`.

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