# PR-033 — Eventos de execução

**Status:** planejada
**Depende de:** PR-003

## Objetivo
Normalizar eventos correlacionáveis de provider, modelo, ferramentas, MCP, memória, compressão, plugins e erros.

## Requisitos
- [ ] `REQ-OBS-3301` Schema versionado com trace/session/turn/event IDs.
- [ ] `REQ-OBS-3302` Ordenação e timestamps têm semântica documentada.
- [ ] `REQ-OBS-3303` Conteúdo de prompt, memória, tool payload e segredos é excluído/redigido.
- [ ] `REQ-OBS-3304` Eventos desconhecidos/futuros não quebram consumidores.
- [ ] `REQ-OBS-3305` Emissão não pode bloquear o turno indefinidamente.
- [ ] `REQ-OBS-3306` Limites de tamanho e retenção são explícitos.

## Testes
- [ ] Schema/round-trip e versões futuras.
- [ ] Ordem, correlação e concorrência.
- [ ] Redaction por origem de evento.
- [ ] Payload excessivo, sink lento/indisponível.
- [ ] Duplicata e perda parcial.
- [ ] Impacto de desempenho dentro do limite definido.

## Done
- [ ] Sem painel nesta PR.
- [ ] Política de privacidade/retenção documentada.
- [ ] Evidência local no SHA final.
