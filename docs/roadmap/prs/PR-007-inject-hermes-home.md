# PR-007 — Injeção de HERMES_HOME

**Status:** planejada
**Depende de:** PR-006

## Objetivo

Conectar o perfil inicializado ao processo Hermes, preservando integralmente outros runtimes ACP.

## Requisitos
- [ ] `REQ-MEM-701` Inicializar o perfil antes do spawn.
- [ ] `REQ-MEM-702` Definir explicitamente `HERMES_HOME` para aliases Hermes.
- [ ] `REQ-MEM-703` Valor herdado do processo pai não substitui o perfil escopado.
- [ ] `REQ-MEM-704` Runtimes não Hermes não recebem nem têm `HERMES_HOME` removido.
- [ ] `REQ-MEM-705` Bridges de credenciais e reasoning effort permanecem funcionais e Hermes-only.
- [ ] `REQ-MEM-706` Falha de bootstrap impede o spawn com erro redigido.

## Testes
- [ ] Ambiente do child para cada alias Hermes.
- [ ] Non-Hermes permanece inalterado.
- [ ] Mesmo contexto após restart reutiliza home.
- [ ] Canal/persona/relay diferentes isolam homes.
- [ ] Parent `HERMES_HOME` não vence.
- [ ] Credential e reasoning bridges não regressam.
- [ ] Falha de filesystem não inicia processo.

## Done
- [ ] Nenhum conteúdo de memória aparece em eventos/logs.
- [ ] Testes focados de `buzz-acp` e Desktop passam.
- [ ] Smoke test local registrado.
- [ ] Rollback remove apenas a integração de spawn.
