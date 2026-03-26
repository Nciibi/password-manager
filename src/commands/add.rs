use crate::crypto::kdf::derive_master_key;
use crate::security::memory::SecureString;
use crate::vault::model::Item;
use crate::vault::storage::{load_salt, load_vault, save_vault, vault_exists};
use chrono::Utc;
use rpassword::prompt_password;
use std::io::{self, Write};
use uuid::Uuid;

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    if !vault_exists() {
        println!("Vault not found. Please run `pm init` first.");
        return Ok(());
    }

    let master_pwd = prompt_password("Enter master password to unlock vault: ")?;
    let salt = load_salt()?;
    let master_key = derive_master_key(&master_pwd, &salt)?;

    let mut vault = load_vault(&*master_key)?;

    print!("Name (e.g., github): ");
    io::stdout().flush()?;
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim().to_string();

    if vault.items.iter().any(|i| i.name == name) {
        println!("Item with name '{}' already exists.", name);
        return Ok(());
    }

    print!("Username / Email: ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();

    let password = prompt_password("Password: ")?;
    let confirm = prompt_password("Confirm Password: ")?;

    if password != confirm {
        println!("Passwords do not match. Aborting.");
        return Ok(());
    }

    let _secure_pwd = SecureString::new(password.clone());

    let item = Item {
        id: Uuid::new_v4().to_string(),
        name: name.clone(),
        username,
        password,
        notes: String::new(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    vault.items.push(item);

    save_vault(&vault, &*master_key)?;

    println!("Item '{}' securely added to the vault.", name);
    Ok(())
}
