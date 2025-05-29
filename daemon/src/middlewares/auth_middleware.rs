use axum::extract::State;
use axum::{body::Body, extract::Request, http, middleware::Next, response::Response};
use axum::http::StatusCode;
use serde_json::json;
use sqlx::{query, Row};

use crate::state::AppState;
use crate::utils::decode_jwt::decode_jwt;
use crate::utils::types::User;


#[axum::debug_middleware]
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next) -> Response {

    let token = req.headers()
        .get(http::header::COOKIE)
        .and_then(|cookie_header| cookie_header.to_str().ok())
        .and_then(|cookie_str| {
            cookie_str
                .split(';')
                .find(|cookie| cookie.trim().starts_with("token="))
                .map(|cookie| cookie.trim()[6..].to_string())
        });
    //    println!("{:?}", state);
    match token {
        Some(token) => {
            match decode_jwt(&token, "cosmic_secret") {
                Ok(user_id) => {
                    let row = query("SELECT id, email, username, is_admin, role FROM users WHERE id = $1")
                        .bind(&user_id)
                        .fetch_one(&state.db)
                        .await
                        .unwrap();
                    let user = User {
                        id: row.get("id"),
                        username: row.get("username"),
                        email: row.get("email"),
                        is_admin: row.get("is_admin"),
                        role: row.get("role"),
                        password_hash: None,
                        created_at: None,
                        updated_at: None,
                    };
                    req.extensions_mut().insert(user.clone());
                    next.run(req).await
                }
                Err(_) => Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(json!({
                        "success": false,
                        "message": "Invalid token",
                    }).to_string()))
                    .unwrap()
            }
        }
        None => Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({
                "success": false,
                "message": "Unauthorized: No token provided",
            }).to_string()))
            .unwrap()
    }
}