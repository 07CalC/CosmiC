use crate::{extra::hash_password::hash_password, state::AppState};
use axum::{Json, extract::State, response::IntoResponse};
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
) -> Result<impl IntoResponse, (axum::http::StatusCode, Json<serde_json::Value>)> {
    let count: i64 = query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db)
        .await
        .map_err(|_| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": "failed to check existing users"
                })),
            )
        })?;

    if count > 0 {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "admin user already exists"
            })),
        ));
    }

    if payload.username.is_empty() || payload.email.is_empty() || payload.password.is_empty() {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "required fields missing",
                "error": "required fields missing"
            })),
        ));
    }

    let username_exists: i64 = query_scalar("SELECT COUNT(*) FROM users WHERE username = ?")
        .bind(&payload.username)
        .fetch_one(&state.db)
        .await
        .map_err(|_| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": "failed to check username"
                })),
            )
        })?;

    if username_exists > 0 {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "username already taken"
            })),
        ));
    }

    let hashed_password = hash_password(&payload.password).unwrap();

    let now = chrono::Utc::now();

    let user_id = uuid::Uuid::new_v4().to_string();

    let db_result  = query("INSERT INTO users (id, username, email, password_hash, is_admin, role, created_at, updated_at) VALUES (?,?,?,?,?,?,?,?)"
).bind(user_id.clone())
.bind(payload.username.clone())
.bind(payload.email.clone())
.bind(hashed_password.clone())
.bind(1)
.bind("owner")
.bind(now.clone().to_string())
.bind(now.clone().to_string())
.execute(&state.db)
.await;

 match db_result {
        Ok(_) => (),
        Err(error) => {
            println!("Error creating admin user: {:?}", error);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": "failed to create admin user"
                })),
            ));
        }
    }

    println!("Admin user created with ID: {}", user_id);

    Ok(Json(json!({"success": true, "message": "Admin setup completed"})))
}
