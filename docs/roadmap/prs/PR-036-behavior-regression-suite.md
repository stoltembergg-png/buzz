# PR-036 — Regressão comportamental

**Status:** planejada
**Depende de:** PR-035

## Objetivo
Transformar cenários de avaliação estáveis em uma suíte de regressão com baselines, tolerâncias e revisão explícita.

## Requisitos
- [ ] `REQ-EVAL-3601` Baseline é versionada e vinculada a configuração/SHA.
- [ ] `REQ-EVAL-3602` Métricas determinísticas e probabilísticas têm regras distintas.
- [ ] `REQ-EVAL-3603` Tolerâncias e número mínimo de amostras são documentados.
- [ ] `REQ-EVAL-3604` Atualizar baseline exige justificativa e diff de impacto.
- [ ] `REQ-EVAL-3605` Falha de infraestrutura não altera baseline.
- [ ] `REQ-EVAL-3606` Execução local pode selecionar subconjunto por feature.

## Testes
- [ ] Melhora, neutralidade e regressão acima/abaixo da tolerância.
- [ ] Variância, amostras insuficientes e outliers.
- [ ] Baseline ausente/corrompida/incompatível.
- [ ] Falha de infraestrutura.
- [ ] Atualização autorizada/não autorizada.
- [ ] Relatório por feature e total.

## Done
- [ ] Nenhuma baseline é atualizada automaticamente.
- [ ] Política de revisão documentada.
- [ ] Evidência local no SHA final.
