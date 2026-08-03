# PR-015 — Medição de contexto

**Status:** planejada
**Depende de:** PR-009

## Objetivo
Medir uso e pressão de contexto antes de introduzir qualquer compressão.

## Requisitos
- [ ] `REQ-CTX-1501` Registrar tokens de entrada, saída, cache e histórico quando disponíveis.
- [ ] `REQ-CTX-1502` Associar limite e margem reservada ao modelo ativo.
- [ ] `REQ-CTX-1503` Suportar estimativa marcada quando o provider não informar tokens.
- [ ] `REQ-CTX-1504` Métricas não incluem conteúdo de mensagens.
- [ ] `REQ-CTX-1505` Valores impossíveis/negativos são rejeitados ou marcados inválidos.

## Testes
- [ ] Provider com métricas completas, parciais e ausentes.
- [ ] Troca de modelo e fallback.
- [ ] Cache read/write e múltiplos turnos.
- [ ] Limite desconhecido e margem inválida.
- [ ] Redaction e serialização.

## Done
- [ ] Nenhuma compressão ou descarte de histórico.
- [ ] Métricas observáveis e documentadas.
- [ ] Evidência local no SHA final.
