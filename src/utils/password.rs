use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

/// Hashes a plaintext password using Argon2id.
/// Returns a PHC string like `$argon2id$v=19$m=4096,t=3,p=1$...`.
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

/// Verifies a plaintext password against a stored hash.
/// Returns Ok(()) if match; Err(_) otherwise.
pub fn verify_password(password: &str, hashed: &str) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hashed)?;
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
}
