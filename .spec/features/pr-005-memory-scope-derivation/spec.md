# PR-005 — Derivação do escopo de memória

**Status:** done
**Phase:** memory
**Depends on:** PR-004

## Objective

Implementar `HermesMemoryScope`, serialização canônica, hash criptográfico
versionado (SHA-256) e caminho determinístico — sem tocar no filesystem.

## Requirements

- **REQ-MEM-501** `HermesMemoryScope` consome `RuntimeScopeContext`.
- **REQ-MEM-502** Entrada canônica usa versão `buzz-hermes-memory-v1`.
- **REQ-MEM-503** Hash criptográfico hexadecimal (SHA-256) é o único fragmento variável do caminho.
- **REQ-MEM-504** Caminho fica sob `<app-data>/hermes/profiles/v1/`.
- **REQ-MEM-505** Display names e strings brutas nunca entram no path.
- **REQ-MEM-506** Somente aliases Hermes são elegíveis.

## Acceptance Criteria

- @spec:AC-MEM-501 Mesmo contexto gera mesmo hash/path (determinismo).
- @spec:AC-MEM-502 Relay, canal ou persona diferentes geram paths diferentes.
- @spec:AC-MEM-503 Display names não afetam o hash (REQ-MEM-505).
- @spec:AC-MEM-504 Path traversal, Unicode, reserved names e strings longas não escapam da raiz.
- @spec:AC-MEM-505 Aliases `hermes`, `hermes-agent`, `hermes-acp` resolvem para o mesmo scope.
- @spec:AC-MEM-506 Runtime não-Hermes (`BuzzAcp`) retorna erro tipado `NotHermesRuntime`.
- @spec:AC-MEM-507 Hash tem exatamente 64 caracteres hex lowercase.
- @spec:AC-MEM-508 `within()` joga o path sob a raiz fornecida sem criar diretórios.

## Limits

- Nenhuma escrita em disco.
- Nenhuma cópia de configuração.
- Nenhuma variável de ambiente alterada.

## Done

- [x] Testes determinísticos passam (16 testes unitários Rust).
- [x] Algoritmo (SHA-256) e formato (`buzz-hermes-memory-v1`) são documentados e versionados.
- [x] Evidência local vinculada ao SHA final.
