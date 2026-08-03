# PR-030 — Lifecycle de plugins

**Status:** planejada
**Depende de:** PR-029

## Objetivo
Implementar instalação local, ativação, desativação, atualização e remoção de plugins como operações transacionais.

## Requisitos
- [ ] `REQ-PLG-3001` Instalação valida manifesto, integridade, compatibilidade e espaço antes do commit.
- [ ] `REQ-PLG-3002` Atualização preserva versão anterior até sucesso completo.
- [ ] `REQ-PLG-3003` Ativar/desativar é idempotente e persistente.
- [ ] `REQ-PLG-3004` Remoção não apaga dados externos sem política explícita.
- [ ] `REQ-PLG-3005` Falha parcial executa rollback verificável.
- [ ] `REQ-PLG-3006` Operações concorrentes no mesmo plugin são serializadas.

## Testes
- [ ] Instalação, ativação, desativação e remoção.
- [ ] Update válido, downgrade e versão incompatível.
- [ ] Falha de cópia, integridade e disco.
- [ ] Concorrência e processo interrompido.
- [ ] Rollback e recuperação de journal.
- [ ] Dados do usuário preservados.

## Done
- [ ] Plugin ainda não recebe capabilities de execução.
- [ ] Journal e rollback documentados.
- [ ] Evidência local no SHA final.
