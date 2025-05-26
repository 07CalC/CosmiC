use axum::{body::Body, extract::Request, http, middleware::Next, response::Response};
use axum::http::StatusCode;
use serde_json::json;

use crate::utils::decode_jwt::decode_jwt;

#[derive(Debug)]
struct AuthError {
    message: String,
    status_code: StatusCode,
}

#[axum::debug_middleware]
pub async fn auth_middleware(mut req: Request, next: Next) -> Response {
    let token = req.headers()
        .get(http::header::COOKIE)
        .and_then(|cookie_header| cookie_header.to_str().ok())
        .and_then(|cookie_str| {
            cookie_str
                .split(';')
                .find(|cookie| cookie.trim().starts_with("token="))
                .map(|cookie| cookie.trim()[6..].to_string())
        });

    match token {
        Some(token) => {
            match decode_jwt(&token, "cosmic_secret") {
                Ok(user_id) => {
                    req.extensions_mut().insert(user_id.clone());
                    next.run(req).await
                }
                Err(_) => Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(json!({
                        "message": "Invalid authentication",
                        "error": "Invalid or expired token"
                    }).to_string()))
                    .unwrap()
            }
        }
        None => Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({
                "message": "Authentication required",
                "error": "No token cookie found"
            }).to_string()))
            .unwrap()
    }
}