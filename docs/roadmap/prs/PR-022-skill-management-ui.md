# PR-022 — Interface de gerenciamento de skills

**Status:** planejada
**Depende de:** PR-021

## Objetivo
Permitir visualizar disponibilidade, compatibilidade e ativação de skills sem editor arbitrário de arquivos.

## Requisitos
- [ ] `REQ-SKL-2201` UI diferencia global, persona e canal.
- [ ] `REQ-SKL-2202` Estado herdado, override, incompatível e erro são explícitos.
- [ ] `REQ-SKL-2203` Alterações exigem confirmação quando afetam sessão ativa.
- [ ] `REQ-SKL-2204` Nenhum conteúdo secreto ou caminho absoluto é exibido.
- [ ] `REQ-SKL-2205` Acessibilidade por teclado, labels e estados de foco.

## Testes
- [ ] Loading, vazio, sucesso e erro.
- [ ] Herança/override/disable e incompatibilidade.
- [ ] Persistência e rollback de falha.
- [ ] Sessão ativa e confirmação.
- [ ] Acessibilidade e navegação por teclado.
- [ ] Redaction de paths/segredos.

## Done
- [ ] Sem instalação/edição de skill.
- [ ] IPC validada e tipada.
- [ ] Evidência local no SHA final.
