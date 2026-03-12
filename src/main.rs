use rust_backup_tool::Args; 
use rust_backup_tool::errors::Result;

use log::{info, error, debug};

fn main() {
    env_logger::init();  // Initialiser le logger pour les messages de debug/info

    info!("Starting Rust Backup Tool...");

    let args = Args::parse_args();

    match run(args) {
        Ok(_) => info!("Backup completed successfully!"),
        Err(e) => error!("Backup failed: {}", e),
    }
}

fn run(args: Args) -> Result<()> {
    debug!("Parsed arguments: {:?}", args);
    Ok(())
}