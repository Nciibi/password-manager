use crate::crypto::kdf::{derive_master_key, generate_salt};
use crate::vault::model::Vault;
use crate::vault::storage::{save_salt, save_vault, vault_exists};
use rpassword::prompt_password;

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    if vault_exists() {
        println!("Vault already exists. Run `pm add` or `pm get`.");
        return Ok(());
    }

    let password = prompt_password("Enter new master password: ")?;
    let confirm = prompt_password("Confirm master password: ")?;

    if password != confirm {
        println!("Passwords do not match. Aborting.");
        return Ok(());
    }

    if password.len() < 8 {
        println!("Password is too short. Please use at least 8 characters.");
        return Ok(());
    }

    println!("Initializing vault. This may take a few seconds due to Argon2id key derivation...");

    let salt = generate_salt();
    let master_key = derive_master_key(&password, &salt)?;

    let vault = Vault {
        version: 1,
        items: Vec::new(),
    };

    save_salt(&salt)?;
    save_vault(&vault, &*master_key)?;

    println!("Vault successfully initialized at ~/.password_manager/vault.json.enc");
    Ok(())
}
