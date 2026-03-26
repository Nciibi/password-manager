use super::model::Vault;
use crate::crypto::encrypt::{decrypt, encrypt, CryptoError};
use std::fs::{self, OpenOptions};
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum StorageError {
    IoError(std::io::Error),
    CryptoError(CryptoError),
    SerdeError(serde_json::Error),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::IoError(e) => write!(f, "IO error: {}", e),
            StorageError::CryptoError(e) => write!(f, "Crypto error: {}", e),
            StorageError::SerdeError(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl std::error::Error for StorageError {}

impl From<std::io::Error> for StorageError {
    fn from(err: std::io::Error) -> Self {
        StorageError::IoError(err)
    }
}

impl From<CryptoError> for StorageError {
    fn from(err: CryptoError) -> Self {
        StorageError::CryptoError(err)
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(err: serde_json::Error) -> Self {
        StorageError::SerdeError(err)
    }
}

pub fn vault_dir() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .expect("Could not determine home directory");
    let mut path = PathBuf::from(home);
    path.push(".password_manager");
    path
}

pub fn vault_path() -> PathBuf {
    let mut path = vault_dir();
    path.push("vault.json.enc");
    path
}

pub fn salt_path() -> PathBuf {
    let mut path = vault_dir();
    path.push("salt");
    path
}

pub fn ensure_vault_dir() -> Result<(), StorageError> {
    let dir = vault_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&dir)?.permissions();
            perms.set_mode(0o700);
            fs::set_permissions(&dir, perms)?;
        }
    }
    Ok(())
}

pub fn save_vault(vault: &Vault, key: &[u8; 32]) -> Result<(), StorageError> {
    ensure_vault_dir()?;
    let json_bytes = serde_json::to_vec(vault)?;
    let ciphertext = encrypt(&json_bytes, key)?;

    let mut options = OpenOptions::new();
    options.write(true).create(true).truncate(true);

    #[cfg(unix)]
    options.mode(0o600);

    let mut file = options.open(vault_path())?;
    file.write_all(&ciphertext)?;
    Ok(())
}

pub fn load_vault(key: &[u8; 32]) -> Result<Vault, StorageError> {
    let ciphertext = fs::read(vault_path())?;
    let plaintext = decrypt(&ciphertext, key)?;
    let vault = serde_json::from_slice(&plaintext)?;
    Ok(vault)
}

pub fn save_salt(salt: &[u8; 32]) -> Result<(), StorageError> {
    ensure_vault_dir()?;
    let mut options = OpenOptions::new();
    options.write(true).create(true).truncate(true);

    #[cfg(unix)]
    options.mode(0o600);

    let mut file = options.open(salt_path())?;
    file.write_all(salt)?;
    Ok(())
}

pub fn load_salt() -> Result<[u8; 32], StorageError> {
    let salt_bytes = fs::read(salt_path())?;
    if salt_bytes.len() != 32 {
        return Err(StorageError::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid salt length",
        )));
    }
    let mut salt = [0u8; 32];
    salt.copy_from_slice(&salt_bytes[..32]);
    Ok(salt)
}

pub fn vault_exists() -> bool {
    vault_path().exists() && salt_path().exists()
}
