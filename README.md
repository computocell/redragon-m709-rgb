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

## 🚀 Manual de Instalação

### Opção 1: Instalação via Pacote `.deb` (Recomendado)

O pacote `.deb` já configura automaticamente os executáveis no sistema (`/usr/bin`), o ícone e atalho no menu de aplicativos (`.desktop`) e as regras de permissão USB (`udev`):

1. Baixe o pacote `redragon-m709-rgb_0.1.0_amd64.deb` da página de [Releases](https://github.com/computocell/redragon-m709-rgb/releases).
2. Instale usando o `apt` no terminal:
   ```bash
   sudo apt install ./redragon-m709-rgb_0.1.0_amd64.deb
   ```
3. Pronto! Abra o aplicativo procurando por **Redragon M709 RGB** no menu de programas ou execute `m709-gui` no terminal.

---

### Opção 2: Compilação a partir do Código-Fonte (Rust / Cargo)

Caso queira compilar manualmente no Linux:

1. Instale as dependências de compilação (Debian, Ubuntu, Zorin OS, Pop!_OS, Linux Mint):
   ```bash
   sudo apt update
   sudo apt install -y build-essential pkg-config libusb-1.0-0-dev libgtk-4-dev
   ```

2. Instale o Rust (se ainda não possuir):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source "$HOME/.cargo/env"
   ```

3. Configure as permissões USB do mouse:
   ```bash
   sudo cp 99-redragon-m709.rules /etc/udev/rules.d/
   sudo udevadm control --reload-rules && sudo udevadm trigger
   ```

4. Compile os binários otimizados:
   ```bash
   cargo build --release
   ```

---

### 📦 Como Gerar o Pacote `.deb` Localmente

Para empacotar uma nova versão do `.deb` em sua máquina:

```bash
./packaging/build-deb.sh
```

O arquivo gerado `redragon-m709-rgb_0.1.0_amd64.deb` estará pronto na raiz do projeto.

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
