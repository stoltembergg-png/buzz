# PR-019 — Descoberta de skills

**Status:** planejada
**Depende de:** PR-003

## Objetivo
Descobrir e normalizar skills Hermes disponíveis sem executá-las ou alterar configuração.

## Requisitos
- [ ] `REQ-SKL-1901` Descoberta é somente leitura.
- [ ] `REQ-SKL-1902` IDs, versões, origem e capacidades são separados de labels.
- [ ] `REQ-SKL-1903` Manifestos inválidos são isolados com diagnóstico redigido.
- [ ] `REQ-SKL-1904` Duplicatas têm resolução determinística.
- [ ] `REQ-SKL-1905` Caminhos externos, symlinks inseguros e traversal são recusados.

## Testes
- [ ] Diretório ausente/vazio/completo.
- [ ] Manifesto inválido e versão desconhecida.
- [ ] Duplicidade e ordem estável.
- [ ] Symlink, traversal e arquivo excessivo.
- [ ] Nenhuma execução ou leitura de segredo.

## Done
- [ ] Sem ativação ou UI.
- [ ] Cache/invalidação documentados se usados.
- [ ] Evidência local no SHA final.
