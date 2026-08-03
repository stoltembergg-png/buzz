# PR-026 — Lifecycle e health checks MCP

**Status:** planejada
**Depende de:** PR-025

## Objetivo
Gerenciar startup, readiness, timeout, crash, retry, restart e shutdown dos MCPs permitidos.

## Requisitos
- [ ] `REQ-MCP-2601` Estado possui máquina explícita e transições válidas.
- [ ] `REQ-MCP-2602` Startup/readiness têm deadlines separados.
- [ ] `REQ-MCP-2603` Retry é limitado, com backoff e sem restart storm.
- [ ] `REQ-MCP-2604` Shutdown encerra filhos e recursos sem órfãos.
- [ ] `REQ-MCP-2605` Health não executa ferramentas destrutivas.
- [ ] `REQ-MCP-2606` Eventos não expõem env, headers ou payload privado.

## Testes
- [ ] Processo saudável, lento, silencioso e inválido.
- [ ] Crash antes/depois de ready.
- [ ] Timeout, cancelamento e shutdown forçado.
- [ ] Limite/backoff de retry.
- [ ] Múltiplos MCPs independentes.
- [ ] Processo órfão e limpeza de recursos.

## Done
- [ ] Fake servers/processes determinísticos nos testes.
- [ ] Sem UI nesta PR.
- [ ] Evidência local no SHA final.
