# PR-010 — Descoberta read-only de providers

**Status:** planejada
**Depende de:** PR-009

## Objetivo
Consultar providers e modelos expostos pelo Hermes sem modificar configuração ou credenciais.

## Requisitos
- [ ] `REQ-PROV-1001` Descoberta é somente leitura e cancelável.
- [ ] `REQ-PROV-1002` Resultados usam o contrato da PR-009.
- [ ] `REQ-PROV-1003` Catálogo vazio e dados parciais são estados válidos.
- [ ] `REQ-PROV-1004` Erros de autenticação/indisponibilidade são diferenciados.
- [ ] `REQ-PROV-1005` Tokens, endpoints privados e payloads sensíveis são redigidos.

## Testes
- [ ] Catálogo completo, vazio e parcial.
- [ ] Provider indisponível, timeout e cancelamento.
- [ ] IDs duplicados e modelo sem provider.
- [ ] Dados desconhecidos/futuros.
- [ ] Nenhuma escrita no config Hermes.
- [ ] Logs e respostas sem credenciais.

## Done
- [ ] Nenhuma UI ou seleção persistente nesta PR.
- [ ] Cache, se houver, tem invalidação e TTL documentados.
- [ ] Evidência local no SHA final.
