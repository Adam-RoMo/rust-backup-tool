// Types d'erreurs personnalisés (thiserror)

use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackupError {

    //Erreurs métier : 
    #[error("Invalid sources : {0}")]
    SourceNotFound(PathBuf),

    #[error("Invalid destination path")]
    InvalidDestination(PathBuf),

    #[error("Invalid Configuration: {0}")]
    InvalidConfig(String),

    //Erreurs techniques :
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, BackupError>;