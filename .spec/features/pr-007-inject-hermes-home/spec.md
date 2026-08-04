# PR-007 — Injeção de HERMES_HOME

**Status:** done
**Phase:** memory
**Depends on:** PR-006

## Objective

Conectar o perfil inicializado ao processo Hermes, preservando integralmente outros runtimes ACP.

## Requirements

- **REQ-MEM-701** Inicializar o perfil antes do spawn.
- **REQ-MEM-702** Definir explicitamente `HERMES_HOME` para aliases Hermes.
- **REQ-MEM-703** Valor herdado do processo pai não substitui o perfil escopado.
- **REQ-MEM-704** Runtimes não Hermes não recebem nem têm `HERMES_HOME` removido.
- **REQ-MEM-705** Bridges de credenciais e reasoning effort permanecem funcionais e Hermes-only.
- **REQ-MEM-706** Falha de bootstrap impede o spawn com erro redigido.

## Acceptance Criteria

- @spec:AC-MEM-701 Hermes runtime recebe overlay com `HERMES_HOME=path`.
- @spec:AC-MEM-702 Non-Hermes runtime não recebe overlay e não tem `HERMES_HOME` removido.
- @spec:AC-MEM-703 `HERMES_HOME` herdado do pai é sobrescrito pelo valor escopado.
- @spec:AC-MEM-704 Mesmo contexto após restart reutiliza o mesmo home.
- @spec:AC-MEM-705 Canal, persona ou relay diferentes isolam homes.
- @spec:AC-MEM-706 Falha de bootstrap retorna erro tipado e não inicia processo.
- @spec:AC-MEM-707 Erros não vazam conteúdo de configuração.

## Done

- [x] Funções puras testáveis (8 testes unitários).
- [x] Overlay só anexa `HERMES_HOME` para runtime Hermes; nunca remove de outros.
- [x] `apply_overlay` substitui valor herdado; preserva env intacto quando `None`.
- [x] Falha de bootstrap propagada como `InjectError::Bootstrap`.
- [x] Erros não contêm conteúdo da configuração.
