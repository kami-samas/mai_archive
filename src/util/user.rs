use argon2::{password_hash::SaltString, Argon2, PasswordHasher, Error, PasswordHash, PasswordVerifier};
use rand::rngs::OsRng;
use crate::models::USERNAME_REGEX;

pub fn generate_token(password: &str) -> Result<String, Error> {
    Ok(
        match Argon2::default().hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng)) {
            Ok(val) => val.to_string(),
            Err(val) => val.to_string()
        }
    )
}

pub fn validate_token(password: &str, hash: &str) -> bool {
    Argon2::default().verify_password(password.as_bytes(), &PasswordHash::new(hash).unwrap()).is_ok()
}

pub fn validate_username(username: &str) -> Result<String, String> {
    let length = username.len();
    if length < 3 || length > 20 {
        return Err("Username must be between 3 and 20 characters long".to_string());
    }

    if !USERNAME_REGEX.is_match(username) {
        return Err("Invalid Username. Remove special characters and try again".to_string());
    }
    Ok(username.to_string())
}