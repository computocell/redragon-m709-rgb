# feat/01-05-gtk4-gui-lighting

**Branch:** `feat/01-redragon-rgb-gui`
**Fase:** 3
**Depende de:** `feat/01-04-profile-persistence`
**Status:** `done`

## Objetivo
Desenvolver a janela principal da aplicação desktop em GTK4 (`gtk4-rs`), implementando a detecção visual de conexão do mouse e a aba de controle de iluminação LED com ColorPicker, modos de luz, sliders de brilho e velocidade, no estilo OpenRGB.

## Critério de conclusão
```bash
# Executar a aplicação desktop e verificar inicialização da interface GTK4
cargo run -p m709-gui
```

## Tarefas
- [x] **01-05-1** Configurar `gtk4-rs` e inicialização de `gtk::Application` com suporte a temas claros/escuros
- [x] **01-05-2** Construir a janela principal:
  - HeaderBar com título, status de conexão (ícone/badge verde se M709 conectado) e botão de atualizar/reconectar
  - Seletor de perfil ativo (1 a 5) em destaque no topo
  - Barra de navegação / abas: "Iluminação", "DPI & Desempenho", "Botões", "Perfis"
- [x] **01-05-3** Implementar a aba **Iluminação**:
  - `GtkColorDialogButton` / Seletor de cor RGB com presets rápidos (Vermelho, Verde, Azul, Ciano, Roxo, Amarelo, Branco)
  - Dropdown com os 10 modos de luz suportados pelo M709
  - Slider de brilho (Níveis 1, 2, 3)
  - Slider de velocidade do efeito (Níveis 1 a 8)
  - Toggle de Ligar/Desligar iluminação
- [x] **01-05-4** Implementar botão "Aplicar no Mouse":
  - Envia comandos USB de forma assíncrona (sem congelar a UI)
  - Toast / notificação visual de sucesso ou erro na barra inferior

## Arquivos gerados
```
crates/m709-gui/src/app.rs
crates/m709-gui/src/window.rs
crates/m709-gui/src/views/lighting.rs
crates/m709-gui/src/widgets/color_picker.rs
crates/m709-gui/src/state.rs
```

## Skills relevantes
- Builder
- Revisor
