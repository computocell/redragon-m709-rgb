use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use m709_core::{
    ButtonAction, LightMode, MODEL_NAME, PRODUCT_ID, Profile, ProfileManager, RgbColor, VENDOR_ID,
};
use m709_usb::{is_device_connected, M709Device};

#[derive(Parser)]
#[command(name = "m709")]
#[command(about = "Redragon M709 RGB and Configuration CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Detect if the Redragon M709 is connected
    Detect,

    /// Read and display active configuration from the mouse
    Read,

    /// Switch active hardware profile (1 to 5)
    SetProfile {
        /// Profile number (1 to 5)
        #[arg(value_parser = clap::value_parser!(u8).range(1..=5))]
        profile: u8,
    },

    /// Configure LED lighting on the active profile
    SetLed {
        /// Color in hex format (e.g. #FF0055 or 00FF00)
        #[arg(short, long)]
        color: Option<String>,

        /// Lighting mode (static, breathing, rainbow, wave, etc.)
        #[arg(short, long)]
        mode: Option<String>,

        /// Brightness level (1 to 3)
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=3))]
        brightness: Option<u8>,

        /// Animation speed (1 to 8)
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=8))]
        speed: Option<u8>,
    },

    /// Manage saved profiles in ~/.config/redragon-mouse/
    Profile {
        #[command(subcommand)]
        action: ProfileCommands,
    },

    /// Restore standard factory button mappings (fixing scroll wheel and buttons)
    RestoreButtons,
}

#[derive(Subcommand)]
enum ProfileCommands {
    /// List all saved local profiles
    List,

    /// Save the mouse's active profile to a local TOML file
    Save {
        /// Profile name
        name: String,
        /// Optional description
        #[arg(short, long)]
        description: Option<String>,
    },

    /// Apply a saved local TOML profile to the mouse's active profile
    Apply {
        /// Profile name
        name: String,
    },

    /// Backup entire mouse hardware memory (all 5 profiles) to local file
    Backup,

    /// Restore entire mouse hardware memory from local backup
    Restore,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Detect => {
            println!(
                "Procurando por {} [VID: {:04x}, PID: {:04x}]...",
                MODEL_NAME, VENDOR_ID, PRODUCT_ID
            );
            if is_device_connected()? {
                println!("✓ {} detectado e conectado com sucesso!", MODEL_NAME);
            } else {
                println!("✗ {} não encontrado no barramento USB.", MODEL_NAME);
            }
        }

        Commands::Read => {
            println!("Conectando ao {}...", MODEL_NAME);
            let dev = M709Device::open().context("Falha ao abrir dispositivo USB M709")?;
            println!("Lendo configurações da memória interna...");
            let config = dev.read_settings().context("Falha ao ler dados do mouse")?;

            println!("==================================================");
            println!(" Dispositivo: {}", MODEL_NAME);
            println!(
                " Perfil Ativo no Hardware: Perfil {}",
                config.active_profile.display_number()
            );
            println!("==================================================");

            for (idx, p) in config.profiles.iter().enumerate() {
                println!(
                    " [Perfil {}] Cor: {} | Modo: {:?} | Brilho: {} | Vel: {} | Polling: {}Hz",
                    idx + 1,
                    p.color.to_hex(),
                    p.lightmode,
                    p.brightness,
                    p.speed,
                    p.polling_rate.hz()
                );
                print!("   DPIs: ");
                for (s_idx, stage) in p.dpi_stages.iter().enumerate() {
                    let mark = if stage.enabled { "✓" } else { "✗" };
                    print!("{}[{} DPI] ", mark, stage.dpi);
                    if s_idx < 4 {
                        print!("| ");
                    }
                }
                println!();
                print!("   Botões: ");
                for (b_idx, action) in p.keymap.iter().enumerate() {
                    print!("{}: {:?} ", b_idx, action);
                }
                println!();
            }
            println!("==================================================");
        }

        Commands::SetProfile { profile } => {
            let prof = Profile::from_index((profile - 1) as usize).unwrap();
            let dev = M709Device::open().context("Falha ao abrir dispositivo USB M709")?;
            dev.write_profile(prof)
                .context("Falha ao gravar perfil no mouse")?;
            println!("✓ Perfil alterado para Perfil {} com sucesso!", profile);
        }

        Commands::SetLed {
            color,
            mode,
            brightness,
            speed,
        } => {
            println!("Conectando ao {}...", MODEL_NAME);
            let dev = M709Device::open().context("Falha ao abrir dispositivo USB M709")?;
            println!("Lendo configuração atual...");
            let mut config = dev.read_settings().context("Falha ao ler dados atuais")?;
            let active_idx = config.active_profile.index();

            if let Some(c) = color {
                if let Some(rgb) = RgbColor::from_hex(&c) {
                    config.profiles[active_idx].color = rgb;
                    println!("- Cor definida para: {}", rgb.to_hex());
                } else {
                    bail!("Formato de cor hexadecimal inválido: {}", c);
                }
            }

            if let Some(m) = mode {
                if let Some(lm) = LightMode::from_str_name(&m) {
                    config.profiles[active_idx].lightmode = lm;
                    println!("- Modo definido para: {:?}", lm);
                } else {
                    bail!(
                        "Modo de luz desconhecido: {}. Modos: static, breathing, rainbow, wave, etc.",
                        m
                    );
                }
            }

            if let Some(b) = brightness {
                config.profiles[active_idx].brightness = b;
                println!("- Brilho definido para: {}", b);
            }

            if let Some(s) = speed {
                config.profiles[active_idx].speed = s;
                println!("- Velocidade definida para: {}", s);
            }

            println!("Gravando nova configuração no hardware...");
            dev.write_settings(&config)
                .context("Falha ao gravar configurações no hardware")?;
            dev.write_profile(config.active_profile)
                .context("Falha ao recarregar perfil no hardware")?;
            println!("✓ Iluminação atualizada e recarregada no mouse com sucesso!");
        }

        Commands::Profile { action } => {
            let pm = ProfileManager::new().context("Falha ao inicializar gerenciador de perfis")?;

            match action {
                ProfileCommands::List => {
                    let profiles = pm.list_profiles()?;
                    println!("Perfis salvos em {}:", pm.dir().display());
                    if profiles.is_empty() {
                        println!("  (Nenhum perfil salvo ainda)");
                    } else {
                        for p in profiles {
                            println!("  - {}", p);
                        }
                    }
                }

                ProfileCommands::Save { name, description } => {
                    let dev = M709Device::open().context("Falha ao abrir dispositivo USB M709")?;
                    let config = dev.read_settings().context("Falha ao ler dados do mouse")?;
                    let active = &config.profiles[config.active_profile.index()];

                    let path = pm.save_profile(&name, description.as_deref(), active)?;
                    println!("✓ Perfil salvo com sucesso em: {}", path.display());
                }

                ProfileCommands::Apply { name } => {
                    let loaded = pm.load_profile(&name)?;
                    let dev = M709Device::open().context("Falha ao abrir dispositivo USB M709")?;
                    let mut config = dev.read_settings().context("Falha ao ler dados do mouse")?;
                    let active_idx = config.active_profile.index();
                    config.profiles[active_idx] = loaded;

                    dev.write_settings(&config)
                        .context("Falha ao aplicar perfil no mouse")?;
                    println!("✓ Perfil '{}' aplicado com sucesso ao hardware!", name);
                }

                ProfileCommands::Backup => {
                    let dev = M709Device::open().context("Falha ao abrir dispositivo USB M709")?;
                    let config = dev.read_settings().context("Falha ao ler dados do mouse")?;
                    let path = pm.save_full_backup(&config)?;
                    println!("✓ Backup completo gravado em: {}", path.display());
                }

                ProfileCommands::Restore => {
                    let config = pm.load_full_backup()?;
                    let dev = M709Device::open().context("Falha ao abrir dispositivo USB M709")?;
                    dev.write_settings(&config)
                        .context("Falha ao restaurar configurações")?;
                    println!("✓ Configuração completa restaurada no mouse com sucesso!");
                }
            }
        }

        Commands::RestoreButtons => {
            println!("Conectando ao {}...", MODEL_NAME);
            let dev = M709Device::open().context("Falha ao abrir dispositivo USB M709")?;
            println!("Lendo configuração atual do mouse...");
            let mut config = dev.read_settings().context("Falha ao ler dados atuais")?;
            println!("Restaurando mapeamento padrão de botões e scroll em todos os perfis...");
            for p in &mut config.profiles {
                p.keymap = [
                    ButtonAction::LeftClick,
                    ButtonAction::RightClick,
                    ButtonAction::MiddleClick,
                    ButtonAction::Forward,
                    ButtonAction::Backward,
                    ButtonAction::DpiCycle,
                    ButtonAction::ScrollUp,
                    ButtonAction::ScrollDown,
                ];
            }
            println!("Gravando no mouse...");
            dev.write_settings(&config)
                .context("Falha ao gravar botões no hardware")?;
            println!("✓ Mapeamento de botões restaurado com sucesso! O scroll do mouse voltou a funcionar.");
        }
    }

    Ok(())
}
