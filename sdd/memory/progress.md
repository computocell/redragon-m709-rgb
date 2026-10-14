# Progress — redragon-mouse

## Status
```
Fase 0 — Foundation   [x] done
Fase 1 — USB Protocol [x] done
Fase 2 — Profiles     [ ] todo
```

## Features ativas
| Feature | Branch | Status |
|---------|--------|--------|
| feat-01-04-profile-persistence | feat/01-redragon-rgb-gui | todo |

## Próximo passo
**Iniciar:** feat-01-04-profile-persistence (Persistência TOML e import/export de perfis)
**Bloqueios:** —

## Handoff da última sessão
- Protocolo M709 codificado em `m709-core` com serialização e parser testados.
- Driver USB `m709-usb` e CLI `m709-cli` testados e validados no hardware real do usuário:
  - Leitura completa de estado: `m709 read`
  - Gravação de LED: `m709 set-led --color "#00FF88" --mode breathing` confirmada pelo hardware.
- `feat-01-02` e `feat-01-03` finalizadas com sucesso.

## Última sessão
> Histórico completo em `progress-log.md`
