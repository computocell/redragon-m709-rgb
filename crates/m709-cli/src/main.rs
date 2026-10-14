use anyhow::Result;
use clap::{Parser, Subcommand};
use m709_core::{MODEL_NAME, PRODUCT_ID, VENDOR_ID};
use m709_usb::is_device_connected;

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
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Detect => {
            println!("Procurando por {} [VID: {:04x}, PID: {:04x}]...", MODEL_NAME, VENDOR_ID, PRODUCT_ID);
            if is_device_connected()? {
                println!("✓ {} detectado e conectado com sucesso!", MODEL_NAME);
            } else {
                println!("✗ {} não encontrado no barramento USB.", MODEL_NAME);
            }
        }
    }

    Ok(())
}
