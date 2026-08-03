# PR-040 — Aplicação versionada de melhorias

**Status:** planejada
**Depende de:** PR-039

## Objetivo
Aplicar somente propostas aprovadas, com snapshot, transação, validação e rollback verificável.

## Requisitos
- [ ] `REQ-IMP-4001` Aplicação exige decisão aprovada e não stale.
- [ ] `REQ-IMP-4002` Snapshot completo do alvo é criado antes da alteração.
- [ ] `REQ-IMP-4003` Escrita é atômica e valida o resultado antes do commit.
- [ ] `REQ-IMP-4004` Falha parcial restaura snapshot automaticamente.
- [ ] `REQ-IMP-4005` Operações concorrentes no mesmo alvo são serializadas.
- [ ] `REQ-IMP-4006` Permissões, credenciais e política permanecem fora do conjunto aplicável.

## Testes
- [ ] Aplicação válida por alvo permitido.
- [ ] Proposta rejeitada/stale/não aprovada.
- [ ] Snapshot, falha parcial e rollback.
- [ ] Validação pós-escrita falha.
- [ ] Concorrência, crash e recuperação de journal.
- [ ] Tentativa de alterar alvo proibido.

## Done
- [ ] Sem aplicação automática/agendada.
- [ ] Histórico de versões e rollback documentados.
- [ ] Evidência local no SHA final.
