# PR-046 — Estratégia de sincronização com upstream

**Status:** planejada
**Depende de:** PR-001, PR-045

## Objetivo
Tornar atualizações de `block/buzz` previsíveis, auditáveis e verificáveis sem perder integrações Hermes.

## Requisitos
- [ ] `REQ-MNT-4601` Upstream remoto, base SHA e intervalo de commits são registrados.
- [ ] `REQ-MNT-4602` Arquivos/áreas de alto risco Hermes possuem owners lógicos e testes obrigatórios.
- [ ] `REQ-MNT-4603` Processo distingue fetch, análise, integração, resolução e validação.
- [ ] `REQ-MNT-4604` Conflitos não são resolvidos automaticamente em arquivos sensíveis.
- [ ] `REQ-MNT-4605` Changelog identifica mudanças upstream e adaptações locais.
- [ ] `REQ-MNT-4606` Verificador de release completo roda após integração.
- [ ] `REQ-MNT-4607` Rollback retorna ao SHA anterior sem apagar dados locais.

## Testes
- [ ] Upstream sem mudanças, avanço simples e histórico divergente.
- [ ] Conflito em arquivo comum e em área de alto risco.
- [ ] Commit upstream removendo/renomeando interface usada.
- [ ] Detecção de testes obrigatórios ausentes.
- [ ] Falha de validação e rollback.
- [ ] Relatório de comparação/changelog.
- [ ] Dados/migrações locais preservados.

## Done
- [ ] Frequência e responsáveis documentados.
- [ ] Nenhum merge automático de arquivo sensível.
- [ ] Evidência local no SHA final.
