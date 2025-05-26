use axum::{
    extract::{State, Extension},
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::{query, Row};
use serde_json::json;

use crate::state::AppState;

#[axum::debug_handler]
pub async fn me(
    State(state): State<AppState>,
    Extension(user_id): Extension<String>,
) -> impl IntoResponse {
    match query("SELECT id, username, email, is_admin FROM users WHERE id = $1")
        .bind(&user_id)
        .fetch_one(&state.db)
        .await
    {
        Ok(row) => {
            Ok(Json(json!({
                "success": true,
                "user": {
                "id": row.get::<String, _>("id"),
                "username": row.get::<String, _>("username"),
                "email": row.get::<String, _>("email"),
                "isAdmin": row.get::<bool, _>("is_admin"),
                }
            })))
        }
        Err(_) => {
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({
                    "success": false,
                    "error": "user not found"
                }))
            ))
        }
    }
}