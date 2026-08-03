# PR-029 — Loader local seguro de plugins

**Status:** planejada
**Depende de:** PR-028

## Objetivo
Descobrir, verificar e preparar plugins locais declarativos sem executar código não validado.

## Requisitos
- [ ] `REQ-PLG-2901` Diretórios e manifestos ficam confinados a raízes permitidas.
- [ ] `REQ-PLG-2902` Integridade é verificada antes de qualquer import/load.
- [ ] `REQ-PLG-2903` Symlink, traversal e arquivos especiais são recusados.
- [ ] `REQ-PLG-2904` Compatibilidade e dependências são resolvidas deterministicamente.
- [ ] `REQ-PLG-2905` Loader produz plano de carga, não inicia plugin nesta PR.

## Testes
- [ ] Plugin válido, vazio e múltiplos plugins.
- [ ] Hash alterado, entrypoint ausente e manifesto trocado após scan.
- [ ] Symlink/traversal/device file/path excessivo.
- [ ] Dependências ordenadas, ausentes e cíclicas.
- [ ] Corrida de filesystem e cancelamento.
- [ ] Logs sem conteúdo/segredos.

## Done
- [ ] Nenhuma execução de plugin.
- [ ] Plano de carga imutável e auditável.
- [ ] Evidência local no SHA final.
