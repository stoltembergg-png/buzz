# PR-008 — Harness funcional de persistência

**Status:** planejada
**Depende de:** PR-007

## Objetivo

Provar localmente, com Hermes real ou fixture compatível, que a memória persiste no mesmo escopo e não vaza entre escopos.

## Requisitos
- [ ] `REQ-MEM-801` Um único comando executa o cenário completo.
- [ ] `REQ-MEM-802` O harness grava uma memória com marcador aleatório.
- [ ] `REQ-MEM-803` Reinício no mesmo escopo recupera o marcador.
- [ ] `REQ-MEM-804` Canal, persona e relay alternativos não recuperam o marcador.
- [ ] `REQ-MEM-805` O relatório registra SHA, Hermes, OS, comandos e resultados.
- [ ] `REQ-MEM-806` Segredos e conteúdo privado não são impressos integralmente.

## Cenários
- [ ] First run sem perfil.
- [ ] Restart no mesmo escopo.
- [ ] Segundo canal.
- [ ] Segunda persona.
- [ ] Segundo relay/comunidade.
- [ ] Configuração principal ausente.
- [ ] Erro controlado de filesystem.

## Done
- [ ] Harness é repetível e limpa somente dados temporários que criou.
- [ ] Relatório JSON possui schema versionado.
- [ ] Falhas produzem exit code diferente de zero.
- [ ] Resultado manual é anexado ao SHA final.
