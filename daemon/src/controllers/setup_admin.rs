use crate::{extra::hash_password::hash_password, state::AppState, utils::types::User};
use axum::{extract::State, response::{IntoResponse, Response}, Json};
use serde::Deserialize;
use serde_json::json;
use sqlx::{query, query_scalar};

#[derive(Deserialize)]
pub struct SetupRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[axum::debug_handler]
pub async fn setup_admin(
    State(state): State<AppState>,
    Json(payload): Json<SetupRequest>,
) -> Result<impl IntoResponse, (axum::http::StatusCode, Json<serde_json::Value>)>  {
    let count: i64 = query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db)
        .await
        .map_err(|_| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "internal server error",
                    "error": "DB error"
                })),
            )
        })?;

    if count > 0 {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "one or more admin already exists",
                "error": "admin already exists, login to add more admins"
            })),
        ));
    }
    if payload.username.is_empty() || payload.email.is_empty() || payload.password.is_empty() {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "required fields missing",
                "error": "required fields missing"
            }))
        ));
    }

    let hashed_password = hash_password(&payload.password).unwrap();

    let now = chrono::Utc::now();

    let user_id = uuid::Uuid::new_v4().to_string();

    let _ = query("INSERT INTO users (id, username, email, password_hash, is_admin, created_at, updated_at) VALUES (?,?,?,?,?,?,?)"
).bind(user_id.clone())
.bind(payload.username.clone())
.bind(payload.email.clone())
.bind(hashed_password.clone())
.bind(1)
.bind(now.clone().to_string())
.bind(now.clone().to_string())
.execute(&state.db)
.await;


    Ok(Json(
        json!({"message": "Admin setup completed"}),
    ))
}
