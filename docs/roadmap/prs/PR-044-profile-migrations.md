# PR-044 — Migração e versionamento de perfis

**Status:** planejada
**Depende de:** PR-018, PR-043

## Objetivo
Migrar de forma idempotente versões de profiles, memória, resumos e configuração, com backup e rollback.

## Requisitos
- [ ] `REQ-DATA-4401` Cada formato persistido possui versão explícita.
- [ ] `REQ-DATA-4402` Migrações formam sequência ordenada sem saltos implícitos.
- [ ] `REQ-DATA-4403` Pré-flight valida espaço, permissões, integridade e backup.
- [ ] `REQ-DATA-4404` Reexecução após interrupção é segura e idempotente.
- [ ] `REQ-DATA-4405` Downgrade não suportado é recusado claramente.
- [ ] `REQ-DATA-4406` Falha restaura backup ou mantém estado anterior utilizável.
- [ ] `REQ-DATA-4407` Migração não publica dados no relay.

## Testes
- [ ] Fresh install e upgrade de cada versão suportada.
- [ ] Upgrade sequencial e tentativa de pular versão.
- [ ] Reexecução, interrupção e crash recovery.
- [ ] Disco/permissão/integridade insuficientes.
- [ ] Downgrade recusado.
- [ ] Backup/rollback e arquivos desconhecidos.
- [ ] Isolamento entre perfis.

## Done
- [ ] Matriz de versões suportadas documentada.
- [ ] Backup obrigatório antes de alteração destrutiva.
- [ ] Evidência local no SHA final.
