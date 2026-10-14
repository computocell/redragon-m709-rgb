# Protocol & Reverse Engineering Specification: Redragon M709 (VID 0x04D9, PID 0xFC2A)

## 1. Executive Summary & Device Identification

- **Device Name**: Redragon M709 (internally designated `"709"`)
- **USB Vendor ID (VID)**: `0x04D9` (Holtek Semiconductor, Inc.)
- **USB Product ID (PID)**: `0xFC2A`
- **USB Interfaces**:
  - The device presents 3 USB HID interfaces (`Interface 0`, `Interface 1`, `Interface 2`).
  - Kernel driver must be detached from active interfaces prior to claiming.
  - All proprietary feature configuration control transfers are targeted to **Interface 2** (`wIndex = 0x0002`).

---

## 2. USB Control Transfer Specifications

All communication with the mouse firmware occurs over the USB Default Control Pipe (Endpoint 0) using HID Class-specific Requests.

### 2.1 Standard Transfer Parameters

| Operation | `bmRequestType` | `bRequest` | `wValue` | `wIndex` | Length | Timeout |
|---|---|---|---|---|---|---|
| **Write Standard (Feature OUT)** | `0x21` (`Host-to-Device \| Class \| Interface`) | `0x09` (`SET_REPORT`) | `0x0302` (Feature, ID `0x02`) | `0x0002` (Interface 2) | 16 bytes | 1000 ms |
| **Write 64B Buffer (Feature OUT)** | `0x21` | `0x09` | `0x0303` (Feature, ID `0x03`) | `0x0002` (Interface 2) | 64 bytes | 1000 ms |
| **Write Macro Bytecode (Feature OUT)**| `0x21` | `0x09` | `0x0302` (Feature, ID `0x02`) | `0x0002` (Interface 2) | 256 bytes | 1000 ms |
| **Read Standard Trigger (OUT)** | `0x21` | `0x09` | `0x0302` | `0x0002` | 16 bytes | 1000 ms |
| **Read Standard Response (IN)** | `0xA1` (`Device-to-Host \| Class \| Interface`) | `0x01` (`GET_REPORT`) | `0x0302` | `0x0002` | 16 bytes | 1000 ms |
| **Read 64B Trigger (OUT)** | `0x21` | `0x09` | `0x0303` | `0x0002` | 64 bytes | 1000 ms |
| **Read 64B Response (IN)** | `0xA1` | `0x01` | `0x0303` | `0x0002` | 64 bytes | 1000 ms |

---

## 3. Profile Switching (`write_profile`)

Switching the active hardware profile sends a burst of **6 packets** of 16 bytes each.

- **Base Template**: `_c_data_s_profile[6][16]`
- **Active Profile Values**: `0x00` (Profile 1) to `0x04` (Profile 5).
- **Modification**:
  - `buffer[0][8] = profile_id` (Byte offset 8 in Packet 0).
  - Packets 1 to 5 are static command/synchronization frames.

### Packet Sequence:
```
Packet 0: [0x02, 0xf3, 0x2c, 0x00, 0x02, 0x00, 0x00, 0x00, PROFILE_ID, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
Packet 1: [0x02, 0xf1, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
Packet 2: [0x02, 0xf1, 0x02, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00,       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
Packet 3: [0x02, 0xf1, 0x02, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00,       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
Packet 4: [0x02, 0xf1, 0x02, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
Packet 5: [0x02, 0xf1, 0x02, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00,       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
```
All 6 packets are sent with `bmRequestType=0x21, bRequest=0x09, wValue=0x0302, wIndex=0x0002, len=16`.

---

## 4. Writing Settings (`write_settings`)

`write_settings` persists all 5 profiles, lighting configurations, polling rates, scroll speeds, DPI levels, and button maps in **3 sequential buffer parts**.

```
Part 1 (buffer1): 15 packets × 16 bytes (Unlock, LED config, Polling Rates)
Part 2 (buffer2):  1 packet  × 64 bytes (Scroll Speeds for all profiles)
Part 3 (buffer3): 80 packets × 16 bytes (DPI levels, Button keymaps, Commit)
```

### 4.1 Part 1 (`buffer1`: 15 packets × 16 bytes)
- Initialized from `_c_data_settings_1[15][16]`.
- **Packet 0**: `{0x02, 0xf5, 0x00, ...}` (Unlock configuration memory write).
- **Packets 3 - 12 (Lighting & Speed for Profiles 0..4)**:
  For profile index `p` in `0..4`:
  - **LED Config & Speed Packet**: row `3 + 2 * p`
    - Byte `8`: Color Red (`0x00` - `0xFF`)
    - Byte `9`: Color Green (`0x00` - `0xFF`)
    - Byte `10`: Color Blue (`0x00` - `0xFF`)
    - Byte `11`: Lightmode Byte 0
    - Byte `12`: Animation Speed (`0x01` to `0x08`)
    - Byte `13`: Lightmode Byte 1
  - **Brightness Packet**: row `4 + 2 * p`
    - Byte `8`: Brightness Level (`0x01` to `0x03`)
- **Packets 13 - 14 (USB Polling Rate)**:
  - Row 13 (Profiles 0, 1, 2):
    - Profile 0: Byte `8`
    - Profile 1: Byte `10`
    - Profile 2: Byte `12`
  - Row 14 (Profiles 3, 4):
    - Profile 3: Byte `8`
    - Profile 4: Byte `10`

### 4.2 Part 2 (`buffer2`: 1 packet × 64 bytes)
- Initialized from `_c_data_settings_2[64]`.
- **Scroll Speed** (`0x01` to `0x3F`):
  - Profile 0: `buffer2[8]`
  - Profile 1: `buffer2[10]`
  - Profile 2: `buffer2[12]`
  - Profile 3: `buffer2[14]`
  - Profile 4: `buffer2[16]`
- Dispatched via `0x21, 0x09, 0x0303, 0x0002, len 64`.

### 4.3 Part 3 (`buffer3`: 80 packets × 16 bytes)
- Initialized from `_c_data_settings_3[80][16]`.
- **Packets 7 - 31 (DPI Settings for 5 Levels × 5 Profiles)**:
  - Formula: `row = 7 + (5 * level) + profile` (where `level = 0..4`, `profile = 0..4`).
    - Byte `8`: `dpi_enabled` (`0x01` = true, `0x00` = false).
    - Byte `9`: DPI Byte 0 (`_s_dpi_levels[profile][level][0]`).
    - Byte `10`: DPI Byte 1 (`_s_dpi_levels[profile][level][1]`).
- **Packets 35 - 74 (Button Keymaps: 5 Profiles × 8 Buttons)**:
  - Formula: `row = 35 + (8 * profile) + button` (where `profile = 0..4`, `button = 0..7`).
    - Byte `8`: Mapping byte 0
    - Byte `9`: Mapping byte 1
    - Byte `10`: Mapping byte 2
    - Byte `11`: Mapping byte 3
- **Packet 79**: `{0x02, 0xf5, 0x01, ...}` (Commit / Lock configuration write).

---

## 5. Reading Settings (`read_settings`)

### 5.1 Part 1: Profiles, LEDs, Polling Rate (9 OUT requests, 8 IN responses)
- Query template: `_c_data_read_1[9][16]`.
1. Send `_c_data_read_1[0]` (OUT).
2. For `i = 1..8`:
   - Send `_c_data_read_1[i]` (OUT `0x21, 0x09, 0x0302, 0x0002, len 16`).
   - Read `buffer_in1[i-1]` (IN `0xA1, 0x01, 0x0302, 0x0002, len 16`).
3. Parsing:
   - **Active Profile**: `buffer_in1[0][8]` (`0x00` = Profile 1 ... `0x04` = Profile 5).
   - **Profiles 0..4 LED Settings** (from `buffer_in1[1]` to `buffer_in1[5]`, index `p = i - 1`):
     - Red: `buffer_in1[i][8]`
     - Green: `buffer_in1[i][9]`
     - Blue: `buffer_in1[i][10]`
     - LightMode: `[buffer_in1[i][11], buffer_in1[i][13]]`
     - Speed: `buffer_in1[i][13]`
     - Brightness: `buffer_in1[i][14]`
   - **Polling Rates**:
     - Profile 0: `buffer_in1[6][8]`
     - Profile 1: `buffer_in1[6][10]`
     - Profile 2: `buffer_in1[6][12]`
     - Profile 3: `buffer_in1[7][8]`
     - Profile 4: `buffer_in1[7][10]`

---

## 6. LED / Lighting Subsystem

| Lightmode Name | Byte 0 | Byte 1 | Description |
|---|---|---|---|
| **Off** | `0x00` | `0x00` | LEDs completely disabled |
| **Breathing Rainbow** | `0x01` | `0x01` | Cyclic multi-color breathing |
| **Static** | `0x01` | `0x02` | Solid static single color |
| **Breathing** | `0x01` | `0x04` | Single color breathing |
| **Rainbow** | `0x01` | `0x08` | Smooth rainbow spectrum cycle |
| **Flashing** | `0x01` | `0x10` | Single color strobe/flash |
| **Wave** | `0x02` | `0x00` | Wave flow pattern |
| **Reactive Button** | `0x03` | `0x00` | Flashes when mouse buttons clicked |
| **Random** | `0x04` | `0x00` | Random color shifting |
| **Alternating** | `0x06` | `0x00` | Alternating color pairs |
| **Reactive** | `0x07` | `0x00` | Movement/click reactive trail |

- **Brightness Range**: `1` (low) to `3` (high).
- **Speed Range**: `1` (slow) to `8` (fast).

---

## 7. Polling Rate

| Polling Rate | Interval | Byte Value |
|---|---|---|
| **1000 Hz** | 1 ms | `0x01` |
| **500 Hz** | 2 ms | `0x02` |
| **250 Hz** | 4 ms | `0x04` |
| **125 Hz** | 8 ms | `0x08` (Padrão) |
