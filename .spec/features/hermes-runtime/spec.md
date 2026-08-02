# Spec: Hermes runtime

> feature: hermes-runtime
> status: auditada

## Contexto

O Buzz já inicia o Hermes por ACP, mas ainda trata Hermes como um preset opaco. Esta fatia promove o runtime para o catálogo compilado e permite ler apenas o provider/model não secreto de `config.yaml`, sem duplicar adapters nem ler credenciais.

## Histórias

### US-001 — Hermes como runtime ACP conhecido

Como operador do Buzz, quero escolher Hermes como runtime ACP conhecido, para que `hermes-acp`/`hermes` receba a mesma descoberta, configuração e normalização que os demais runtimes.

#### AC-001 — Identidade Hermes resolve todos os entrypoints

- **Dado** o catálogo compilado do Desktop
- **Quando** o comando recebido for `hermes-acp`, `hermes` ou `hermes-agent`
- **Então** todos devem resolver para o runtime ID `hermes`, preservando a identidade usada por personas existentes

#### AC-002 — Metadata declara o contrato ACP do Hermes

- **Dado** o runtime `hermes`
- **Quando** o Buzz construir seu metadata
- **Então** deve declarar ACP model switching, `~/.hermes/config.yaml` em YAML, ausência de provider/model env var inventada e `HERMES_ACP_SKIP_CONFIGURED_MCP=1` como default de host

#### AC-006 — Não há preset duplicado e o fallback CLI usa ACP

- **Dado** o catálogo de runtimes
- **Quando** Hermes for promovido a builtin e o fallback `hermes`/`hermes-agent` for usado sem args
- **Então** Hermes não deve continuar em `PRESET_HARNESSES` e o ACP deve receber o subcomando `acp`

### US-002 — Configuração inicial não secreta

Como operador do Buzz, quero ver o provider/model padrão do Hermes, para configurar o agente sem o Buzz precisar reimplementar os adapters do Hermes.

#### AC-003 — Configuração extrai provider/model com tolerância

- **Dado** um `config.yaml` Hermes válido
- **Quando** o Buzz ler o arquivo
- **Então** deve extrair `model.provider` e `model.default` (ou `model.model` legado), aceitar model escalar e não transformar valores ausentes em erro de descoberta

#### AC-004 — Configuração respeita HERMES_HOME

- **Dado** `HERMES_HOME` definido
- **Quando** o Buzz resolver o arquivo
- **Então** deve usar `$HERMES_HOME/config.yaml`; sem a variável, deve usar `~/.hermes/config.yaml`

### US-003 — Credenciais permanecem fora do Buzz

Como administrador de uma instalação multi-tenant, quero que o Buzz leia apenas configuração não secreta, para que tokens e OAuth nunca sejam exibidos ou publicados em eventos.

#### AC-005 — Credenciais não aparecem na configuração

- **Dado** um YAML com campos que parecem API key, access token ou refresh token
- **Quando** o Buzz construir `RuntimeFileConfig`
- **Então** esses valores não devem aparecer em `extra`, no debug/serialização do resultado ou em qualquer superfície normalizada

## Fora de escopo

- Implementar providers Hermes em Rust.
- Ler ou migrar `.env`, tokens OAuth ou secret stores.
- Criar isolamento/per-agent `HERMES_HOME`; isso será uma fase posterior de credential scoping.
- Implementar provider catalog/fallback completo no UI; isso será a próxima fatia após este contrato.

## Suposições

| ID | Suposição | Status | Resolução |
|---|---|---|---|
| ASM-001 | O Hermes ACP resolve provider/model via sua sessão ACP (`session/new`/`session/set_model`), portanto o Buzz não inventa `HERMES_PROVIDER`. | confirmada | Confirmada pela implementação ACP analisada. |
| ASM-002 | As credenciais e OAuth do Hermes permanecem sob controle do Hermes; o bridge só lê `config.yaml` e ignora `.env`/stores de segredo. | confirmada | Confirmada pelo escopo deste slice e pelos testes de redaction. |
| ASM-003 | O `buzz-acp` e os sidecars nativos do Desktop são preparados pelo build normal do Buzz antes da execução da suíte Tauri. | confirmada | Sidecars reais foram construídos para a verificação local. |

## Perguntas em aberto

Nenhuma.
