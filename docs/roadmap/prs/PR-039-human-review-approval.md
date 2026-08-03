# PR-039 — Revisão e aprovação humana

**Status:** planejada
**Depende de:** PR-038

## Objetivo
Exibir propostas com diff, justificativa, evidências, testes e rollback, exigindo decisão humana explícita.

## Requisitos
- [ ] `REQ-IMP-3901` Estado base e proposto são exibidos em diff canônico.
- [ ] `REQ-IMP-3902` Aprovar, rejeitar e editar são ações explícitas e auditadas.
- [ ] `REQ-IMP-3903` Proposta stale após mudança do estado base exige nova revisão.
- [ ] `REQ-IMP-3904` Alvos proibidos continuam bloqueados mesmo após edição.
- [ ] `REQ-IMP-3905` Aprovação não aplica a mudança nesta PR.
- [ ] `REQ-IMP-3906` UI é acessível e não exibe segredos.

## Testes
- [ ] Aprovar, rejeitar, editar e cancelar.
- [ ] Base alterada/stale e conflito.
- [ ] Edição para alvo proibido ou payload inválido.
- [ ] Evidência/testes/rollback ausentes.
- [ ] Auditoria, redaction e acessibilidade.
- [ ] Ações duplicadas e IPC fora de ordem.

## Done
- [ ] Nenhuma aplicação da proposta.
- [ ] Registro de decisão é imutável/versionado.
- [ ] Evidência local no SHA final.
