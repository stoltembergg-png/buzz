# PR-025 — Permissões MCP

**Status:** planejada
**Depende de:** PR-024

## Objetivo
Aplicar política explícita e default-deny para filesystem, rede, execução, ferramentas e credenciais de cada MCP.

## Requisitos
- [ ] `REQ-MCP-2501` Toda capability sensível exige decisão explícita.
- [ ] `REQ-MCP-2502` Ausência de regra significa negar.
- [ ] `REQ-MCP-2503` Escopo de filesystem e rede é mínimo e normalizado.
- [ ] `REQ-MCP-2504` Credenciais são referenciadas por capability, nunca serializadas inline.
- [ ] `REQ-MCP-2505` Alteração de permissão exige reinício/revalidação quando aplicável.
- [ ] `REQ-MCP-2506` Tentativas negadas geram evento redigido.

## Testes
- [ ] Allow/deny/default-deny por capability.
- [ ] Path traversal, symlink e host/porta fora do escopo.
- [ ] Execução não declarada e tentativa de escalada.
- [ ] Credencial ausente, expirada e fora do escopo.
- [ ] Alteração durante sessão e restart.
- [ ] Logs sem segredo ou conteúdo de arquivos.

## Done
- [ ] Trust boundaries documentadas.
- [ ] Política compartilhada entre UI e runtime.
- [ ] Evidência local no SHA final.
