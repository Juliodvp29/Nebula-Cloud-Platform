use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("invalid password hash")]
    InvalidHash,
    #[error("argon2 error: {0}")]
    Argon2(String),
    #[error("password hash error: {0}")]
    PasswordHash(String),
    #[error("password verification failed")]
    VerificationFailed,
}

pub fn hash_password(password: &str) -> Result<String, PasswordError> {
    use argon2::password_hash::{SaltString, rand_core::OsRng};
    use argon2::{Argon2, PasswordHasher};

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| PasswordError::Argon2(e.to_string()))?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<(), PasswordError> {
    use argon2::PasswordHash;
    use argon2::{Argon2, PasswordVerifier};

    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| PasswordError::PasswordHash(e.to_string()))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|e| PasswordError::PasswordHash(e.to_string()))?;
    Ok(())
}
