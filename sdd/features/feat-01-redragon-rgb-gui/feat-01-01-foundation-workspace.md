# feat/01-01-foundation-workspace

**Branch:** `feat/01-redragon-rgb-gui`
**Fase:** 0
**Depende de:** —
**Status:** `done`

## Objetivo
Estruturar o workspace Cargo com crates modulares (`m709-core`, `m709-usb`, `m709-cli`, `m709-gui`), configurar dependências base (`rusb`, `serde`, `toml`, `clap`, `gtk4`), e garantir compilação inicial limpa.

## Critério de conclusão
```bash
# Compilar todo o workspace Cargo sem erros nem warnings críticos
cargo check --workspace
```

## Tarefas
- [x] **01-01-1** Criar `Cargo.toml` raiz configurando workspace com os crates (`m709-core`, `m709-usb`, `m709-cli`, `m709-gui`)
- [x] **01-01-2** Inicializar crate `crates/m709-core` (biblioteca pura, sem dependência de SO)
- [x] **01-01-3** Inicializar crate `crates/m709-usb` com dependência de `rusb` e `m709-core`
- [x] **01-01-4** Inicializar crate `crates/m709-cli` com `clap` para testes e depuração de linha de comando
- [x] **01-01-5** Inicializar crate `crates/m709-gui` preparando ponto de entrada para GTK4
- [x] **01-01-6** Validar compilação com `cargo check --workspace`

## Arquivos gerados
```
Cargo.toml
crates/m709-core/Cargo.toml
crates/m709-core/src/lib.rs
crates/m709-usb/Cargo.toml
crates/m709-usb/src/lib.rs
crates/m709-cli/Cargo.toml
crates/m709-cli/src/main.rs
crates/m709-gui/Cargo.toml
crates/m709-gui/src/main.rs
```

## Skills relevantes
- Builder
- Revisor
