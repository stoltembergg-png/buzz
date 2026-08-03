# PR-017 — Compressão por sessão

**Status:** planejada
**Depende de:** PR-016

## Objetivo
Aplicar o contrato de compressão somente à sessão corrente, preservando fallback seguro para histórico bruto.

## Requisitos
- [ ] `REQ-CTX-1701` Compressão ocorre apenas após gatilho válido.
- [ ] `REQ-CTX-1702` Resumo é validado antes de substituir a janela ativa.
- [ ] `REQ-CTX-1703` Falha, timeout ou cancelamento mantém histórico bruto.
- [ ] `REQ-CTX-1704` Tool calls e respostas críticas mantêm relação causal.
- [ ] `REQ-CTX-1705` Eventos registram tamanho, versão e motivo sem conteúdo privado.

## Testes
- [ ] Abaixo/no/acima do limiar.
- [ ] Resumo válido e inválido.
- [ ] Timeout, cancelamento e erro de provider.
- [ ] Tool call/result, anexos e mensagens protegidas.
- [ ] Múltiplas compressões na mesma sessão.
- [ ] Troca de modelo após compressão.

## Done
- [ ] Nenhuma persistência entre sessões.
- [ ] Histórico bruto continua disponível para recuperação.
- [ ] Evidência local no SHA final.
