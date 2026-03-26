use crate::crypto::kdf::derive_master_key;
use crate::vault::storage::{load_salt, load_vault, save_vault, vault_exists};
use rpassword::prompt_password;
use std::io::{self, Write};

pub fn execute(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !vault_exists() {
        println!("Vault not found. Please run `pm init` first.");
        return Ok(());
    }

    let master_pwd = prompt_password("Enter master password to unlock vault: ")?;
    let salt = load_salt()?;
    let master_key = derive_master_key(&master_pwd, &salt)?;

    let mut vault = load_vault(&*master_key)?;

    let initial_count = vault.items.len();
    vault.items.retain(|i| i.name != name);

    if vault.items.len() == initial_count {
        println!("Item '{}' not found in vault.", name);
        return Ok(());
    }

    print!("Are you sure you want to delete '{}'? (y/n): ", name);
    io::stdout().flush()?;
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm)?;

    if confirm.trim().eq_ignore_ascii_case("y") {
        save_vault(&vault, &*master_key)?;
        println!("Item '{}' securely deleted.", name);
    } else {
        println!("Deletion aborted.");
    }

    Ok(())
}
