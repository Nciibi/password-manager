use argon2::{Algorithm, Argon2, Params, Version};
use rand::{rngs::OsRng, RngCore};
use sha2::Sha256;
use zeroize::Zeroizing;

pub fn generate_salt() -> [u8; 32] {
    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);
    salt
}

#[derive(Debug)]
pub struct KdfError(String);

impl std::fmt::Display for KdfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KDF Error: {}", self.0)
    }
}

impl std::error::Error for KdfError {}

pub fn derive_master_key(
    password: &str,
    salt: &[u8],
) -> Result<Zeroizing<[u8; 32]>, KdfError> {
    // 64 MB (65536 KB), 3 iterations, parallelism 1.
    // Length is 32 bytes for ChaCha20Poly1305.
    let params = Params::new(65536, 3, 1, Some(32)).map_err(|e| KdfError(e.to_string()))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut master_key = Zeroizing::new([0u8; 32]);
    match argon2.hash_password_into(password.as_bytes(), salt, &mut *master_key) {
        Ok(_) => Ok(master_key),
        Err(e) => Err(KdfError(e.to_string())),
    }
}

pub fn derive_encryption_key(master_key: &[u8]) -> Zeroizing<[u8; 32]> {
    let hk = hkdf::Hkdf::<Sha256>::new(None, master_key);
    let mut ek = Zeroizing::new([0u8; 32]);
    hk.expand(b"encryption", &mut *ek)
        .expect("HKDF expansion should not fail for 32 bytes");
    ek
}
