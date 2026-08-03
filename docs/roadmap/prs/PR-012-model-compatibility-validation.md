# PR-012 — Validação de compatibilidade de modelo

**Status:** planejada
**Depende de:** PR-009, PR-011

## Objetivo
Impedir configurações que exijam capacidades não suportadas pelo modelo selecionado.

## Requisitos
- [ ] `REQ-PROV-1201` Validador puro recebe capacidades e configuração desejada.
- [ ] `REQ-PROV-1202` Reasoning effort, tools, imagens, structured output e contexto são validados.
- [ ] `REQ-PROV-1203` Erros são determinísticos, localizáveis e acionáveis.
- [ ] `REQ-PROV-1204` Capacidade desconhecida não é tratada como suportada.
- [ ] `REQ-PROV-1205` Configurações existentes inválidas entram em estado reparável.

## Testes
- [ ] Matriz positiva e negativa por capacidade.
- [ ] Combinações múltiplas e ordem estável de erros.
- [ ] Campos desconhecidos e catálogo desatualizado.
- [ ] Modelo alterado durante edição.
- [ ] Não regressão do reasoning effort Hermes.

## Done
- [ ] Nenhum fallback ou roteamento automático.
- [ ] UI e runtime usam a mesma função/regra.
- [ ] Evidência local no SHA final.
