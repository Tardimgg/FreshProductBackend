use argon2::{password_hash::{
    rand_core::OsRng,
    PasswordHash, PasswordHasher, PasswordVerifier, SaltString
}, Argon2};
use crate::hashing_api::VerifyPasswordErr::{InternalError, InvalidPassword};

pub fn hashing_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();

    let password_hash = match argon.hash_password(password.as_bytes(), &salt) {
        Ok(v) => { v.to_string() }
        Err(v) => { v.to_string() }
    };

    // println!("password = {}\n salt = {}", password_hash, salt.to_string());
    Ok(password_hash)

}

pub enum VerifyPasswordErr {

    InvalidPassword(String),
    InternalError(String)

}

pub fn verify_password(password: &str, password_hash: &str) -> Result<(), VerifyPasswordErr> {

    let parsed_hash = match PasswordHash::new(&password_hash) {
        Ok( v ) => { v }
        Err(v) => { return Err(InternalError(v.to_string())) }
    };

    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => { Ok(()) }
        Err(v) => { Err(InvalidPassword(v.to_string())) }
    }
}
