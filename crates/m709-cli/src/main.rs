use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use m709_core::{
    LightMode, MODEL_NAME, PRODUCT_ID, Profile, RgbColor, VENDOR_ID,
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
            println!(" Perfil Ativo no Hardware: Perfil {}", config.active_profile.display_number());
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
                    bail!("Modo de luz desconhecido: {}. Modos: static, breathing, rainbow, wave, etc.", m);
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
            println!("✓ Iluminação atualizada no mouse com sucesso!");
        }
    }

    Ok(())
}
