use crate::crypto::kdf::derive_master_key;
use crate::security::clipboard::copy_and_clear;
use crate::vault::storage::{load_salt, load_vault, vault_exists};
use rpassword::prompt_password;

pub fn execute(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !vault_exists() {
        println!("Vault not found. Please run `pm init` first.");
        return Ok(());
    }

    let password = prompt_password("Enter master password to unlock vault: ")?;
    let salt = load_salt()?;
    let master_key = derive_master_key(&password, &salt)?;

    let vault = load_vault(&*master_key)?;

    if let Some(item) = vault.items.iter().find(|i| i.name == name) {
        copy_and_clear(item.password.clone(), 15)?;
        println!("Password for '{}' copied to clipboard. It will clear in 15 seconds.", item.name);
    } else {
        println!("Item '{}' not found in vault.", name);
    }

    Ok(())
}
