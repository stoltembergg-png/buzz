# PR-043 — Backup e exportação local

**Status:** planejada
**Depende de:** PR-018, PR-040

## Objetivo
Exportar e restaurar memória, resumos, configuração não secreta e snapshots aprovados com segurança.

## Requisitos
- [ ] `REQ-DATA-4301` Exportação é versionada, possui manifest e checksums.
- [ ] `REQ-DATA-4302` Segredos, tokens, OAuth stores e chave privada são excluídos por padrão e não podem ser incluídos acidentalmente.
- [ ] `REQ-DATA-4303` Usuário seleciona escopos e categorias explicitamente.
- [ ] `REQ-DATA-4304` Restore possui dry-run, valida integridade e detecta conflitos.
- [ ] `REQ-DATA-4305` Restore nunca sobrescreve sem política/confirmacão explícita.
- [ ] `REQ-DATA-4306` Path traversal, symlinks e archives maliciosos são recusados.

## Testes
- [ ] Export mínimo/completo e múltiplos escopos.
- [ ] Exclusão de toda classe de segredo.
- [ ] Checksum/manifest ausente ou alterado.
- [ ] Archive traversal, symlink e decompression bomb limitada.
- [ ] Dry-run, conflito, merge/replace permitido.
- [ ] Falha parcial e rollback do restore.
- [ ] Compatibilidade de versão.

## Done
- [ ] Formato e limites documentados.
- [ ] Restore é transacional.
- [ ] Evidência local no SHA final.
