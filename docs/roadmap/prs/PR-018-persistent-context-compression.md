# PR-018 — Compressão persistente por escopo

**Status:** planejada
**Depende de:** PR-017, PR-007

## Objetivo
Persistir resumos validados no escopo local persona/canal e recuperá-los com segurança após restart.

## Requisitos
- [ ] `REQ-CTX-1801` Persistência usa o perfil escopado da memória.
- [ ] `REQ-CTX-1802` Escrita é atômica, versionada e ligada ao hash de origem.
- [ ] `REQ-CTX-1803` Corrupção ou versão desconhecida não substitui histórico bruto.
- [ ] `REQ-CTX-1804` Resumos não vazam entre persona, canal ou relay.
- [ ] `REQ-CTX-1805` Rollback pode ignorar/remover índice sem apagar histórico bruto.

## Testes
- [ ] Restart no mesmo escopo.
- [ ] Isolamento entre escopos.
- [ ] Arquivo truncado, corrompido e versão futura.
- [ ] Escrita concorrente e falha parcial.
- [ ] Hash de origem divergente.
- [ ] Rollback para sessão sem resumo.

## Done
- [ ] Nenhuma sincronização via relay.
- [ ] Dados antigos permanecem recuperáveis.
- [ ] Evidência local no SHA final.
