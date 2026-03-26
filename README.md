<div align="center">
  <h1>🔐 pm — Zero-Knowledge CLI Password Manager</h1>
  <p>A fast, secure, and offline-first password manager written in Rust.</p>
  
  <p>
    <img alt="Rust" src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />
    <img alt="Linux" src="https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black" />
    <img alt="Security" src="https://img.shields.io/badge/Security-AEAD_|_Argon2-green?style=for-the-badge" />
  </p>
</div>

## ✨ Features

- **Zero-Knowledge Architecture:** Your master password never leaves your machine. 
- **Offline-First:** All data is stored locally in an encrypted vault. No cloud dependencies, no servers to be breached.
- **Strong Cryptography:** 
  - Key Derivation: **Argon2id** (64MB memory, 3 iterations)
  - Key Separation: **HKDF-SHA256**
  - Encryption: **ChaCha20-Poly1305** (AEAD)
- **Secure Memory Management:** Uses the `zeroize` crate to securely wipe secrets from RAM as soon as they are no longer needed.
- **Clipboard Management:** Automatically clears copied passwords from your clipboard after an adjustable timeout (default 15s).
- **Lightweight & Fast:** Written in Rust with minimal dependencies, compiling to a single executable binary.

## 🚀 Installation

Ensure you have [Rust and Cargo](https://rustup.rs/) installed, then build the project:

```bash
git clone https://github.com/yourusername/password-manager.git
cd password-manager
cargo build --release
```

The compiled binary will be available at `./target/release/password_manager`. You can move it to your `$PATH` for easy execution:

```bash
sudo mv ./target/release/password_manager /usr/local/bin/pm
```

## 📚 Usage

### 1. Initialize the Vault
Sets up your vault and master password. A strong cryptographic salt is generated and saved alongside your new encrypted vault in `~/.password_manager/`.

```bash
pm init
```

### 2. Add a Password
Securely prompts you for a new entry (name, username, and password).

```bash
pm add
# Prompts:
# Name (e.g., github): 
# Username / Email: 
# Password: 
# Confirm Password: 
```

### 3. Get a Password
Fetches the password from your vault and automatically copies it to your clipboard.

```bash
pm get github
# Output: Password for 'github' copied to clipboard. It will clear in 15 seconds.
```

### 4. List Vault Entries
Displays a table of all your saved accounts (without revealing the passwords).

```bash
pm list
```

### 5. Delete an Entry
Removes an item from your vault.

```bash
pm delete github
```

### 6. Generate a Strong Password
Generates a random, cryptographically secure password and lets you instantly copy it.

```bash
pm generate --length 24
pm generate --no-symbols
```

## 🔒 Security Model

### Threat Model
**Protects against:**
- Physical theft of your hard drive (Vault is encrypted at rest).
- Stolen vault files (Argon2id KDF prevents brute-forcing).
- Shoulder surfing (Passwords are never printed to the terminal, only securely copied to the clipboard).
- Memory scraping (Keys and plaintexts are zeroized eagerly on drop).

**Does NOT protect against:**
- Active keyloggers.
- Root-level OS compromise (Endpoint security is the weakest link).

### Cryptographic Details
1. **Master Key Generation:** Uses `Argon2id` spanning a randomly generated 32-byte salt and the master password.
2. **Encryption Key Derivation:** Uses `HKDF-SHA256` to expand the master key.
3. **Data Encryption:** The vault is serialized to JSON and encrypted using `ChaCha20-Poly1305` with a random 96-bit nonce for every save operation.

## 🤝 Contributing
Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/yourusername/password-manager/issues).

