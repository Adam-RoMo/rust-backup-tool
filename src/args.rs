// Définition des arguments CLI (clap)

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Source directory to backup
    #[arg(short, long, required_unless_present = "config")]
    source: Option<String>,

    /// Destination directory for backup
    #[arg(short, long, required_unless_present = "config")]
    destination: Option<String>,

    /// Fichier de configuration
    #[arg(short, long)]
    config: Option<String>,

    /// Exclude patterns (comma separated)
    #[arg(short, long)]
    exclude: Option<String>,

    /// Compress the backup    
    #[arg(short= 'z', long)]
    compress: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

impl Args {
    /// Méthode utilitaire pour parser les arguments facilement depuis main
    pub fn parse_args() -> Self {
        Args::parse()
    }
}
