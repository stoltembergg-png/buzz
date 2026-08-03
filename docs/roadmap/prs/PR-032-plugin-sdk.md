# PR-032 — SDK mínimo de plugins

**Status:** planejada
**Depende de:** PR-031

## Objetivo
Expor uma API mínima, versionada e capability-aware para ferramentas e hooks permitidos.

## Requisitos
- [ ] `REQ-PLG-3201` SDK possui versão e negociação de compatibilidade.
- [ ] `REQ-PLG-3202` Toda operação exige capability emitida pelo host.
- [ ] `REQ-PLG-3203` Tipos de request/response e erros são estáveis e serializáveis.
- [ ] `REQ-PLG-3204` Hooks têm ordem, timeout, cancelamento e limites documentados.
- [ ] `REQ-PLG-3205` Plugins não recebem objetos internos mutáveis do Buzz.
- [ ] `REQ-PLG-3206` Deprecações têm janela e comportamento explícitos.

## Testes
- [ ] Handshake compatível/incompatível.
- [ ] Capability ausente/expirada/revogada.
- [ ] Request válido, inválido, timeout e cancelamento.
- [ ] Hook ordering, falha e isolamento entre plugins.
- [ ] Versão anterior e fixture futura.
- [ ] Nenhum segredo em erros/telemetria.

## Done
- [ ] API mínima; sem APIs experimentais não usadas.
- [ ] Exemplos e contrato publicados no repositório.
- [ ] Evidência local no SHA final.
