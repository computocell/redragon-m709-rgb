use serde::{Deserialize, Serialize};

pub const NUM_PROFILES: usize = 5;
pub const NUM_DPI_LEVELS: usize = 5;
pub const NUM_BUTTONS: usize = 8;

/// Hardware Profile selection (1..=5)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Profile {
    Profile1 = 0,
    Profile2 = 1,
    Profile3 = 2,
    Profile4 = 3,
    Profile5 = 4,
}

impl Profile {
    pub fn index(self) -> usize {
        self as usize
    }

    pub fn from_index(idx: usize) -> Option<Self> {
        match idx {
            0 => Some(Profile::Profile1),
            1 => Some(Profile::Profile2),
            2 => Some(Profile::Profile3),
            3 => Some(Profile::Profile4),
            4 => Some(Profile::Profile5),
            _ => None,
        }
    }

    pub fn display_number(self) -> u8 {
        (self as u8) + 1
    }
}

/// 24-bit RGB Color
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim().trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Self { r, g, b })
    }

    pub fn to_hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    pub fn to_bytes(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

/// LED Lighting Modes supported by M709 hardware
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LightMode {
    Off,
    BreathingRainbow,
    Static,
    Breathing,
    Rainbow,
    Flashing,
    Wave,
    ReactiveButton,
    Random,
    Alternating,
    Reactive,
}

impl LightMode {
    pub fn to_bytes(self) -> [u8; 2] {
        match self {
            LightMode::Off => [0x00, 0x00],
            LightMode::BreathingRainbow => [0x01, 0x01],
            LightMode::Static => [0x01, 0x02],
            LightMode::Breathing => [0x01, 0x04],
            LightMode::Rainbow => [0x01, 0x08],
            LightMode::Flashing => [0x01, 0x10],
            LightMode::Wave => [0x02, 0x00],
            LightMode::ReactiveButton => [0x03, 0x00],
            LightMode::Random => [0x04, 0x00],
            LightMode::Alternating => [0x06, 0x00],
            LightMode::Reactive => [0x07, 0x00],
        }
    }

    pub fn from_bytes(bytes: [u8; 2]) -> Option<Self> {
        match bytes {
            [0x00, 0x00] => Some(LightMode::Off),
            [0x01, 0x01] => Some(LightMode::BreathingRainbow),
            [0x01, 0x02] => Some(LightMode::Static),
            [0x01, 0x04] => Some(LightMode::Breathing),
            [0x01, 0x08] => Some(LightMode::Rainbow),
            [0x01, 0x10] => Some(LightMode::Flashing),
            [0x02, 0x00] => Some(LightMode::Wave),
            [0x03, 0x00] => Some(LightMode::ReactiveButton),
            [0x04, 0x00] => Some(LightMode::Random),
            [0x06, 0x00] => Some(LightMode::Alternating),
            [0x07, 0x00] => Some(LightMode::Reactive),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            LightMode::Off => "off",
            LightMode::BreathingRainbow => "breathing_rainbow",
            LightMode::Static => "static",
            LightMode::Breathing => "breathing",
            LightMode::Rainbow => "rainbow",
            LightMode::Flashing => "flashing",
            LightMode::Wave => "wave",
            LightMode::ReactiveButton => "reactive_button",
            LightMode::Random => "random",
            LightMode::Alternating => "alternating",
            LightMode::Reactive => "reactive",
        }
    }

    pub fn from_str_name(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "off" => Some(LightMode::Off),
            "breathing_rainbow" => Some(LightMode::BreathingRainbow),
            "static" => Some(LightMode::Static),
            "breathing" => Some(LightMode::Breathing),
            "rainbow" => Some(LightMode::Rainbow),
            "flashing" => Some(LightMode::Flashing),
            "wave" => Some(LightMode::Wave),
            "reactive_button" => Some(LightMode::ReactiveButton),
            "random" => Some(LightMode::Random),
            "alternating" => Some(LightMode::Alternating),
            "reactive" => Some(LightMode::Reactive),
            _ => None,
        }
    }
}

/// USB Polling / Report Rate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum PollingRate {
    Hz1000 = 0x01,
    Hz500  = 0x02,
    Hz250  = 0x04,
    Hz125  = 0x08,
}

impl PollingRate {
    pub fn to_byte(self) -> u8 {
        self as u8
    }

    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            0x01 => Some(PollingRate::Hz1000),
            0x02 => Some(PollingRate::Hz500),
            0x04 => Some(PollingRate::Hz250),
            0x08 => Some(PollingRate::Hz125),
            _ => None,
        }
    }

    pub fn hz(self) -> u16 {
        match self {
            PollingRate::Hz1000 => 1000,
            PollingRate::Hz500 => 500,
            PollingRate::Hz250 => 250,
            PollingRate::Hz125 => 125,
        }
    }
}

/// Single DPI Stage Configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DpiStage {
    pub enabled: bool,
    pub raw: [u8; 2],
    pub dpi: u32,
}

impl DpiStage {
    pub fn new(dpi: u32, enabled: bool) -> Self {
        let raw = dpi_to_bytes(dpi);
        Self { enabled, raw, dpi }
    }
}

/// Helper function to convert DPI value to 2-byte register code
pub fn dpi_to_bytes(dpi: u32) -> [u8; 2] {
    match dpi {
        200 => [0x04, 0x00],
        300 => [0x06, 0x00],
        400 => [0x09, 0x00],
        500 => [0x0B, 0x00],
        600 => [0x0D, 0x00],
        700 => [0x0F, 0x00],
        800 => [0x12, 0x00],
        900 => [0x14, 0x00],
        1000 => [0x16, 0x00],
        1200 => [0x1B, 0x00],
        1500 => [0x21, 0x00],
        1800 => [0x28, 0x00],
        2000 => [0x2D, 0x00],
        2200 => [0x31, 0x00],
        2500 => [0x38, 0x00],
        3000 => [0x43, 0x00],
        3500 => [0x4F, 0x00],
        4000 => [0x5A, 0x00],
        4500 => [0x66, 0x00],
        5000 => [0x71, 0x00],
        5500 => [0x7C, 0x00],
        6000 => [0x87, 0x00],
        6200 => [0x8C, 0x00],
        6400 => [0x48, 0x01],
        8000 => [0x5A, 0x01],
        10000 => [0x71, 0x01],
        12400 => [0x8C, 0x01],
        other => {
            if other <= 6200 {
                let code = ((other as f32 / 100.0) * 2.25).round() as u8;
                [code, 0x00]
            } else {
                let code = (((other - 6200) as f32 / 100.0) * 1.15).round() as u8 + 0x44;
                [code, 0x01]
            }
        }
    }
}

/// Helper function to convert 2-byte register code to approximate DPI value
pub fn bytes_to_dpi(bytes: [u8; 2]) -> u32 {
    match bytes {
        [0x04, 0x00] => 200,
        [0x06, 0x00] => 300,
        [0x09, 0x00] => 400,
        [0x0B, 0x00] => 500,
        [0x0D, 0x00] => 600,
        [0x0F, 0x00] => 700,
        [0x12, 0x00] => 800,
        [0x14, 0x00] => 900,
        [0x16, 0x00] => 1000,
        [0x1B, 0x00] => 1200,
        [0x21, 0x00] => 1500,
        [0x28, 0x00] => 1800,
        [0x2D, 0x00] => 2000,
        [0x31, 0x00] => 2200,
        [0x38, 0x00] => 2500,
        [0x43, 0x00] => 3000,
        [0x4F, 0x00] => 3500,
        [0x5A, 0x00] => 4000,
        [0x66, 0x00] => 4500,
        [0x71, 0x00] => 5000,
        [0x7C, 0x00] => 5500,
        [0x87, 0x00] => 6000,
        [0x8C, 0x00] => 6200,
        [0x48, 0x01] => 6400,
        [0x5A, 0x01] => 8000,
        [0x71, 0x01] => 10000,
        [0x8C, 0x01] => 12400,
        [b0, 0x00] => ((b0 as f32 / 2.25) * 100.0).round() as u32,
        [b0, 0x01] => 6200 + (((b0.saturating_sub(0x44)) as f32 / 1.15) * 100.0).round() as u32,
        _ => 1000,
    }
}

/// Physical buttons on Redragon M709
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(usize)]
pub enum PhysicalButton {
    Left = 0,
    Right = 1,
    Middle = 2,
    Forward = 3,
    Backward = 4,
    Dpi = 5,
    ScrollUp = 6,
    ScrollDown = 7,
}

impl PhysicalButton {
    pub const ALL: [PhysicalButton; 8] = [
        PhysicalButton::Left,
        PhysicalButton::Right,
        PhysicalButton::Middle,
        PhysicalButton::Forward,
        PhysicalButton::Backward,
        PhysicalButton::Dpi,
        PhysicalButton::ScrollUp,
        PhysicalButton::ScrollDown,
    ];

    pub fn name(self) -> &'static str {
        match self {
            PhysicalButton::Left => "Botão Esquerdo",
            PhysicalButton::Right => "Botão Direito",
            PhysicalButton::Middle => "Botão do Meio",
            PhysicalButton::Forward => "Avançar",
            PhysicalButton::Backward => "Voltar",
            PhysicalButton::Dpi => "DPI",
            PhysicalButton::ScrollUp => "Scroll Up",
            PhysicalButton::ScrollDown => "Scroll Down",
        }
    }
}

/// Action mapped to a physical button (4 bytes representation)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonAction {
    LeftClick,
    RightClick,
    MiddleClick,
    Forward,
    Backward,
    DpiCycle,
    DpiDown,
    DpiUp,
    ScrollUp,
    ScrollDown,
    ProfileSwitch,
    DpiLedToggle,
    None,
    Raw([u8; 4]),
}

impl ButtonAction {
    pub fn to_bytes(&self) -> [u8; 4] {
        match *self {
            ButtonAction::LeftClick => [0x81, 0x00, 0x00, 0x00],
            ButtonAction::RightClick => [0x82, 0x00, 0x00, 0x00],
            ButtonAction::MiddleClick => [0x83, 0x00, 0x00, 0x00],
            ButtonAction::Forward => [0x85, 0x00, 0x00, 0x00],
            ButtonAction::Backward => [0x84, 0x00, 0x00, 0x00],
            ButtonAction::DpiCycle => [0x88, 0x00, 0x00, 0x00],
            ButtonAction::DpiDown => [0x89, 0x00, 0x00, 0x00],
            ButtonAction::DpiUp => [0x8A, 0x00, 0x00, 0x00],
            ButtonAction::ScrollUp => [0x8B, 0x00, 0x00, 0x00],
            ButtonAction::ScrollDown => [0x8C, 0x00, 0x00, 0x00],
            ButtonAction::ProfileSwitch => [0x8D, 0x00, 0x00, 0x00],
            ButtonAction::DpiLedToggle => [0x9B, 0x01, 0x00, 0x00],
            ButtonAction::None => [0x00, 0x00, 0x00, 0x00],
            ButtonAction::Raw(raw) => raw,
        }
    }

    pub fn from_bytes(bytes: [u8; 4]) -> Self {
        match bytes {
            [0x81, 0x00, 0x00, 0x00] => ButtonAction::LeftClick,
            [0x82, 0x00, 0x00, 0x00] => ButtonAction::RightClick,
            [0x83, 0x00, 0x00, 0x00] => ButtonAction::MiddleClick,
            [0x85, 0x00, 0x00, 0x00] | [0x90, 0x00, 0xe0, 0x00] => ButtonAction::Forward,
            [0x84, 0x00, 0x00, 0x00] | [0x90, 0x00, 0xe1, 0x00] => ButtonAction::Backward,
            [0x88, 0x00, 0x00, 0x00] => ButtonAction::DpiCycle,
            [0x89, 0x00, 0x00, 0x00] => ButtonAction::DpiDown,
            [0x8A, 0x00, 0x00, 0x00] => ButtonAction::DpiUp,
            [0x8B, 0x00, 0x00, 0x00] => ButtonAction::ScrollUp,
            [0x8C, 0x00, 0x00, 0x00] => ButtonAction::ScrollDown,
            [0x8D, 0x00, 0x00, 0x00] => ButtonAction::ProfileSwitch,
            [0x9B, 0x01, 0x00, 0x00] => ButtonAction::DpiLedToggle,
            [0x00, 0x00, 0x00, 0x00] => ButtonAction::None,
            other => ButtonAction::Raw(other),
        }
    }
}

/// Full configuration for one Profile
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub color: RgbColor,
    pub lightmode: LightMode,
    pub brightness: u8, // 1..=3
    pub speed: u8,      // 1..=8
    pub scroll_speed: u8,
    pub polling_rate: PollingRate,
    pub dpi_stages: [DpiStage; NUM_DPI_LEVELS],
    pub keymap: [ButtonAction; NUM_BUTTONS],
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            color: RgbColor::new(0xFF, 0x00, 0x00),
            lightmode: LightMode::Static,
            brightness: 3,
            speed: 8,
            scroll_speed: 1,
            polling_rate: PollingRate::Hz125,
            dpi_stages: [
                DpiStage::new(200, true),
                DpiStage::new(1000, true),
                DpiStage::new(2000, true),
                DpiStage::new(3000, true),
                DpiStage::new(6200, true),
            ],
            keymap: [
                ButtonAction::LeftClick,
                ButtonAction::RightClick,
                ButtonAction::MiddleClick,
                ButtonAction::Forward,
                ButtonAction::Backward,
                ButtonAction::DpiCycle,
                ButtonAction::ScrollUp,
                ButtonAction::ScrollDown,
            ],
        }
    }
}

/// Full configuration of the Redragon M709
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MouseConfig {
    pub active_profile: Profile,
    pub profiles: [ProfileConfig; NUM_PROFILES],
}

impl Default for MouseConfig {
    fn default() -> Self {
        let mut p1 = ProfileConfig::default();
        p1.color = RgbColor::new(0xFF, 0x00, 0x00); // Red

        let mut p2 = ProfileConfig::default();
        p2.color = RgbColor::new(0x00, 0x00, 0xFF); // Blue

        let mut p3 = ProfileConfig::default();
        p3.color = RgbColor::new(0x00, 0xFF, 0x00); // Green

        let mut p4 = ProfileConfig::default();
        p4.color = RgbColor::new(0xFF, 0x00, 0xFF); // Magenta

        let mut p5 = ProfileConfig::default();
        p5.color = RgbColor::new(0xFF, 0xFF, 0x00); // Yellow

        Self {
            active_profile: Profile::Profile1,
            profiles: [p1, p2, p3, p4, p5],
        }
    }
}
