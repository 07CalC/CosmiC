use bcrypt::{hash, DEFAULT_COST};

extern crate bcrypt;


pub fn hash_password(pass: &str) -> Result<String, bcrypt::BcryptError> {
    let hashed_password = hash(pass.as_bytes(), DEFAULT_COST).unwrap();
    Ok(hashed_password)
}