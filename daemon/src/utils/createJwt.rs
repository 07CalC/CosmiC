pub fn create_jwt(user_id: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    use jsonwebtoken::{EncodingKey, Header, encode};
    use chrono::{Utc, Duration};

    #[derive(serde::Serialize)]
    struct Claims {
        sub: String,     
        exp: i64,       
        iat: i64,        
        iss: String,     
    }

    let now = Utc::now();
    let claims = Claims {
        sub: user_id.to_string(),
        exp: (now + Duration::hours(24)).timestamp(), // Token expires in 24 hours
        iat: now.timestamp(),
        iss: "cosmic_daemon".to_string(),
    };

    let header = Header::default();
    let encoding_key = EncodingKey::from_secret(secret.as_ref());

    encode(&header, &claims, &encoding_key)
}
