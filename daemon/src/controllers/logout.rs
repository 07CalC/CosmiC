use axum::{
    response::IntoResponse,
    http::{StatusCode, header::{self, HeaderValue}},
    Json,
};
use serde_json::json;

#[axum::debug_handler]
pub async  fn logout() -> impl IntoResponse {
    let cookie = "token=; Path=/; Max-Age=0; HttpOnly; SameSite=Strict";
    
    let mut response = Json(json!({
        "success": true,
        "message": "Logged out successfully"
    })).into_response();

    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(cookie).unwrap()
    );

    (StatusCode::OK, response)
}