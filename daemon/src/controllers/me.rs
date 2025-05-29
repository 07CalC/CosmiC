use axum::{
    extract::{State, Extension},
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::{query, Row};
use serde_json::json;

use crate::{state::AppState, utils::types::User};

#[axum::debug_handler]
pub async fn me(
    State(_state): State<AppState>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    Json(json!({
        "success": true,
        "user": {
            "id": user.id,
            "username": user.username,
            "email": user.email,
            "isAdmin": user.is_admin,
            "role": user.role,
        }
    })).into_response()
}