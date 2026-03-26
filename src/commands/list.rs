use crate::crypto::kdf::derive_master_key;
use crate::vault::storage::{load_salt, load_vault, vault_exists};
use rpassword::prompt_password;

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    if !vault_exists() {
        println!("Vault not found. Please run `pm init` first.");
        return Ok(());
    }

    let password = prompt_password("Enter master password to unlock vault: ")?;
    let salt = load_salt()?;
    let master_key = derive_master_key(&password, &salt)?;

    let vault = load_vault(&*master_key)?;

    if vault.items.is_empty() {
        println!("Vault is empty.");
        return Ok(());
    }

    println!("{:-<50}", "");
    println!("{:<20} | {:<25}", "Name", "Username");
    println!("{:-<50}", "");

    for item in &vault.items {
        println!("{:<20} | {:<25}", item.name, item.username);
    }
    println!("{:-<50}", "");

    Ok(())
}
