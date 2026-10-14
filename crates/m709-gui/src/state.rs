use anyhow::{Context, Result};
use m709_core::{MouseConfig, Profile};
use m709_usb::{is_device_connected, M709Device};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct AppState {
    pub is_connected: bool,
    pub config: MouseConfig,
    pub status_message: String,
    pub is_dirty: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            is_connected: false,
            config: MouseConfig::default(),
            status_message: "Procurando Redragon M709...".to_string(),
            is_dirty: false,
        }
    }
}

pub type SharedState = Rc<RefCell<AppState>>;

pub fn new_shared_state() -> SharedState {
    Rc::new(RefCell::new(AppState::default()))
}

/// Polls USB device and loads settings from EEPROM if connected
pub fn refresh_from_hardware(state: &SharedState) -> Result<bool> {
    let connected = is_device_connected().unwrap_or(false);
    let mut s = state.borrow_mut();
    s.is_connected = connected;

    if !connected {
        s.status_message = "Dispositivo desconectado. Conecte o Redragon M709 via USB.".to_string();
        return Ok(false);
    }

    match M709Device::open() {
        Ok(dev) => match dev.read_settings() {
            Ok(cfg) => {
                s.config = cfg;
                s.status_message = format!(
                    "Conectado: Redragon M709 (Perfil {} ativo)",
                    s.config.active_profile.display_number()
                );
                s.is_dirty = false;
                Ok(true)
            }
            Err(e) => {
                s.status_message = format!("Erro ao ler configurações: {}", e);
                Ok(false)
            }
        },
        Err(e) => {
            s.status_message = format!("Erro ao comunicar com USB: {}", e);
            Ok(false)
        }
    }
}

/// Applies active configuration to the mouse hardware EEPROM
pub fn save_to_hardware(state: &SharedState) -> Result<()> {
    let s = state.borrow();
    if !s.is_connected {
        anyhow::bail!("Mouse não conectado!");
    }

    let dev = M709Device::open().context("Falha ao abrir USB do mouse")?;
    dev.write_settings(&s.config)
        .context("Falha ao gravar configurações na memória interna")?;
    let _ = dev.write_profile(s.config.active_profile);
    drop(s);

    let mut s = state.borrow_mut();
    s.is_dirty = false;
    s.status_message = "✓ Configuração gravada e aplicada ao mouse!".to_string();
    Ok(())
}

/// Switches active profile on the mouse hardware
pub fn switch_profile_hardware(state: &SharedState, profile: Profile) -> Result<()> {
    let mut s = state.borrow_mut();
    s.config.active_profile = profile;
    s.is_dirty = true;

    if s.is_connected {
        if let Ok(dev) = M709Device::open() {
            let _ = dev.write_profile(profile);
            s.status_message = format!("Perfil {} ativado!", profile.display_number());
        }
    }
    Ok(())
}
