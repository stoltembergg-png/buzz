# PR-027 — Interface de MCP

**Status:** planejada
**Depende de:** PR-026

## Objetivo
Exibir catálogo, escopo, permissões, lifecycle e erros de MCP com controles acessíveis.

## Requisitos
- [ ] `REQ-MCP-2701` UI mostra estado real e última transição.
- [ ] `REQ-MCP-2702` Permissões herdadas e overrides são diferenciados.
- [ ] `REQ-MCP-2703` Ações destrutivas/restart exigem confirmação.
- [ ] `REQ-MCP-2704` Env, headers, argumentos sensíveis e paths absolutos são redigidos.
- [ ] `REQ-MCP-2705` UI suporta teclado, leitores de tela e estados de foco.

## Testes
- [ ] Loading, vazio, sucesso, erro e reconexão.
- [ ] Estados lifecycle e health.
- [ ] Permissões, overrides e denial reasons.
- [ ] Start/stop/restart e confirmação.
- [ ] Redaction e acessibilidade.
- [ ] IPC stale/out-of-order.

## Done
- [ ] Nenhum editor arbitrário de comando/env.
- [ ] Ações passam pelo policy engine.
- [ ] Evidência local no SHA final.
