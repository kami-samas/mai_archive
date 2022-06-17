use argon2::{password_hash::SaltString, Argon2, PasswordHasher, Error};
use rand::rngs::OsRng;

pub fn gen_token(password: &str) -> Result<String, Error> {
    Ok(
        match Argon2::default().hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng)) {
            Ok(val) => val.to_string(),
            Err(val) => val.to_string()
        }
    )
}