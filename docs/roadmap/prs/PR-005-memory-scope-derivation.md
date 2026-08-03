# PR-005 — Derivação do escopo de memória

**Status:** planejada
**Depende de:** PR-004

## Objetivo

Implementar somente a identidade e o caminho determinístico do perfil Hermes, sem criar arquivos e sem alterar o spawn.

## Requisitos
- [ ] `REQ-MEM-501` `HermesMemoryScope` consome `RuntimeScopeContext`.
- [ ] `REQ-MEM-502` Entrada canônica usa versão `buzz-hermes-memory-v1`.
- [ ] `REQ-MEM-503` Hash criptográfico hexadecimal é o único fragmento variável do caminho.
- [ ] `REQ-MEM-504` Caminho fica sob `<app-data>/hermes/profiles/v1/`.
- [ ] `REQ-MEM-505` Display names e strings brutas nunca entram no path.
- [ ] `REQ-MEM-506` Somente aliases Hermes são elegíveis.

## Testes
- [ ] Mesmo contexto gera mesmo hash/path.
- [ ] Relay, canal ou persona diferentes geram paths diferentes.
- [ ] Mudança de label não altera path.
- [ ] Traversal, Unicode, reserved names e strings longas não escapam da raiz.
- [ ] Aliases `hermes`, `hermes-agent`, `hermes-acp` funcionam.
- [ ] Runtime não Hermes retorna `None`/erro tipado conforme contrato.

## Limites
- Nenhuma escrita em disco.
- Nenhuma cópia de configuração.
- Nenhuma variável de ambiente alterada.

## Done
- [ ] Testes determinísticos passam em plataformas suportadas.
- [ ] Algoritmo e formato são documentados e versionados.
- [ ] Evidência local vinculada ao SHA final.
