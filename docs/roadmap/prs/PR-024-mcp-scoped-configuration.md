# PR-024 — Configuração MCP por persona/canal

**Status:** planejada
**Depende de:** PR-023, PR-004

## Objetivo
Aplicar seleção de MCPs por escopo estável sem vazar configuração entre agentes.

## Requisitos
- [ ] `REQ-MCP-2401` Configuração usa relay/canal/persona estáveis.
- [ ] `REQ-MCP-2402` Precedência global, persona e canal é explícita.
- [ ] `REQ-MCP-2403` Enable/disable é atômico e versionado.
- [ ] `REQ-MCP-2404` Referências ausentes ficam reparáveis, não substituídas.
- [ ] `REQ-MCP-2405` Configuração não armazena segredo inline.

## Testes
- [ ] Herança/override em todos os níveis.
- [ ] Isolamento entre persona, canal e relay.
- [ ] MCP removido/renomeado/duplicado.
- [ ] Restart, concorrência e escrita interrompida.
- [ ] Migração e rollback.

## Done
- [ ] Nenhum servidor é iniciado.
- [ ] Sem permissões ou UI nesta PR.
- [ ] Evidência local no SHA final.
