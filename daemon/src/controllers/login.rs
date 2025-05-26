use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
    http::{StatusCode, header::{SET_COOKIE, HeaderValue}},
};
use bcrypt::verify;
use serde::Deserialize;
use serde_json::json;
use sqlx::{Row, query};
use time::{Duration, OffsetDateTime};

use crate::{
    state::{self, AppState},
    utils::{createJwt::create_jwt, types::User},
};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let result = query("SELECT username, id, email, is_admin, password_hash FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_one(&state.db)
        .await;

    match result {
        Ok(row) => {
            let username: String = row.get("username");
            let match_password = verify(&payload.password.as_bytes(), row.get("password_hash"));
            match match_password {
                Ok(true) => {},
                _ => {
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({
                            "success": false,
                            "error": "incorrect password"
                        }))
                    ));
                }
                
            } 

            let cookie_value = create_jwt(row.get("id"), &state.jwt_secret)
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({
                            "success": false,
                            "error": "failed to create JWT"
                        }))
                    )
                })?;

            let cookie = format!(
                "token={}; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=86400; Domain=localhost",
                cookie_value
            );

            let mut response = Json(json!({
                    "success": true,
                    "message": "login successful",
                    "user":{
                    "id": row.get::<String, _>("id"),
                    "username": username,
                    "email": row.get::<String, _>("email"),
                    "isAdmin": row.get::<bool, _>("is_admin"),
                    }
                
            })).into_response();

            response.headers_mut().insert(
                SET_COOKIE,
                HeaderValue::from_str(&cookie).expect("Failed to create cookie header")
            );

            Ok(response)
        }
        Err(_) => {
            Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "success": false,
                    "error": "no user found with the provided username"
                }))
            ))
        }
    }
}
