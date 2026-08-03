# PR-041 — Autoaperfeiçoamento opt-in

**Status:** planejada
**Depende de:** PR-040

## Objetivo
Automatizar geração, avaliação e aplicação de melhorias somente com opt-in e limites rígidos.

## Requisitos
- [ ] `REQ-IMP-4101` Desativado por padrão e configurado por escopo.
- [ ] `REQ-IMP-4102` Limites de frequência, quantidade, custo e janela são obrigatórios.
- [ ] `REQ-IMP-4103` Somente alvos previamente permitidos podem ser automatizados.
- [ ] `REQ-IMP-4104` Permissões, credenciais, segurança e código executável permanecem proibidos.
- [ ] `REQ-IMP-4105` Avaliação pós-aplicação compara baseline e executa rollback quando necessário.
- [ ] `REQ-IMP-4106` Kill switch interrompe geração e aplicação pendente.
- [ ] `REQ-IMP-4107` Toda ação possui auditoria e justificativa.

## Testes
- [ ] Desativado/ativado e isolamento por escopo.
- [ ] Limites de frequência, custo e quantidade.
- [ ] Alvo proibido e tentativa de ampliar política.
- [ ] Avaliação positiva, inconclusiva e regressiva.
- [ ] Rollback automático e falha de rollback.
- [ ] Kill switch durante cada fase.
- [ ] Restart e recuperação de estado pendente.

## Done
- [ ] Modo experimental e opt-in explícito.
- [ ] Riscos/limitações documentados.
- [ ] Evidência local no SHA final.
