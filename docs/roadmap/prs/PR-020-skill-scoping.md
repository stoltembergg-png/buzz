# PR-020 — Escopo de skills

**Status:** planejada
**Depende de:** PR-019, PR-004

## Objetivo
Definir configuração e precedência de skills globais, por persona e por canal.

## Requisitos
- [ ] `REQ-SKL-2001` Escopos são identificados por IDs estáveis.
- [ ] `REQ-SKL-2002` Precedência global < persona < canal é explícita e testada.
- [ ] `REQ-SKL-2003` Disable explícito vence enable herdado conforme contrato.
- [ ] `REQ-SKL-2004` Configuração é atômica, versionada e secret-free.
- [ ] `REQ-SKL-2005` Skills incompatíveis permanecem visíveis com erro, não ativadas silenciosamente.

## Testes
- [ ] Herança e override em todas as camadas.
- [ ] Persona/canal/relay diferentes.
- [ ] Enable/disable conflitante.
- [ ] Skill removida, duplicada ou incompatível.
- [ ] Restart, concorrência e migração.

## Done
- [ ] Nenhuma execução de skill.
- [ ] Sem UI nesta PR.
- [ ] Evidência local no SHA final.
