# Discovery 01 — Redragon RGB GUI

## Porquê

O Redragon M709 não possui software de configuração oficial para Linux. O projeto open-source **mouse_m908** (dokutan/mouse_m908) implementou suporte via CLI em C++, mas a experiência é baseada em arquivos `.ini` e linha de comando. Este projeto visa criar uma **GUI desktop nativa em Rust (GTK4)** — inspirada no OpenRGB — que ofereça uma interface visual moderna e fluida para controlar LEDs, perfis, DPI e mapeamento dos 8 botões do M709.

## Para quem

- Usuários Linux que possuem o Redragon M709 e querem configurar o mouse nativamente.
- Comunidade gamer Linux que busca alternativas gráficas e leves ao estilo OpenRGB.

## Como (macro)

### Abordagem REA (Reverse Engineer Anything)

O protocolo USB HID do M709 está implementado no projeto local `../mouse_m908/include/m709/` em C++. Usaremos o **REA** (e análise estática/dinâmica) para:

1. **Analisar a implementação C++ do M709** — mapear os pacotes de escrita (`_c_data_settings_1`, `_c_data_settings_2`, `_c_data_settings_3`, `_c_data_s_profile`) e leitura (`_c_data_read_*`).
2. **Documentar a estrutura de pacotes USB HID** — control transfers, feature reports e buffers.
3. **Reimplementar em Rust puro** usando `rusb` (bindings seguros para libusb) e arquitetura modular.
4. **Construir a interface gráfica** com `gtk4-rs`, trazendo color picker, seleção de modos de luz, sliders de brilho/velocidade, steps de DPI e mapeamento de botões.

### Stack

| Camada         | Tecnologia                  | Motivo                                              |
|----------------|-----------------------------|-----------------------------------------------------|
| Linguagem      | Rust                        | Performance nativa, memory safety, sem runtime/GC   |
| GUI Framework  | GTK4 (gtk4-rs)              | Interface gráfica moderna, nativa no Linux/GNOME    |
| USB            | rusb (libusb-1.0)           | Acesso robusto ao dispositivo USB HID               |
| RE             | REA (rea-agents)            | Engenharia reversa e extração do protocolo C++      |
| Config / Perfis| TOML (serde)                | Formato limpo e legível para perfis de usuário      |

### Funcionalidades alvo (Redragon M709)

| Funcionalidade                          | Prioridade | Referência M709 (`mouse_m908`) |
|-----------------------------------------|------------|--------------------------------|
| Detecção automática do M709 (VID:PID)   | P0         | `0x04d9:0xfc2a`                |
| Controle de LEDs (cor, modo, brilho, velocidade) | P0 | 10 modos de iluminação |
| Gerenciamento de perfis (1 a 5)         | P0         | 5 perfis onboard na memória    |
| Configuração de DPI (5 steps por perfil)| P1         | 5 níveis ajustáveis            |
| Mapeamento dos 8 botões físicos         | P1         | 8 botões configuráveis         |
| Leitura do estado atual do mouse        | P1         | Dump/read settings via USB     |
| Preview visual interativo               | P2         | Inspiração OpenRGB             |

### Referências técnicas do M709

- **Hardware:** Redragon M709 (VID: `0x04D9`, PID: `0xFC2A`)
- **Botões físicos (8):** `button_left`, `button_right`, `button_middle`, `button_forward`, `button_backward`, `button_dpi`, `scroll_up`, `scroll_down`
- **Modos de LED:** `static`, `breathing`, `breathing_rainbow`, `rainbow`, `wave`, `alternating`, `reactive`, `flashing`, `random`, `off`
- **Brilho:** 3 níveis (1-3)
- **Velocidade:** 8 níveis (1-8)
- **Polling Rates:** 125Hz, 250Hz, 500Hz, 1000Hz
- **Fonte de referência:** `../mouse_m908/include/m709/`

### Clarify — decisões do usuário

| Pergunta | Resposta |
|----------|---------|
| Tipo de interface | GUI Desktop (GTK4 via gtk4-rs) |
| Linguagem | Rust |
| Funcionalidades | Controle completo: LEDs, perfis 1-5, DPI, botões e detecção automática |
| Uso do REA | Engenharia reversa do protocolo C++ do m908 e reimplementação em Rust |
| Dispositivo alvo | Redragon M709 (VID `0x04d9`, PID `0xfc2a`) |
