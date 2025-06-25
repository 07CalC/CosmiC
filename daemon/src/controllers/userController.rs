use axum::{
    Extension, Json,
    extract::State,
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
use serde::Deserialize;
use serde_json::json;
use sqlx::{Row, query, query_scalar};

use crate::{extra::hash_password::hash_password, state::AppState, utils::types::User};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
}

#[axum::debug_handler]
pub async fn create_user(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let username_exists: i64 = query_scalar("SELECT COUNT(*) FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_one(&state.db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": "failed to check username"
                })),
            )
        })?;

    if username_exists > 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "username already taken"
            })),
        ));
    }

    if payload.is_admin && !user.role.contains("owner") {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "success": false,
                "error": "only owners can create admin users"
            })),
        ));
    }

    let password_hash = hash_password(&payload.password).unwrap();
    let now = chrono::Utc::now();

    let user_id = uuid::Uuid::new_v4().to_string();

    let role = if payload.is_admin {
        "admin".to_string()
    } else {
        "user".to_string()
    };

    let db_result  = query("INSERT INTO users (id, username, email, password_hash, is_admin, role, created_at, updated_at) VALUES (?,?,?,?,?,?,?,?)"
).bind(user_id.clone())
.bind(payload.username.clone())
.bind(payload.email.clone())
.bind(password_hash.clone())
.bind(if payload.is_admin { 1 } else { 0 })
.bind(role)
.bind(now.clone().to_string())
.bind(now.clone().to_string())
.execute(&state.db)
.await;

    match db_result {
        Ok(_) => (),
        Err(error) => {
            println!("Error creating user: {:?}", error);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": "failed to create user"
                })),
            ));
        }
    }

    Ok(Json(json!({
        "success": true,
        "message": "user created successfully",
    }))
    .into_response())
}

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
    }))
    .into_response())
}
