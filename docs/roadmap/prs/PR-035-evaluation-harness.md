# PR-035 — Harness de avaliação

**Status:** planejada
**Depende de:** PR-033

## Objetivo
Executar cenários versionados e repetíveis para comparar comportamento, custo, latência e uso de ferramentas.

## Requisitos
- [ ] `REQ-EVAL-3501` Cenários, entradas e critérios são versionados.
- [ ] `REQ-EVAL-3502` Execução registra modelo/provider/configuração e SHA.
- [ ] `REQ-EVAL-3503` Dados sensíveis não entram nas fixtures padrão.
- [ ] `REQ-EVAL-3504` Resultados distinguem falha de infraestrutura e falha comportamental.
- [ ] `REQ-EVAL-3505` Repetições, seeds e tolerâncias são explícitas.
- [ ] `REQ-EVAL-3506` Relatório é legível e machine-readable.

## Testes
- [ ] Cenário válido/inválido e fixture ausente.
- [ ] Seed/repetição e agregação.
- [ ] Timeout, cancelamento e provider indisponível.
- [ ] Classificação de falhas.
- [ ] Redaction e schema do relatório.
- [ ] Comparação de duas execuções.

## Done
- [ ] Sem gate automático de regressão nesta PR.
- [ ] Custos e limitações documentados.
- [ ] Evidência local no SHA final.
