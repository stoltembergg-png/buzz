# PR-021 — Ativação e carregamento de skills

**Status:** planejada
**Depende de:** PR-020

## Objetivo
Materializar no runtime apenas as skills efetivamente habilitadas, compatíveis e seguras.

## Requisitos
- [ ] `REQ-SKL-2101` Resolver escopos antes do spawn/sessão.
- [ ] `REQ-SKL-2102` Validar versão, dependências e capacidades exigidas.
- [ ] `REQ-SKL-2103` Falha de uma skill não ativa conteúdo parcial.
- [ ] `REQ-SKL-2104` Paths são canônicos e confinados à raiz permitida.
- [ ] `REQ-SKL-2105` A lista carregada é observável sem revelar conteúdo privado.

## Testes
- [ ] Uma/múltiplas skills e nenhuma habilitada.
- [ ] Versão/dependência incompatível.
- [ ] Manifesto muda entre descoberta e load.
- [ ] Symlink/traversal e arquivo removido.
- [ ] Falha parcial, cancelamento e restart.
- [ ] Runtime não Hermes permanece inalterado.

## Done
- [ ] Carregamento é transacional por sessão.
- [ ] Sem UI nesta PR.
- [ ] Evidência local no SHA final.
