# PR-013 — Fallback determinístico de modelo

**Status:** planejada
**Depende de:** PR-012

## Objetivo
Executar uma cadeia explícita e finita de modelos alternativos somente para classes de erro recuperáveis.

## Requisitos
- [ ] `REQ-PROV-1301` Cadeia ordenada, persistida e validada sem duplicatas.
- [ ] `REQ-PROV-1302` Timeout, rate limit e indisponibilidade podem avançar conforme política.
- [ ] `REQ-PROV-1303` Autenticação/configuração inválida não avança silenciosamente.
- [ ] `REQ-PROV-1304` Limite de tentativas e detecção de ciclo são obrigatórios.
- [ ] `REQ-PROV-1305` Eventos registram tentativa e motivo sem conteúdo privado.
- [ ] `REQ-PROV-1306` Cancelamento do usuário interrompe toda a cadeia.

## Testes
- [ ] Sucesso primário e em cada fallback.
- [ ] Timeout, rate limit, indisponibilidade, auth e erro fatal.
- [ ] Duplicata, ciclo, cadeia vazia e modelo removido.
- [ ] Cancelamento e deadline global.
- [ ] Preservação de ferramentas/capacidades requeridas.

## Done
- [ ] Sem roteamento automático por heurística.
- [ ] Política e classes de erro documentadas.
- [ ] Evidência local no SHA final.
