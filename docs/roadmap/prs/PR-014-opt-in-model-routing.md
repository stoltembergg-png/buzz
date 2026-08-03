# PR-014 — Roteamento automático opt-in

**Status:** planejada
**Depende de:** PR-013

## Objetivo
Selecionar modelo por capacidades, política e orçamento somente quando o usuário habilitar o roteamento.

## Requisitos
- [ ] `REQ-PROV-1401` Desativado por padrão.
- [ ] `REQ-PROV-1402` Política recebe capacidades exigidas, orçamento e preferências.
- [ ] `REQ-PROV-1403` Decisão é determinística para o mesmo catálogo/entrada.
- [ ] `REQ-PROV-1404` Explicação da decisão não expõe prompt ou segredos.
- [ ] `REQ-PROV-1405` Modelos incompatíveis são excluídos antes do ranking.
- [ ] `REQ-PROV-1406` Ausência de candidato gera erro explícito.

## Testes
- [ ] Roteamento desligado.
- [ ] Capacidade obrigatória, preferência e orçamento.
- [ ] Empates e ordenação estável.
- [ ] Catálogo parcial/alterado.
- [ ] Nenhum candidato.
- [ ] Integração com fallback sem ciclos ou dupla contagem.

## Done
- [ ] Política versionada e auditável.
- [ ] Limites de custo documentados.
- [ ] Evidência local no SHA final.
