use rand::seq::SliceRandom;
use rand::rngs::OsRng;
use crate::security::clipboard::copy_and_clear;
use std::io::{self, Write};

pub fn execute(length: usize, no_symbols: bool) -> Result<(), Box<dyn std::error::Error>> {
    let lowercase = b"abcdefghijklmnopqrstuvwxyz";
    let uppercase = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = b"0123456789";
    let symbols = b"!@#$%^&*()_+-=[]{}|;:,.<>?";

    let mut charset = Vec::new();
    charset.extend_from_slice(lowercase);
    charset.extend_from_slice(uppercase);
    charset.extend_from_slice(numbers);
    
    if !no_symbols {
        charset.extend_from_slice(symbols);
    }

    let mut rng = OsRng;
    let mut password_bytes = Vec::with_capacity(length);

    // Ensure at least one of each (basic constraints)
    password_bytes.push(*lowercase.choose(&mut rng).unwrap());
    password_bytes.push(*uppercase.choose(&mut rng).unwrap());
    password_bytes.push(*numbers.choose(&mut rng).unwrap());
    if !no_symbols {
        password_bytes.push(*symbols.choose(&mut rng).unwrap());
    }

    while password_bytes.len() < length {
        password_bytes.push(*charset.choose(&mut rng).unwrap());
    }

    password_bytes.shuffle(&mut rng);

    let password = String::from_utf8(password_bytes)?;
    println!("Generated Password: {}", password);

    print!("Copy to clipboard? (y/n) [y]: ");
    io::stdout().flush()?;
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm)?;
    
    let confirm = confirm.trim();
    if confirm.is_empty() || confirm.eq_ignore_ascii_case("y") {
        copy_and_clear(password, 20)?;
        println!("Copied to clipboard. Will clear in 20 seconds.");
    }

    Ok(())
}
