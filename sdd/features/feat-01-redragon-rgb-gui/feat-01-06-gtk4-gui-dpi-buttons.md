# feat/01-06-gtk4-gui-dpi-buttons

**Branch:** `feat/01-redragon-rgb-gui`
**Fase:** 3
**Depende de:** `feat/01-05-gtk4-gui-lighting`
**Status:** `todo`

## Objetivo
Implementar na interface GTK4 as telas para ajuste de DPI (5 steps configuráveis por perfil), taxa de atualização (Polling Rate) e remapeamento dos 8 botões físicos do Redragon M709.

## Critério de conclusão
```bash
# Executar a aplicação com todas as abas integradas e funcionais
cargo run -p m709-gui
```

## Tarefas
- [ ] **01-06-1** Implementar aba **DPI & Desempenho**:
  - 5 sliders de DPI com valores de 100 a 10.000 DPI (steps múltiplos de 100)
  - Checkboxes para ativar/desativar steps individuais de DPI
  - Indicadores de cor padrão de cada step de DPI
  - Seletor de Polling Rate / Taxa de relatório: 125Hz, 250Hz, 500Hz, 1000Hz
- [ ] **01-06-2** Implementar aba **Mapeamento de Botões**:
  - Layout visual esquemático dos 8 botões físicos do M709:
    1. Botão Esquerdo (`button_left`)
    2. Botão Direito (`button_right`)
    3. Botão do Meio / Scroll Click (`button_middle`)
    4. Botão Avançar lateral (`button_forward`)
    5. Botão Voltar lateral (`button_backward`)
    6. Botão de DPI no topo (`button_dpi`)
    7. Roda Scroll Up (`scroll_up`)
    8. Roda Scroll Down (`scroll_down`)
  - Dropdown por botão para remapeamento (Funções padrão de mouse, atalhos de teclado, controle de mídia, troca de perfil)
- [ ] **01-06-3** Implementar aba **Perfis**:
  - Lista de perfis locais salvos em `~/.config/redragon-mouse/`
  - Botão de Criar Novo Perfil, Duplicar, Renomear e Excluir
  - Botão de "Gravar no Hardware" (envia o perfil selecionado para a memória onboard 1-5 do mouse)
  - Botão de "Restaurar Padrões de Fábrica"
- [ ] **01-06-4** Validação completa de integração ponta a ponta com o mouse M709 físico

## Arquivos gerados
```
crates/m709-gui/src/views/dpi.rs
crates/m709-gui/src/views/buttons.rs
crates/m709-gui/src/views/profiles.rs
crates/m709-gui/src/widgets/button_map_row.rs
crates/m709-gui/src/widgets/dpi_slider.rs
```

## Skills relevantes
- Builder
- Revisor
