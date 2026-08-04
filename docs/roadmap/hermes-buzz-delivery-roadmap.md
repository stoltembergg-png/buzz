# Hermes + Buzz Delivery Roadmap

## Regras globais de entrega

- Cada capacidade é entregue por uma Pull Request pequena e focada.
- Todo requisito recebe um identificador estável `REQ-XXX-NNN`.
- Todo critério de aceitação recebe um identificador `AC-XXX-NNN` e deve apontar para teste ou evidência.
- Nenhuma PR pode ser marcada como pronta enquanto houver checkbox pendente.
- CI hospedada não é presumida; a evidência local registra comandos, versões, plataforma e SHA.
- Código de produção segue TDD quando houver ambiente executável.
- Toda PR documenta rollback, segurança, compatibilidade, migração e não objetivos.
- Uma PR não pode enfraquecer, ignorar ou apagar testes sem justificativa explícita.

## Sequência planejada

### PR-001 — Roadmap e Definition of Done
- **Fase:** Governança
- **Depende de:** nenhuma
- **Objetivo:** criar roadmap mestre, convenção de IDs e Definition of Done global.
- **Validação:** todos os itens possuem dependências, critérios de saída e evidência esperada.

### PR-002 — Templates de PR e feature spec
- **Fase:** Governança
- **Depende de:** PR-001
- **Objetivo:** adicionar templates obrigatórios para PR, especificação, tarefas e evidências.
- **Validação:** testes de contrato para campos obrigatórios e ausência de placeholders.

### PR-003 — Verificador local de feature
- **Fase:** Governança
- **Depende de:** PR-001, PR-002
- **Objetivo:** relacionar requisitos, testes e evidências sem CI hospedada.
- **Validação:** feature inexistente, requisito sem teste, teste sem requisito, tarefa sem evidência e execução bem-sucedida.

### PR-004 — Contexto estável de runtime
- **Fase:** Memória
- **Depende de:** PR-003
- **Objetivo:** propagar relay/comunidade, canal, persona e runtime até o spawn.
- **Validação:** todos os entrypoints produzem a mesma identidade canônica.

### PR-005 — Derivação do escopo de memória
- **Fase:** Memória
- **Depende de:** PR-004
- **Objetivo:** implementar `HermesMemoryScope`, serialização canônica, hash versionado e caminho sem tocar no filesystem.
- **Validação:** determinismo, isolamento, aliases, caracteres maliciosos e independência de display name.

### PR-006 — Bootstrap seguro do perfil Hermes
- **Fase:** Memória
- **Depende de:** PR-005
- **Objetivo:** criar diretório idempotente e copiar apenas `config.yaml` com proteção contra symlink e overwrite.
- **Validação:** idempotência, concorrência, symlink, conflitos e cópia seletiva.

### PR-007 — Injeção de HERMES_HOME
- **Fase:** Memória
- **Depende de:** PR-006
- **Objetivo:** injetar perfil local no Hermes sem alterar outros runtimes.
- **Validação:** aliases, restart, isolamento por persona/canal e não regressão.

### PR-008 — Harness funcional de persistência
- **Fase:** Memória
- **Depende de:** PR-007
- **Objetivo:** gravar e recuperar memória após restart e comprovar isolamento.
- **Validação:** smoke test local com evidência JSON.

### PR-009 — Contrato de capacidades de provider
- **Fase:** Providers
- **Depende de:** PR-003
- **Objetivo:** definir tipos internos para providers, modelos e capacidades.
- **Validação:** serialização, compatibilidade e campos desconhecidos.

### PR-010 — Descoberta read-only de providers
- **Fase:** Providers
- **Depende de:** PR-009
- **Objetivo:** consultar catálogo Hermes sem alterar configuração.
- **Validação:** catálogo vazio, dados parciais, indisponibilidade e redaction.

### PR-011 — Seleção e persistência de provider/modelo
- **Fase:** Providers
- **Depende de:** PR-010
- **Objetivo:** escolher provider/modelo por persona e persistir localmente.
- **Validação:** round-trip, restart, valores inválidos e isolamento.

### PR-012 — Validação de compatibilidade de modelo
- **Fase:** Providers
- **Depende de:** PR-009, PR-011
- **Objetivo:** bloquear combinações incompatíveis de modelo, reasoning effort e ferramentas.
- **Validação:** matriz positiva/negativa e erros determinísticos.

### PR-013 — Fallback determinístico de modelo
- **Fase:** Providers
- **Depende de:** PR-012
- **Objetivo:** executar cadeia explícita de fallback sem loops.
- **Validação:** timeout, rate limit, autenticação, indisponibilidade, erro fatal e loop.

### PR-014 — Roteamento automático opt-in
- **Fase:** Providers
- **Depende de:** PR-013
- **Objetivo:** escolher modelo por capacidade e orçamento com justificativa observável.
- **Validação:** política, orçamento, preferência, empate e desativação.

### PR-015 — Medição de contexto
- **Fase:** Contexto
- **Depende de:** PR-009
- **Objetivo:** medir tokens, histórico, limite do modelo e margem reservada.
- **Validação:** limites, ausência de métricas e múltiplos providers.

### PR-016 — Contrato de compressão
- **Fase:** Contexto
- **Depende de:** PR-015
- **Objetivo:** definir gatilhos, conteúdo preservado, formato e versionamento.
- **Validação:** contrato, versionamento e compatibilidade.

### PR-017 — Compressão por sessão
- **Fase:** Contexto
- **Depende de:** PR-016
- **Objetivo:** comprimir somente a sessão corrente.
- **Validação:** gatilho, preservação, falha e fallback para histórico bruto.

### PR-018 — Compressão persistente por escopo
- **Fase:** Contexto
- **Depende de:** PR-017, PR-007
- **Objetivo:** persistir resumos por persona/canal.
- **Validação:** restart, corrupção, versão incompatível e rollback.

### PR-019 — Descoberta de skills
- **Fase:** Skills
- **Depende de:** PR-003
- **Objetivo:** listar skills sem executá-las.
- **Validação:** diretório ausente, manifesto inválido, duplicidade e redaction.

### PR-020 — Escopo de skills
- **Fase:** Skills
- **Depende de:** PR-019, PR-004
- **Objetivo:** definir skills globais, por persona e por canal.
- **Validação:** precedência, isolamento e conflito.

### PR-021 — Ativação e carregamento de skills
- **Fase:** Skills
- **Depende de:** PR-020
- **Objetivo:** carregar apenas skills habilitadas e compatíveis.
- **Validação:** versão, dependência, arquivo malformado e diretório inseguro.

### PR-022 — Interface de gerenciamento de skills
- **Fase:** Skills
- **Depende de:** PR-021
- **Objetivo:** visualizar e ativar skills, sem editor arbitrário.
- **Validação:** estado, erro, acessibilidade e persistência.

### PR-023 — Catálogo de MCPs
- **Fase:** MCP
- **Depende de:** PR-003
- **Objetivo:** descobrir servidores, transportes e capacidades.
- **Validação:** stdio, HTTP, inválido, duplicado e indisponível.

### PR-024 — Configuração MCP por persona/canal
- **Fase:** MCP
- **Depende de:** PR-023, PR-004
- **Objetivo:** aplicar MCPs por escopo sem vazamento entre agentes.
- **Validação:** isolamento, restart e precedência.

### PR-025 — Permissões MCP
- **Fase:** MCP
- **Depende de:** PR-024
- **Objetivo:** definir permissões para filesystem, rede, execução e credenciais.
- **Validação:** allow/deny, default deny e escalonamento bloqueado.

### PR-026 — Lifecycle e health checks MCP
- **Fase:** MCP
- **Depende de:** PR-025
- **Objetivo:** controlar startup, timeout, crash, restart e shutdown.
- **Validação:** processos falsos, limites de retry e limpeza.

### PR-027 — Interface de MCP
- **Fase:** MCP
- **Depende de:** PR-026
- **Objetivo:** exibir estado, capacidades, erros e permissões.
- **Validação:** acessibilidade, loading, erro e atualização.

### PR-028 — Manifesto de plugin
- **Fase:** Plugins
- **Depende de:** PR-003
- **Objetivo:** definir ID, versão, entrypoint, permissões, compatibilidade e integridade.
- **Validação:** schema, hash e campos desconhecidos.

### PR-029 — Loader local seguro de plugins
- **Fase:** Plugins
- **Depende de:** PR-028
- **Objetivo:** carregar plugins locais declarativos sem marketplace.
- **Validação:** traversal, symlink, hash inválido e entrypoint ausente.

### PR-030 — Lifecycle de plugins
- **Fase:** Plugins
- **Depende de:** PR-029
- **Objetivo:** instalar, ativar, desativar, atualizar e remover.
- **Validação:** transações, rollback, versão e estado corrompido.

### PR-031 — Sandbox e permissões de plugins
- **Fase:** Plugins
- **Depende de:** PR-030
- **Objetivo:** restringir filesystem, rede, ferramentas e segredos.
- **Validação:** negação, isolamento e tentativa de escalada.

### PR-032 — SDK mínimo de plugins
- **Fase:** Plugins
- **Depende de:** PR-031
- **Objetivo:** expor interfaces estáveis para ferramentas e hooks permitidos.
- **Validação:** compatibilidade, versionamento e capability errors.

### PR-033 — Eventos de execução
- **Fase:** Observabilidade
- **Depende de:** PR-003
- **Objetivo:** normalizar eventos de provider, ferramentas, MCP, memória, compressão e erros.
- **Validação:** schema, redaction, ordem e correlação.

### PR-034 — Painel de execução
- **Fase:** Observabilidade
- **Depende de:** PR-033
- **Objetivo:** mostrar traces sem prompts privados ou credenciais.
- **Validação:** redaction, paginação, filtros e acessibilidade.

### PR-035 — Harness de avaliação
- **Fase:** Avaliação
- **Depende de:** PR-033
- **Objetivo:** executar cenários repetíveis e produzir resultados comparáveis.
- **Validação:** determinismo, fixtures versionadas e relatório.

### PR-036 — Regressão comportamental
- **Fase:** Avaliação
- **Depende de:** PR-035
- **Objetivo:** detectar pioras silenciosas com tarefas de referência.
- **Validação:** baseline, tolerâncias e comparação reproduzível.

### PR-037 — Registro de feedback
- **Fase:** Autoaperfeiçoamento
- **Depende de:** PR-033, PR-004
- **Objetivo:** armazenar resultados, correções e falhas localmente.
- **Validação:** isolamento, retenção, corrupção e redaction.

### PR-038 — Geração de propostas de melhoria
- **Fase:** Autoaperfeiçoamento
- **Depende de:** PR-037
- **Objetivo:** permitir propostas sem aplicação automática.
- **Validação:** formato, justificativa, limites e conteúdo proibido.

### PR-039 — Revisão e aprovação humana
- **Fase:** Autoaperfeiçoamento
- **Depende de:** PR-038
- **Objetivo:** exibir diff, justificativa, testes e rollback.
- **Validação:** aprovação, rejeição, edição e auditoria.

### PR-040 — Aplicação versionada de melhorias
- **Fase:** Autoaperfeiçoamento
- **Depende de:** PR-039
- **Objetivo:** aplicar apenas propostas aprovadas com snapshot e rollback.
- **Validação:** transação, falha parcial, rollback e concorrência.

### PR-041 — Autoaperfeiçoamento opt-in
- **Fase:** Autoaperfeiçoamento
- **Depende de:** PR-040
- **Objetivo:** automatizar dentro de limites de orçamento, frequência e segurança.
- **Validação:** limites, desativação, rollback automático e proibições.

### PR-042 — Delegação temporária de credenciais
- **Fase:** Segurança
- **Depende de:** PR-003
- **Objetivo:** remover chave privada persistente do ambiente.
- **Validação:** expiração, escopo, revogação, replay e não vazamento.

### PR-043 — Backup e exportação local
- **Fase:** Dados
- **Depende de:** PR-018, PR-040
- **Objetivo:** exportar memória, configuração e snapshots sem segredos por padrão.
- **Validação:** inclusão/exclusão, corrupção e restore dry-run.

### PR-044 — Migração e versionamento de perfis
- **Fase:** Dados
- **Depende de:** PR-018, PR-043
- **Objetivo:** controlar migrações de profiles, compressão e configuração.
- **Validação:** upgrade, downgrade recusado, idempotência e rollback.

### PR-045 — Verificador local de release
- **Fase:** Release
- **Depende de:** PR-003
- **Objetivo:** validar Rust, TypeScript, Tauri, recursos e smoke tests em um comando.
- **Validação:** testes do orquestrador e relatório de evidências.

### PR-046 — Estratégia de sincronização com upstream
- **Fase:** Manutenção
- **Depende de:** PR-001, PR-045
- **Objetivo:** documentar e automatizar atualização com `block/buzz`.
- **Validação:** detecção de conflitos, arquivos de risco e checklist pós-merge.

## Definition of Done global

- [ ] Escopo e não objetivos estão explícitos.
- [ ] Requisitos e critérios de aceitação têm IDs estáveis.
- [ ] Testes positivos, negativos, de regressão e integração existem quando aplicável.
- [ ] Comandos e resultados foram registrados contra o SHA final.
- [ ] Nenhum teste foi ignorado, enfraquecido ou removido sem justificativa.
- [ ] Entradas controladas pelo usuário, segredos e trust boundaries foram revisados.
- [ ] Compatibilidade, migração e rollback estão documentados.
- [ ] O diff final foi revisado depois da última alteração.
- [ ] A PR é reversível de forma independente.
- [ ] A documentação corresponde ao comportamento real.
