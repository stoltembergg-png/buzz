# PR-034 — Painel de execução

**Status:** planejada
**Depende de:** PR-033

## Objetivo
Exibir traces correlacionados e redigidos para diagnóstico local sem expor prompts, memórias ou credenciais.

## Requisitos
- [ ] `REQ-OBS-3401` Timeline agrupa trace, sessão e turno.
- [ ] `REQ-OBS-3402` Filtros incluem tipo, status, provider, modelo e período.
- [ ] `REQ-OBS-3403` Redaction é aplicada no backend e reforçada na UI.
- [ ] `REQ-OBS-3404` Paginação/virtualização suporta volume limitado documentado.
- [ ] `REQ-OBS-3405` Exportação, se incluída, usa formato redigido por padrão.
- [ ] `REQ-OBS-3406` Interface é acessível e funciona sem rede.

## Testes
- [ ] Loading, vazio, erro e dados parciais.
- [ ] Ordenação, filtros, paginação e atualização ao vivo.
- [ ] Redaction de cada classe de evento.
- [ ] Evento desconhecido e payload excessivo.
- [ ] Acessibilidade e navegação por teclado.
- [ ] Exportação sem segredo.

## Done
- [ ] Nenhum prompt/memória integral exibido.
- [ ] Performance e retenção documentadas.
- [ ] Evidência local no SHA final.
