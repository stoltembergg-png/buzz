# PR-037 — Registro de feedback

**Status:** planejada
**Depende de:** PR-033, PR-004

## Objetivo
Armazenar localmente resultados, correções e falhas como registros estruturados e escopados.

## Requisitos
- [ ] `REQ-IMP-3701` Registro possui versão, origem, escopo, timestamp e correlação.
- [ ] `REQ-IMP-3702` Conteúdo sensível é minimizado/redigido antes da persistência.
- [ ] `REQ-IMP-3703` Retenção, tamanho e exclusão são explícitos.
- [ ] `REQ-IMP-3704` Corrupção de um registro não invalida todo o store.
- [ ] `REQ-IMP-3705` Feedback de um escopo não é lido por outro.
- [ ] `REQ-IMP-3706` Nenhum feedback é publicado no relay.

## Testes
- [ ] Registros de sucesso, falha e correção.
- [ ] Isolamento por persona/canal/relay.
- [ ] Redaction e limite de tamanho.
- [ ] Retenção/expiração/exclusão.
- [ ] Arquivo corrompido e escrita interrompida.
- [ ] Concorrência e restart.

## Done
- [ ] Nenhuma proposta ou autoalteração nesta PR.
- [ ] Política de privacidade documentada.
- [ ] Evidência local no SHA final.
