# PR-038 — Geração de propostas de melhoria

**Status:** planejada
**Depende de:** PR-037

## Objetivo
Gerar propostas estruturadas de melhoria a partir de feedback, sem aplicar mudanças automaticamente.

## Requisitos
- [ ] `REQ-IMP-3801` Proposta declara alvo, estado base, alteração, justificativa e evidências usadas.
- [ ] `REQ-IMP-3802` Alvos permitidos são limitados a instruções, skills, ferramentas e seleção de modelo definida.
- [ ] `REQ-IMP-3803` Credenciais, permissões e políticas de segurança não podem ser alteradas.
- [ ] `REQ-IMP-3804` Proposta inclui testes esperados, impacto e rollback.
- [ ] `REQ-IMP-3805` Limites de quantidade, tamanho e frequência são aplicados.
- [ ] `REQ-IMP-3806` Geração não modifica arquivos/configuração.

## Testes
- [ ] Proposta válida para cada alvo permitido.
- [ ] Alvo proibido, segredo e alteração de permissão recusados.
- [ ] Evidência ausente/inconsistente.
- [ ] Limites de tamanho/frequência/quantidade.
- [ ] Dados maliciosos em feedback.
- [ ] Determinismo do formato e redaction.

## Done
- [ ] Nenhuma aplicação automática ou manual nesta PR.
- [ ] Schema e política de alvos documentados.
- [ ] Evidência local no SHA final.
