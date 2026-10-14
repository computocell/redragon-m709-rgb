# feat/01-03-usb-driver-cli

**Branch:** `feat/01-redragon-rgb-gui`
**Fase:** 1
**Depende de:** `feat/01-02-rea-protocol-core`
**Status:** `done`

## Objetivo
Implementar a camada de comunicação USB com o Redragon M709 (`04d9:fc2a`) no crate `m709-usb` utilizando `rusb`, e expor comandos operacionais no crate `m709-cli` para detecção, leitura e gravação no mouse real conectado.

## Critério de conclusão
```bash
# Detectar o mouse e aplicar uma cor de LED via CLI
cargo run -p m709-cli -- detect
cargo run -p m709-cli -- set-led --color "#00FF88" --mode static
```

## Tarefas
- [x] **01-03-1** Implementar detecção de dispositivo USB em `m709-usb`:
  - Varredura de barramento para VID `0x04D9` e PID `0xFC2A`
  - Obtenção de handle e desanexação de driver de kernel (`detach_kernel_driver`) se necessário
  - Claim da interface USB correta (interface 2 / HID)
- [x] **01-03-2** Implementar envio de control transfers:
  - Envio de Feature Reports (Request Type `0x21`, Request `0x09`, Value `0x0302`, Index `0x0002`)
  - Transmissão sequencial com timeouts e tratamento de erro
- [x] **01-03-3** Implementar métodos de alto nível em `M709Device`:
  - `write_profile(profile)`
  - `write_settings(&MouseSettings)`
  - `read_settings() -> Result<MouseSettings>`
- [x] **01-03-4** Construir a CLI em `m709-cli`:
  - Subcomando `detect`: lista o dispositivo e conexão
  - Subcomando `read`: lê e imprime as configurações ativas do mouse
  - Subcomando `set-led`: altera cor, modo, brilho e velocidade
  - Subcomando `set-profile`: troca o perfil ativo (1 a 5)
  - Subcomando `set-dpi`: ajusta níveis de DPI do perfil atual
- [x] **01-03-5** Testar em hardware real conectado

## Arquivos gerados
```
crates/m709-usb/src/device.rs
crates/m709-usb/src/transfer.rs
crates/m709-usb/src/error.rs
crates/m709-cli/src/main.rs
crates/m709-cli/src/commands/mod.rs
```

## Skills relevantes
- Builder
- Revisor
