# PR-004 — Contexto estável de runtime

**Status:** planejada
**Fase:** Memória
**Depende de:** PR-003

## Objetivo

Propagar uma identidade canônica de relay/comunidade, canal, persona e runtime até a camada que constrói o processo ACP, sem alterar comportamento de execução.

## Requisitos

- [ ] `REQ-MEM-401` Usar identificadores estáveis, nunca display names.
- [ ] `REQ-MEM-402` Todos os entrypoints de agente produzem o mesmo tipo `RuntimeScopeContext`.
- [ ] `REQ-MEM-403` Relay URL é normalizada de forma determinística.
- [ ] `REQ-MEM-404` Canal usa UUID/coordinate estável.
- [ ] `REQ-MEM-405` Persona usa ID estável ou fallback canônico documentado.
- [ ] `REQ-MEM-406` O contexto não contém chaves, tokens, prompts ou memória.

## Escopo técnico

- Introduzir tipo focado para contexto de escopo.
- Mapear criação por onboarding, persona existente, restart e agentes remotos suportados.
- Passar o contexto até o spawn sem ainda derivar caminho ou criar arquivos.
- Manter serialização interna mínima e redigida.

## Testes

- [ ] Mesma entidade em entrypoints diferentes produz igualdade.
- [ ] Alteração de display name não altera identidade.
- [ ] Relay/canal/persona diferentes alteram o contexto correto.
- [ ] Inputs ausentes falham com erro explícito.
- [ ] Debug/serialização não expõe campos sensíveis.
- [ ] Runtimes existentes continuam iniciando com comportamento idêntico.

## Definition of Done

- [ ] Todos os call sites mapeados.
- [ ] Testes unitários e de integração passam.
- [ ] Nenhuma mudança de filesystem ou `HERMES_HOME` nesta PR.
- [ ] Evidência local registrada para o SHA final.
