use jsonwebtoken::{DecodingKey, Validation, errors::Error as JwtError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,     // Subject (user_id)
    exp: i64,        // Expiration time
    iat: i64,        // Issued at
    iss: String,     // Issuer
}

pub fn decode_jwt(token: &str, secret: &str) -> Result<String, JwtError> {
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    
    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation
    )?;

    Ok(token_data.claims.sub)
}