# PR-023 — Catálogo de MCPs

**Status:** planejada
**Depende de:** PR-003

## Objetivo
Descobrir servidores MCP configurados, transportes e capacidades sem iniciá-los.

## Requisitos
- [ ] `REQ-MCP-2301` Identidade estável separada de label/comando.
- [ ] `REQ-MCP-2302` Transportes stdio e HTTP são representados por tipos distintos.
- [ ] `REQ-MCP-2303` Credenciais, headers e env sensível são redigidos.
- [ ] `REQ-MCP-2304` Configuração inválida é isolada e diagnosticada.
- [ ] `REQ-MCP-2305` Duplicatas têm resolução determinística.

## Testes
- [ ] Catálogo vazio, stdio, HTTP e misto.
- [ ] Entrada inválida, duplicada e versão futura.
- [ ] Comando/URL ausente ou malformado.
- [ ] Redaction de env, tokens e headers.
- [ ] Nenhum processo ou conexão é iniciado.

## Done
- [ ] Sem escopo, permissão ou UI.
- [ ] Schema documentado.
- [ ] Evidência local no SHA final.
