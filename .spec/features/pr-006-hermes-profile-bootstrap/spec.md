# PR-006 — Bootstrap seguro do perfil Hermes

**Status:** done
**Phase:** memory
**Depends on:** PR-005

## Objective

Criar e inicializar o diretório local do escopo Hermes de forma idempotente e fail-closed.

## Requirements

- **REQ-MEM-601** Criar o diretório somente sob a raiz calculada.
- **REQ-MEM-602** Copiar apenas `config.yaml` da home Hermes principal.
- **REQ-MEM-603** Nunca copiar `memories`, `sessions`, `state`, `.env`, caches ou secret stores.
- **REQ-MEM-604** Nunca sobrescrever `config.yaml` existente.
- **REQ-MEM-605** Ausência da configuração principal não é erro.
- **REQ-MEM-606** Symlink e objetos incompatíveis bloqueiam a inicialização.
- **REQ-MEM-607** Inicializações concorrentes preservam o primeiro vencedor.

## Acceptance Criteria

- @spec:AC-MEM-601 Criação e repetição idempotente.
- @spec:AC-MEM-602 Cópia seletiva — apenas `config.yaml`, nunca `.env`/`memories`/`sessions`/`state`.
- @spec:AC-MEM-603 Configuração existente preservada (não sobrescrita).
- @spec:AC-MEM-604 Fonte ausente aceita sem erro.
- @spec:AC-MEM-605 Symlink de raiz recusado.
- @spec:AC-MEM-606 Arquivo onde diretório é esperado recusado.
- @spec:AC-MEM-607 Corrida de inicialização não trunca nem sobrescreve (primeiro vencedor).
- @spec:AC-MEM-608 Erros e logs não contêm conteúdo da configuração copiada.

## Done

- [x] API de filesystem é pequena e testável com diretório temporário (11 testes passando).
- [x] Permissões não são ampliadas intencionalmente (OpenOptions::create_new para copy).
- [x] Nenhum spawn é alterado nesta PR (apenas bootstrap profile dir).
- [x] Evidência local vinculada ao SHA final.
