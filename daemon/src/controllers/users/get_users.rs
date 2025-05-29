use axum::{extract::State, response::{IntoResponse, Response}, Json};
use hyper::StatusCode;
use serde_json::json;
use sqlx::{query, Row};

use crate::{state::AppState, utils::types::User};






#[axum::debug_handler]
pub async fn get_users(
    State(state): State<AppState>,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let users = query("SELECT id, username, email, is_admin, role FROM users")
        .fetch_all(&state.db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"success": false, "error": "failed to fetch users"})),
            )
        })?;

    let users: Vec<User> = users
        .into_iter()
        .map(|row| User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            is_admin: row.get("is_admin"),
            role: row.get("role"),
            password_hash: None,
            created_at: None,
            updated_at: None,
        })
        .collect();

    Ok(Json(json!({
        "success": true,
        "users": users
    })).into_response())
}