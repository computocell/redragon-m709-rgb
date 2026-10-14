# Redragon M709 RGB — Linux Driver & GTK4 Control Panel 🎮

[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)
[![GTK4](https://img.shields.io/badge/GUI-GTK4-blue.svg)](https://gtk.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux-lightgrey.svg)](https://kernel.org)

Painel de controle nativo em **Rust** e **GTK4** para o mouse gamer **Redragon M709 (Ranger RGB)** no Linux. Permite configurar iluminação RGB em tempo real, sensibilidade DPI, taxa de atualização USB (polling rate), remapeamento de botões físicos e gerenciamento de perfis de hardware gravados diretamente na memória interna do mouse.

---

## ✨ Funcionalidades

- 💡 **Controle Completo de Iluminação RGB:**
  - 11 modos de luz compatíveis com o microcontrolador Holtek: *Estático*, *Respiração*, *Arco-íris*, *Respiração Arco-íris*, *Onda*, *Piscando*, *Reativo ao Clique*, *Aleatório*, *Alternando*, *Trilha Reativa* e *Desligado*.
  - Paleta de cores rápida e seletor HEX (#RRGGBB).
  - Controle de brilho (1 a 3) e velocidade de animação (1 a 8).
  - Aplicação instantânea ao vivo no mouse sem travamentos.
- ⚡ **DPI & Desempenho:**
  - 5 estágios configuráveis individualmente (200 a 10.000 DPI com snap de 100 em 100).
  - Taxa de atualização (Polling Rate): 125 Hz, 250 Hz, 500 Hz e 1000 Hz.
- 🎯 **Mapeamento de Botões Físicos:**
  - Remapeamento dos 8 botões (Clique Esquerdo, Direito, Meio, Avançar, Voltar, DPI, Scroll Up e Scroll Down).
  - Prevenção de bugs de firmware para proteger a rolagem do mouse.
- 💾 **Perfis de Hardware e Backup:**
  - Alternância rápida entre os 5 perfis da memória onboard do mouse.
  - Exportação e importação de configurações completas em formato TOML.
- 🖥️ **Interface Moderna:**
  - Aplicação nativa GTK4 com tema escuro de alto contraste adaptado para Linux (GNOME, Zorin OS, KDE).
  - Ferramenta CLI (`m709-cli`) para automação por terminal ou scripts de inicialização.

---

## 📦 Estrutura do Workspace

```
.
├── crates/
│   ├── m709-core/    # Codec de protocolo USB, montagem de pacotes e tipos de dados
│   ├── m709-usb/     # Driver USB HID com rusb (Interface 2, VID 04d9 / PID fc2a)
│   ├── m709-cli/     # Interface de linha de comando para terminal
│   └── m709-gui/     # Interface gráfica desktop em GTK4
├── 99-redragon-m709.rules # Regras udev para acesso USB sem sudo
└── Cargo.toml        # Workspace Cargo
```

---

## 🚀 Instalação e Requisitos

### Pré-requisitos (Debian / Ubuntu / Zorin OS / Pop!_OS):

```bash
sudo apt update
sudo apt install -y build-essential pkg-config libusb-1.0-0-dev libgtk-4-dev
```

### Permissões USB (udev):

Para acessar o mouse sem precisar rodar comandos como `sudo`:

```bash
sudo cp 99-redragon-m709.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
```

---

## 🛠️ Como Usar

### 1. Interface Gráfica (GUI)

```bash
cargo run --release -p m709-gui
```

### 2. Linha de Comando (CLI)

```bash
# Detectar dispositivo
cargo run -p m709-cli -- detect

# Ler configurações ativas da memória do mouse
cargo run -p m709-cli -- read

# Trocar perfil de hardware ativo (1 a 5)
cargo run -p m709-cli -- set-profile 1

# Definir iluminação estática vermelha
cargo run -p m709-cli -- set-led --mode static --color "#FF0000" --brightness 3

# Modo arco-íris rápido
cargo run -p m709-cli -- set-led --mode rainbow --speed 8

# Fazer backup do perfil ativo para arquivo TOML
cargo run -p m709-cli -- profile export perfil_gamer.toml

# Restaurar perfil a partir de arquivo TOML
cargo run -p m709-cli -- profile apply perfil_gamer.toml
```

---

## 📜 Licença

Distribuído sob a licença MIT. Veja `LICENSE` para mais informações.
