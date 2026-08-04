# PR-009 — Provider Capability Contract

**Status:** implemented
**Depende de:** PR-003 (runtime scope)

## Requisitos
- [x] REQ-PROV-901: Identidade estável de provider/modelo separada de label.
- [x] REQ-PROV-902: Capacidades incluem contexto, reasoning, tools, imagens, structured output, custo e disponibilidade.
- [x] REQ-PROV-903: Campos desconhecidos são tolerados conforme regra documentada.
- [x] REQ-PROV-904: Nenhum segredo faz parte do contrato.
- [x] REQ-PROV-905: Rust e TypeScript compartilham semântica equivalente.

## Acceptance Criteria
- @spec:AC-PROV-901: ProviderId normaliza para lowercase kebab-case e valida
- @spec:AC-PROV-902: ModelId preserva case original e valida não-vazio
- @spec:AC-PROV-903: ModelCapabilities serializa/deserializa round-trip
- @spec:AC-PROV-904: Campos desconhecidos no top-level do catalog são preservados em extra
- @spec:AC-PROV-905: Campos desconhecidos em ModelEntry são silenciosamente descartados
- @spec:AC-PROV-906: assert_no_secrets_in_json detecta campos secretos no top-level
- @spec:AC-PROV-907: assert_no_secrets_in_json detecta campos secretos aninhados
- @spec:AC-PROV-908: ProviderCatalog round-trip completo preserva todos os dados
