use crate::constants::*;
use crate::types::*;

/// Builds the 6 packets required to change the active hardware profile.
pub fn build_profile_packets(profile: Profile) -> [[u8; 16]; 6] {
    let mut buffer = DATA_S_PROFILE;
    buffer[0][8] = profile as u8;
    buffer
}

/// Builds the 3 parts of packets to write settings to the mouse.
pub fn build_settings_packets(config: &MouseConfig) -> ([[u8; 16]; 15], [u8; 64], [[u8; 16]; 80]) {
    let mut buffer1 = DATA_SETTINGS_1;
    let mut buffer2 = DATA_SETTINGS_2;
    let mut buffer3 = DATA_SETTINGS_3;

    // Buffer 1: Lighting, Speed, Brightness, Polling Rate
    for i in 0..NUM_PROFILES {
        let p = &config.profiles[i];

        // Color & Speed & Lightmode
        buffer1[3 + (2 * i)][8] = p.color.r;
        buffer1[3 + (2 * i)][9] = p.color.g;
        buffer1[3 + (2 * i)][10] = p.color.b;

        let lm = p.lightmode.to_bytes();
        buffer1[3 + (2 * i)][11] = lm[0];
        buffer1[3 + (2 * i)][12] = p.speed;
        buffer1[3 + (2 * i)][13] = lm[1];

        // Brightness
        buffer1[4 + (2 * i)][8] = p.brightness;
    }

    // Polling rates
    for i in 0..3 {
        buffer1[13][8 + (2 * i)] = config.profiles[i].polling_rate.to_byte();
    }
    for i in 3..5 {
        buffer1[14][2 + (2 * i)] = config.profiles[i].polling_rate.to_byte();
    }

    // Buffer 2: Scroll speed
    for i in 0..NUM_PROFILES {
        buffer2[8 + (2 * i)] = config.profiles[i].scroll_speed;
    }

    // Buffer 3: DPI levels and Key mappings
    // DPI: row 7 + 5*dpi_level + profile_idx
    for dpi_lvl in 0..NUM_DPI_LEVELS {
        for prof in 0..NUM_PROFILES {
            let row = 7 + (5 * dpi_lvl) + prof;
            let stage = &config.profiles[prof].dpi_stages[dpi_lvl];
            buffer3[row][8] = if stage.enabled { 0x01 } else { 0x00 };
            buffer3[row][9] = stage.raw[0];
            buffer3[row][10] = stage.raw[1];
        }
    }

    // Key mapping: row 35 + 8*profile_idx + button_idx
    for prof in 0..NUM_PROFILES {
        for btn in 0..NUM_BUTTONS {
            let row = 35 + (8 * prof) + btn;
            let bytes = config.profiles[prof].keymap[btn].to_bytes();
            buffer3[row][8..=11].copy_from_slice(&bytes);
        }
    }

    (buffer1, buffer2, buffer3)
}

/// Parses read responses into a `MouseConfig`.
pub fn parse_settings(
    in1: &[[u8; 16]; 8],
    in2: &[[u8; 64]; 85],
    in3: &[[u8; 16]; 100],
) -> MouseConfig {
    let active_profile = Profile::from_index(in1[0][8] as usize).unwrap_or(Profile::Profile1);
    let mut config = MouseConfig {
        active_profile,
        profiles: [
            ProfileConfig::default(),
            ProfileConfig::default(),
            ProfileConfig::default(),
            ProfileConfig::default(),
            ProfileConfig::default(),
        ],
    };

    for i in 0..NUM_PROFILES {
        let resp_idx = i + 1; // in1[1..=5] corresponds to profiles 0..=4
        let p = &mut config.profiles[i];

        // Color
        p.color = RgbColor::new(
            in1[resp_idx][8],
            in1[resp_idx][9],
            in1[resp_idx][10],
        );

        // Lightmode
        let lm_bytes = [in1[resp_idx][11], in1[resp_idx][13]];
        p.lightmode = LightMode::from_bytes(lm_bytes).unwrap_or(LightMode::Static);

        // Speed & Brightness
        p.speed = in1[resp_idx][13];
        p.brightness = in1[resp_idx][14];

        // Polling rate
        let rate_byte = if i < 3 {
            in1[6][8 + (2 * i)]
        } else {
            in1[7][2 + (2 * i)]
        };
        p.polling_rate = PollingRate::from_byte(rate_byte).unwrap_or(PollingRate::Hz125);

        // DPI stages from in2[i]
        for lvl in 0..NUM_DPI_LEVELS {
            let j = lvl + 1;
            let enabled = in2[i][4 + (6 * j)] != 0;
            let raw = [in2[i][5 + (6 * j)], in2[i][6 + (6 * j)]];
            let dpi = bytes_to_dpi(raw);
            p.dpi_stages[lvl] = DpiStage { enabled, raw, dpi };
        }

        // Keymaps from in3
        for btn in 0..NUM_BUTTONS {
            let row = btn + (8 * i);
            let bytes = [
                in3[row][8],
                in3[row][9],
                in3[row][10],
                in3[row][11],
            ];
            p.keymap[btn] = ButtonAction::from_bytes(bytes);
        }
    }

    config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_packets() {
        let p2 = build_profile_packets(Profile::Profile2);
        assert_eq!(p2[0][8], 1);
        assert_eq!(p2[1][1], 0xf1);
    }

    #[test]
    fn test_settings_builder() {
        let config = MouseConfig::default();
        let (b1, b2, b3) = build_settings_packets(&config);

        // Profile 1 color should be Red [0xFF, 0x00, 0x00]
        assert_eq!(b1[3][8], 0xFF);
        assert_eq!(b1[3][9], 0x00);
        assert_eq!(b1[3][10], 0x00);

        // Scroll speed
        assert_eq!(b2[8], 1);

        // Commit packet at end of b3
        assert_eq!(b3[79][1], 0xF5);
        assert_eq!(b3[79][2], 0x01);
    }
}
