# PR-006 — Bootstrap seguro do perfil Hermes

**Status:** planejada
**Depende de:** PR-005

## Objetivo

Criar e inicializar o diretório local do escopo Hermes de forma idempotente e fail-closed.

## Requisitos
- [ ] `REQ-MEM-601` Criar o diretório somente sob a raiz calculada.
- [ ] `REQ-MEM-602` Copiar apenas `config.yaml` da home Hermes principal.
- [ ] `REQ-MEM-603` Nunca copiar `memories`, `sessions`, `state`, `.env`, caches ou secret stores.
- [ ] `REQ-MEM-604` Nunca sobrescrever `config.yaml` existente.
- [ ] `REQ-MEM-605` Ausência da configuração principal não é erro.
- [ ] `REQ-MEM-606` Symlink e objetos incompatíveis bloqueiam a inicialização.
- [ ] `REQ-MEM-607` Inicializações concorrentes preservam o primeiro vencedor.

## Testes
- [ ] Criação e repetição idempotente.
- [ ] Cópia seletiva.
- [ ] Configuração existente preservada.
- [ ] Fonte ausente aceita.
- [ ] Symlink de raiz, diretório e destino recusado.
- [ ] Arquivo onde diretório é esperado recusado.
- [ ] Corrida de inicialização não trunca nem sobrescreve.
- [ ] Erros e logs não contêm conteúdo da configuração.

## Done
- [ ] API de filesystem é pequena e testável com diretório temporário.
- [ ] Permissões não são ampliadas intencionalmente.
- [ ] Nenhum spawn é alterado nesta PR.
- [ ] Evidência local vinculada ao SHA final.
