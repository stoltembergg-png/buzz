# PR-008 — Harness funcional de persistência

**Status:** done
**Phase:** memory
**Depends on:** PR-007

## Objective

Provar localmente, com Hermes real ou fixture compatível, que a memória persiste no mesmo escopo e não vaza entre escopos.

## Requirements

- **REQ-MEM-801** Um único comando executa o cenário completo.
- **REQ-MEM-802** O harness grava uma memória com marcador aleatório.
- **REQ-MEM-803** Reinício no mesmo escopo recupera o marcador.
- **REQ-MEM-804** Canal, persona e relay alternativos não recuperam o marcador.
- **REQ-MEM-805** O relatório registra SHA, Hermes, OS, comandos e resultados.
- **REQ-MEM-806** Segredos e conteúdo privado não são impressos integralmente.

## Acceptance Criteria

- @spec:AC-MEM-801 Harness roda todos os 5 cenários em um único comando (`cargo test --lib persistence_harness`).
- @spec:AC-MEM-802 Marcador aleatório é gravado no primeiro escopo (write scenario).
- @spec:AC-MEM-803 Reinício no mesmo escopo recupera o marcador.
- @spec:AC-MEM-804 Canal, persona ou relay diferentes não recuperam o marcador.
- @spec:AC-MEM-805 Relatório JSON possui `schema=1`, `os`, `arch`, `scenarios` e `all_passed`.
- @spec:AC-MEM-806 Marcador recuperado é truncado a 8 chars (não imprime conteúdo integral).
- @spec:AC-MEM-807 Harness é determinístico e limpa dados temporários (TempDir).

## Done

- [x] 9 testes unitários passam em ~50ms.
- [x] Schema versionado (`REPORT_SCHEMA_VERSION = 1`).
- [x] Falhas produzem `all_passed=false` (sentinel `test_all_scenarios_pass`).
- [x] Nenhum dado temporário vaza (TempDir auto-cleanup).
- [x] Evidência JSON vinculada ao SHA.
