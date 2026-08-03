# PR-009 — Contrato de capacidades de provider

**Status:** planejada
**Depende de:** PR-003

## Objetivo
Definir tipos internos versionados para providers, modelos e capacidades antes de qualquer UI ou roteamento.

## Requisitos
- [ ] `REQ-PROV-901` Identidade estável de provider/modelo separada de label.
- [ ] `REQ-PROV-902` Capacidades incluem contexto, reasoning, tools, imagens, structured output, custo e disponibilidade.
- [ ] `REQ-PROV-903` Campos desconhecidos são tolerados conforme regra documentada.
- [ ] `REQ-PROV-904` Nenhum segredo faz parte do contrato.
- [ ] `REQ-PROV-905` Rust e TypeScript compartilham semântica equivalente.

## Testes
- [ ] Serialização/deserialize round-trip.
- [ ] Campos ausentes, desconhecidos e versões futuras.
- [ ] IDs duplicados e labels mutáveis.
- [ ] Redaction e ausência de credenciais.
- [ ] Fixtures Rust/TypeScript compatíveis.

## Done
- [ ] Nenhuma descoberta, UI ou seleção é alterada.
- [ ] Schema e migração futura documentados.
- [ ] Evidência local no SHA final.
