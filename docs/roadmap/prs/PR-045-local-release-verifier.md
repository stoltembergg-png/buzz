# PR-045 — Verificador local de release

**Status:** planejada
**Depende de:** PR-003

## Objetivo
Executar em um único comando verificações locais de Rust, TypeScript, Tauri, recursos, versões, checksums e smoke tests.

## Requisitos
- [ ] `REQ-REL-4501` Verificador identifica plataforma, arquitetura, toolchains e SHA.
- [ ] `REQ-REL-4502` Cada etapa tem comando, timeout, exit code e log separado.
- [ ] `REQ-REL-4503` Falha interrompe release, mas relatório preserva etapas executadas.
- [ ] `REQ-REL-4504` Subconjuntos rápidos e validação completa são distintos.
- [ ] `REQ-REL-4505` Artefatos, versões e checksums são conferidos entre manifests.
- [ ] `REQ-REL-4506` Relatório JSON não contém segredos/env completo.
- [ ] `REQ-REL-4507` Nenhum upload ou publicação acontece automaticamente.

## Testes
- [ ] Etapas bem-sucedidas, falha, timeout e ferramenta ausente.
- [ ] Ordem/dependência e skip justificado.
- [ ] Versão divergente e checksum inválido.
- [ ] Relatório parcial após falha.
- [ ] Redaction de env/log.
- [ ] Modos rápido, feature e completo.
- [ ] Smoke test e artefato ausente.

## Done
- [ ] Comandos reais do projeto documentados.
- [ ] Sem dependência de CI hospedada.
- [ ] Evidência do verificador contra seu próprio SHA.
