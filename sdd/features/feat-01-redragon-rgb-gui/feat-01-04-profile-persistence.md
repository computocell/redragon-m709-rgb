# feat/01-04-profile-persistence

**Branch:** `feat/01-redragon-rgb-gui`
**Fase:** 2
**Depende de:** `feat/01-03-usb-driver-cli`
**Status:** `done`

## Objetivo
Implementar o gerenciador de perfis locais em formato TOML com `serde`, permitindo exportar, importar e salvar presets de iluminação, DPI e botões em `~/.config/redragon-mouse/`.

## Critério de conclusão
```bash
# Executar testes unitários de serialização/deserialização e conversão de perfis
cargo test -p m709-core --test profile_tests
```

## Tarefas
- [x] **01-04-1** Definir esquema TOML limpo e legível para perfis de usuário
- [x] **01-04-2** Implementar conversão bidirecional: `ProfileFile` (TOML) ↔ `MouseSettings` (Hardware)
- [x] **01-04-3** Implementar gerenciador de arquivos (`ProfileManager`):
  - Local padrão: `$XDG_CONFIG_HOME/redragon-mouse/` ou `~/.config/redragon-mouse/`
  - Métodos: `list_profiles()`, `load_profile(name)`, `save_profile(name, settings)`
- [x] **01-04-4** Adicionar comandos de perfis na CLI (`m709-cli profile export/import/apply/backup`)
- [x] **01-04-5** Criar perfis de exemplo padrão (ex: `gaming.toml`, `work.toml`, `rainbow.toml`)

## Arquivos gerados
```
crates/m709-core/src/profile.rs
crates/m709-core/tests/profile_tests.rs
crates/m709-cli/src/commands/profile.rs
```

## Skills relevantes
- Builder
- Revisor
