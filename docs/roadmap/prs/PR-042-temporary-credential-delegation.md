# PR-042 — Delegação temporária de credenciais

**Status:** planejada
**Depende de:** PR-003

## Objetivo
Substituir a chave privada persistente no ambiente do Hermes por capability/token de curta duração e escopo mínimo.

## Requisitos
- [ ] `REQ-SEC-4201` Chave privada principal não é entregue ao processo Hermes/filhos.
- [ ] `REQ-SEC-4202` Capability possui audiência, escopo, expiração, nonce e identidade da sessão.
- [ ] `REQ-SEC-4203` Operações permitidas são mínimas e explicitamente listadas.
- [ ] `REQ-SEC-4204` Replay, uso após expiração e uso fora do escopo são recusados.
- [ ] `REQ-SEC-4205` Revogação/encerramento da sessão invalida capabilities pendentes.
- [ ] `REQ-SEC-4206` Tokens não aparecem em argv, logs, eventos ou relatórios.
- [ ] `REQ-SEC-4207` Migração mantém fallback apenas sob flag temporária e auditável.

## Testes
- [ ] Emissão e uso permitido.
- [ ] Expiração, revogação, audiência e escopo incorretos.
- [ ] Replay e nonce duplicado.
- [ ] Sessões/personas/canais distintos.
- [ ] Processo filho sem acesso implícito.
- [ ] Redaction em logs, crashes e eventos.
- [ ] Migração/fallback temporário e remoção futura.

## Done
- [ ] Threat model e fluxo de assinatura documentados.
- [ ] Chave privada ausente do ambiente final.
- [ ] Evidência local no SHA final.
