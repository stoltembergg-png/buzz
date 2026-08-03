# PR-011 — Seleção e persistência de provider/modelo

**Status:** planejada
**Depende de:** PR-010

## Objetivo
Permitir seleção explícita por persona, persistida localmente e validada contra o catálogo descoberto.

## Requisitos
- [ ] `REQ-PROV-1101` Seleção usa IDs estáveis, não labels.
- [ ] `REQ-PROV-1102` Persistência é isolada por persona.
- [ ] `REQ-PROV-1103` Restart restaura a seleção.
- [ ] `REQ-PROV-1104` Provider/modelo ausente gera estado reparável, não escolha silenciosa.
- [ ] `REQ-PROV-1105` Escrita é atômica e não inclui credenciais.

## Testes
- [ ] Seleção e round-trip após restart.
- [ ] Duas personas com escolhas distintas.
- [ ] ID inválido, removido e catálogo vazio.
- [ ] Falha/concorrência de escrita.
- [ ] Migração de configuração antiga.
- [ ] UI/IPC não expõe segredos.

## Done
- [ ] Sem fallback automático nesta PR.
- [ ] Rollback preserva configuração anterior.
- [ ] Evidência local no SHA final.
