# PR-028 — Manifesto de plugin

**Status:** planejada
**Depende de:** PR-003

## Objetivo
Definir schema versionado para identidade, entrypoint, compatibilidade, permissões, dependências e integridade de plugins.

## Requisitos
- [ ] `REQ-PLG-2801` ID e versão seguem formato canônico.
- [ ] `REQ-PLG-2802` Entrypoints e runtimes suportados são declarados.
- [ ] `REQ-PLG-2803` Capacidades/permissões são explícitas e default-deny.
- [ ] `REQ-PLG-2804` Integridade usa hash/assinatura com algoritmo versionado.
- [ ] `REQ-PLG-2805` Compatibilidade com Buzz/SDK e dependências é verificável.
- [ ] `REQ-PLG-2806` Campos desconhecidos seguem política documentada.

## Testes
- [ ] Manifesto mínimo/completo.
- [ ] ID, versão e entrypoint inválidos.
- [ ] Hash ausente/incorreto e algoritmo desconhecido.
- [ ] Capability não reconhecida.
- [ ] Dependência ausente/cíclica/incompatível.
- [ ] Fixtures de versões futuras.

## Done
- [ ] Nenhum plugin é carregado.
- [ ] Schema e threat model documentados.
- [ ] Evidência local no SHA final.
