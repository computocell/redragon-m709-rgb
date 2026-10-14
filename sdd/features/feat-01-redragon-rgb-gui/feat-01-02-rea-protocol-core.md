# feat/01-02-rea-protocol-core

**Branch:** `feat/01-redragon-rgb-gui`
**Fase:** 1
**Depende de:** `feat/01-01-foundation-workspace`
**Status:** `done`

## Objetivo
Analisar a especificação do protocolo USB HID do Redragon M709 (armazenada em `../mouse_m908/include/m709/`) utilizando REA / análise de protocolo, e codificar no crate `m709-core` os tipos de dados, estruturas de pacote e serializadores seguros em Rust.

## Critério de conclusão
```bash
# Executar testes unitários do crate m709-core validando a montagem exata dos pacotes de dados
cargo test -p m709-core
```

## Tarefas
- [x] **01-02-1** Mapear via REA/análise de código as matrizes de pacotes do M709:
  - `_c_data_s_profile` (6 x 16 bytes para troca de perfil)
  - `_c_data_settings_1` (15 x 16 bytes para cabeçalhos de configurações)
  - `_c_data_settings_2` (64 bytes para configurações globais)
  - `_c_data_settings_3` (80 x 16 bytes para tabela de botões e luzes)
  - `_c_data_read_*` (sequência de leitura de configurações)
- [x] **01-02-2** Implementar enums fortemente tipados em Rust:
  - `LightMode` (Static, Breathing, BreathingRainbow, Rainbow, Wave, Alternating, Reactive, Flashing, Random, Off)
  - `ReportRate` (125Hz, 250Hz, 500Hz, 1000Hz)
  - `Profile` (Profile1 a Profile5)
  - `Button` (Left, Right, Middle, Forward, Backward, Dpi, ScrollUp, ScrollDown)
- [x] **01-02-3** Implementar structs de domínio:
  - `RgbColor` (r, g, b), `LedSettings` (mode, color, brightness 1-3, speed 1-8)
  - `DpiSettings` (5 níveis x 5 perfis, flags ativo/inativo)
  - `KeymapSettings` (8 botões mapeáveis para mouse/teclado/mídia)
  - `MouseSettings` agrupando todos os parâmetros
- [x] **01-02-4** Implementar construtores de pacotes de escrita (`PacketBuilder`):
  - Montagem de buffers de troca de perfil
  - Montagem de buffers de gravação de configurações
- [x] **01-02-5** Implementar parser de pacotes de leitura (`PacketParser`):
  - Decodificação dos buffers lidos do mouse em `MouseSettings`
- [x] **01-02-6** Adicionar testes unitários comparando os buffers gerados com as constantes de referência do `data.cpp`

## Arquivos gerados
```
crates/m709-core/src/types.rs
crates/m709-core/src/packets/mod.rs
crates/m709-core/src/packets/constants.rs
crates/m709-core/src/packets/writer.rs
crates/m709-core/src/packets/reader.rs
crates/m709-core/tests/packet_tests.rs
```

## Skills relevantes
- Builder
- Revisor
