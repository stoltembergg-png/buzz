# PR-002 — Templates de PR e feature spec

**Status:** planejada
**Fase:** Governança
**Depende de:** PR-001

## Objetivo

Padronizar toda entrega com templates obrigatórios de PR, especificação, tarefas e evidências.

## Escopo

- Criar `.github/PULL_REQUEST_TEMPLATE.md`.
- Criar templates em `.spec/templates/` para `spec.md`, `tasks.md`, `contract.test.mjs` e evidência JSON.
- Definir identificadores `REQ-*`, `AC-*`, `T-*` e `E-*`.
- Exigir não objetivos, riscos, segurança, rollback e compatibilidade.

## Não objetivos

- Implementar funcionalidades Hermes.
- Executar testes de produto.
- Criar CI hospedada.

## Requisitos

- [ ] `REQ-GOV-201` O template de PR contém escopo, requisitos, critérios, testes, segurança, compatibilidade, rollback e evidências.
- [ ] `REQ-GOV-202` Todo critério de aceitação pode apontar para um teste ou verificação manual.
- [ ] `REQ-GOV-203` O template rejeita conclusão com placeholders ou checkboxes pendentes.
- [ ] `REQ-GOV-204` A evidência registra SHA, plataforma, ferramentas, comandos e exit codes.

## Critérios de aceitação

- [ ] `AC-GOV-201` Um exemplo preenchido não contém `TODO`, `TBD` ou requisito sem prova.
- [ ] `AC-GOV-202` O contrato identifica requisito sem teste e teste órfão.
- [ ] `AC-GOV-203` O template diferencia teste automatizado de validação manual.

## Testes

- [ ] Teste de presença de todas as seções obrigatórias.
- [ ] Teste negativo removendo uma seção.
- [ ] Teste negativo inserindo placeholder.
- [ ] Teste de unicidade dos IDs no exemplo.

## Definition of Done

- [ ] Templates adicionados.
- [ ] Exemplos completos adicionados.
- [ ] Testes de contrato passam localmente.
- [ ] Comandos e SHA final registrados.
- [ ] Documentação revisada contra PR-001.
