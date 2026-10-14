# Plano — Discovery 01: Redragon RGB GUI (M709)

## Roadmap por fases

### Fase 0 — Foundation & Ambiente
> Setup de Git, toolchain Rust, dependências de sistema (GTK4/libusb) e estrutura do projeto Cargo.

- Inicializar repositório Git (`git init`) e criar `.gitignore` adequado
- Instalar toolchain Rust (`rustup` / `cargo`) e `libgtk-4-dev` se necessário
- Criar workspace Cargo com arquitetura modular:
  - `crates/m709-core` (protocolo, tipos, serialização de pacotes)
  - `crates/m709-usb` (comunicação USB via `rusb`)
  - `crates/m709-cli` (ferramenta CLI para testes diretos)
  - `crates/m709-gui` (interface GTK4)
- Commit inicial na `main` e criação da branch de desenvolvimento

### Fase 1 — Engenharia Reversa (REA) & Driver USB
> Extrair e codificar a especificação exata do protocolo USB HID do M709 em Rust.

- **REA:** Analisar `../mouse_m908/include/m709/` (`writers.cpp`, `readers.cpp`, `data.cpp`, `mouse_m709.h`)
- Mapear tabelas de constantes e estrutura de relatórios:
  - `_c_data_s_profile` (seleção de perfil 1-5)
  - `_c_data_settings_1`, `_c_data_settings_2`, `_c_data_settings_3` (escrita de configurações completas)
  - `_c_data_read_*` (leitura de configurações do hardware)
- Implementar crate `m709-core` (tipos: `LightMode`, `Color`, `DpiLevel`, `ButtonMapping`, `ReportRate`)
- Implementar crate `m709-usb` (`M709Device::open()`, `write_profile()`, `write_settings()`, `read_settings()`)
- Testar via `m709-cli detect` e `m709-cli set-led --color ff0000 --mode static`

### Fase 2 — Persistência & Gerenciador de Perfis
> Salvar e exportar configurações em TOML localmente.

- Crate `m709-profile`: serialização serde para `~/.config/redragon-mouse/*.toml`
- Conversão bidirecional: `Profile` local ↔ Pacotes de hardware M709

### Fase 3 — Interface Gráfica GTK4 (OpenRGB Style)
> Desenvolvimento da GUI desktop moderna com GTK4 / Libadwaita.

- Janela principal com sidebar de navegação e status do mouse (Conectado / Desconectado)
- **Aba Iluminação:**
  - Color Picker interativo com presets rápidos
  - Seletor de modos (Estático, Respiração, Arco-íris, Onda, etc.)
  - Sliders de brilho e velocidade do efeito
- **Aba Desempenho / DPI:**
  - 5 sliders de DPI com indicação visual de cor/step
  - Seletor de Polling Rate (125Hz, 250Hz, 500Hz, 1000Hz)
- **Aba Botões:**
  - Mapeamento dos 8 botões físicos do M709 com dropdowns
- Botões de ação globais: "Aplicar no Mouse", "Salvar Perfil", "Recarregar do Mouse"

### Fase 4 — Polimento & Empacotamento
> Feedback visual, atalhos, tray icon e empacotamento Linux.

- Preview visual esquemático do mouse com a cor selecionada
- Ícone de bandeja (system tray) para troca rápida de perfis
- Empacotamento (.deb / Flatpak / AppImage)

## Dependências de Sistema & Crates

| Dependência | Versão / Tipo | Finalidade |
|-------------|---------------|------------|
| `rustc` / `cargo` | stable | Compilador e gerenciador Rust |
| `libusb-1.0-dev` | sistema (instalado) | Header C para acesso USB |
| `libgtk-4-dev` | sistema (apt) | Headers GTK4 para GUI nativa |
| `rusb` | crate | Bindings seguros de libusb |
| `gtk4` (`gtk4-rs`) | crate | Interface gráfica desktop GTK4 |
| `serde` + `toml` | crate | Serialização de perfis |

## Handoff → `/split-features`

Artefatos atualizados e prontos para decomposição em features pelo comando `/split-features`.
