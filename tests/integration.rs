// Tests fonctionnels :

use assert_cmd::Command;    // Pour lancer le binaire
use predicates::prelude::*; // Pour les assertions (ex: fichier existant...) 
use std::fs;
use tempfile::TempDir;      // Pour créer un environnement de test isolé


// Helper pour l'usage du binaire
fn get_cli() -> Command {
    let mut cmd  = Command::cargo_bin("rust-backup-tool")
        .unwrap();  // Assure que le binaire est bien créé et accessible
}

// Test 1: option --help
#[test]
fn test_help_flag() {
    let mut cmd = get_cli();
    cmd.arg("--help")
        .assert()
        .success()                                              // Return bien 0
        .stdout(predicate::str::contains("Usage:"))             // Affiche l'aide
        .stdout(predicate::str::contains("rust-backup-tool"));  // Affiche le nom du binaire

}

//Test 2: option --version
#[test]
fn test_version_flag() {
    let mut cmd = get_cli();
    cmd.arg("--version")
        .assert()
        .success()                                              // Return bien 0
        .stdout(predicate::str::contains("rust-backup-tool"));  // Affiche le nom du binaire
}

//Test 3: sauvegarde basique
#[test]
fn test_basic_backup() {
    let mut cmd = get_cli();
    let temp_dir = TempDir::new().unwrap();  // Crée un répertoire
    fs::write(temp_dir.path().join("file1.txt"), "Hello World").unwrap();   // Créer le fichier à sauvegarder

    cmd.args(&[
        "--source", temp_dir.path().to_str().unwrap(),
        "--destination", temp_dir.path().join("backup.tar.gz").to_str().unwrap()
    ])
        .assert()
        .success();
}

//Test 4 : sauvegarde avec exclusion
#[test]
fn test_backup_with_exclusion() {
    let mut cmd = get_cli();
    let temp_dir = TempDir::new().unwrap();  // Crée un répertoire
    fs::write(temp_dir.path().join("file1.txt"), "Hello World").unwrap();   // Créer le fichier à sauvegarder
    fs::write(temp_dir.path().join("exclude.txt"), "This should be excluded").unwrap();   // Créer le fichier à exclure

    cmd.args(&[
        "--source", temp_dir.path().to_str().unwrap(),
        "--destination", temp_dir.path().join("backup.tar.gz").to_str().unwrap(),
        "--exclude", "exclude.txt"
    ])
        .assert()
        .success();
}

//Test 5 : sauvegarde avec compression
#[test]
fn test_backup_with_compression() {
    let mut cmd = get_cli();
    let temp_dir = TempDir::new().unwrap();  // Crée un répertoire
    fs::write(temp_dir.path().join("file1.txt"), "Hello World").unwrap();   // Créer le fichier à sauvegarder

    cmd.args(&[
        "--source", temp_dir.path().to_str().unwrap(),
        "--destination", temp_dir.path().join("backup.tar.gz").to_str().unwrap(),
        "--compress"
    ])
        .assert()
        .success();
}

//Test 6 : sauvegarde avec un fichier de config
#[test]
fn test_backup_with_config_file() {
    let mut cmd = get_cli();
    let temp_dir = TempDir::new().unwrap();  // Crée un répertoire
    fs::write(temp_dir.path().join("file1.txt"), "Hello World").unwrap();   // Créer le fichier à sauvegarder

    // Créer un fichier de config
    let config_content = r#"
        source = ["file1.txt"]
        destination = "backup.tar.gz"
        compress = true
    "#;
    fs::write(temp_dir.path().join("config.toml"), config_content).unwrap();

    cmd.args(&[
        "--config", temp_dir.path().join("config.toml").to_str().unwrap()
    ])
        .assert()
        .success();
}

//Test 7 : sauvegarde sans sources valides
#[test]
fn test_backup_no_valid_sources() {
    let mut cmd = get_cli();
    let temp_dir = TempDir::new().unwrap();  // Crée un répertoire

    cmd.args(&[
        "--source", temp_dir.path().join("non_existent").to_str().unwrap(),
        "--destination", temp_dir.path().join("backup.tar.gz").to_str().unwrap()
    ])
        .assert()
        .failure()  // Doit échouer car la source n'existe pas
        .stderr(predicate::str::contains("No valid sources found"));  // Affiche un message d'erreur pertinent
}

//Test 8 : sauvegarde sans destination valide
#[test]
fn test_backup_no_valid_destination() {
    let mut cmd = get_cli();
    let temp_dir = TempDir::new().unwrap();  // Crée un répertoire
    fs::write(temp_dir.path().join("file1.txt"), "Hello World").unwrap();   // Créer le fichier à sauvegarder

    cmd.args(&[
        "--source", temp_dir.path().to_str().unwrap(),
        "--destination", temp_dir.path().join("non_existent/backup.tar.gz").to_str().unwrap()
    ])
        .assert()
        .failure()  // Doit échouer car la destination n'est pas valide
        .stderr(predicate::str::contains("Invalid destination path"));  // Affiche un message d'erreur pertinent
}

//Test 9: sauvegarde avec un fichier de config invalide
#[test]
fn test_backup_with_invalid_config_file() {
    let mut cmd = get_cli();
    let temp_dir = TempDir::new().unwrap();  // Crée un répertoire

    // Créer un fichier de config invalide
    let config_content = r#"
        source = ["file1.txt"]
        destination = "backup.tar.gz
        compress = true
    "#;  // Note: la ligne destination est mal formée (guillemet manquant)
    fs::write(temp_dir.path().join("config.toml"), config_content).unwrap();

    cmd.args(&[
        "--config", temp_dir.path().join("config.toml").to_str().unwrap()
    ])
        .assert()
        .failure()  // Doit échouer car le fichier de config est invalide
        .stderr(predicate::str::contains("Failed to parse config file"));  // Affiche un message d'erreur pertinent
}