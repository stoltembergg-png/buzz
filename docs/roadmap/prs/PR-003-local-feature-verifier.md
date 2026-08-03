# PR-003 — Verificador local de feature

**Status:** planejada
**Fase:** Governança
**Depende de:** PR-001, PR-002

## Objetivo

Criar um comando local que valide rastreabilidade entre requisitos, critérios de aceitação, testes, tarefas e evidências sem depender de GitHub Actions.

## Escopo

- `scripts/verify-feature.sh <feature>`.
- Leitura de `.spec/features/<feature>/` e `.spec/verification/<feature>.json`.
- Códigos de saída determinísticos.
- Relatório legível e opcionalmente JSON.
- Nenhuma execução de comandos arbitrários vindos da spec.

## Requisitos

- [ ] `REQ-GOV-301` Falhar quando a feature não existe.
- [ ] `REQ-GOV-302` Falhar quando requisito não possui critério/teste.
- [ ] `REQ-GOV-303` Falhar para teste ou evidência órfã.
- [ ] `REQ-GOV-304` Falhar para tarefa concluída sem evidência.
- [ ] `REQ-GOV-305` Registrar SHA, plataforma e versões no relatório.
- [ ] `REQ-GOV-306` Não interpretar conteúdo da spec como shell.

## Testes obrigatórios

- [ ] Feature inexistente.
- [ ] Arquivo obrigatório ausente.
- [ ] ID duplicado.
- [ ] Requisito sem prova.
- [ ] Teste sem requisito.
- [ ] Evidência com SHA divergente.
- [ ] Caminho malicioso e conteúdo com metacaracteres.
- [ ] Feature válida retorna exit code 0.

## Definition of Done

- [ ] Testes positivos e negativos passam.
- [ ] Relatório JSON possui schema documentado.
- [ ] Shellcheck ou equivalente executado quando disponível.
- [ ] Funciona no ambiente local suportado.
- [ ] Evidência anexada ao SHA final.
