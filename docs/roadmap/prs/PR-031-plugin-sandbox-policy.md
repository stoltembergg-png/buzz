# PR-031 — Sandbox e permissões de plugins

**Status:** planejada
**Depende de:** PR-030

## Objetivo
Executar plugins somente com capabilities declaradas e uma política default-deny verificável.

## Requisitos
- [ ] `REQ-PLG-3101` Filesystem, rede, processos, ferramentas e segredos exigem capability explícita.
- [ ] `REQ-PLG-3102` Ausência de capability significa negar.
- [ ] `REQ-PLG-3103` Escopos são mínimos, canônicos e não ampliáveis pelo plugin.
- [ ] `REQ-PLG-3104` Segredos são entregues por handle/capability de curta duração.
- [ ] `REQ-PLG-3105` Violação encerra ou bloqueia a operação conforme política.
- [ ] `REQ-PLG-3106` Plataforma sem sandbox suficiente falha fechada ou exige modo explicitamente inseguro.

## Testes
- [ ] Allow/deny/default-deny por capability.
- [ ] Traversal, symlink, rede externa e subprocesso não declarado.
- [ ] Exfiltração por logs/erros e acesso a env.
- [ ] Capability expirada/revogada.
- [ ] Escape/elevação simulados.
- [ ] Diferenças entre plataformas suportadas.

## Done
- [ ] Threat model e limitações por OS documentados.
- [ ] Nenhuma permissão implícita de compatibilidade.
- [ ] Evidência local no SHA final.
