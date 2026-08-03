# PR-016 — Contrato de compressão

**Status:** planejada
**Depende de:** PR-015

## Objetivo
Definir quando e como o histórico pode ser resumido, sem implementar compressão.

## Requisitos
- [ ] `REQ-CTX-1601` Gatilho considera limite, margem e in-flight tokens.
- [ ] `REQ-CTX-1602` System prompt, instruções vigentes, decisões e tool results críticos têm política de preservação.
- [ ] `REQ-CTX-1603` Resumo possui versão, intervalo de mensagens e hash de origem.
- [ ] `REQ-CTX-1604` Falha mantém histórico bruto.
- [ ] `REQ-CTX-1605` Conteúdo protegido/não resumível é representado explicitamente.

## Testes
- [ ] Gatilhos nos limites inferior, exato e superior.
- [ ] Seleção de conteúdo preservado/descartável.
- [ ] Resumo inválido, versão futura e hash divergente.
- [ ] Falha/cancelamento sem perda de histórico.
- [ ] Compatibilidade com troca de modelo.

## Done
- [ ] Somente contrato, tipos e testes.
- [ ] Nenhum histórico é modificado.
- [ ] Evidência local no SHA final.
