use axum::{
    Extension, Json,
    extract::State,
    http::HeaderValue,
    response::{IntoResponse, Response},
};
use bcrypt::verify;
use hyper::{
    StatusCode,
    header::{self, SET_COOKIE},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Row, query, query_scalar};

use crate::{
    extra::hash_password::hash_password,
    state::AppState,
    utils::{createJwt::create_jwt, types::User},
};

#[derive(Serialize)]
pub struct CheckSetupResponse {
    pub needs_setup: bool,
}

#[axum::debug_handler]
pub async fn check_setup(State(state): State<AppState>) -> impl IntoResponse {
    let count: i64 = query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db)
        .await
        .unwrap_or(1);

    Json(CheckSetupResponse {
        needs_setup: count == 0,
    })
}

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

    Ok(Json(
        json!({"success": true, "message": "Admin setup completed"}),
    ))
}

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
    let result = query(
        "SELECT username, id, email, is_admin, password_hash, role FROM users WHERE username = $1",
    )
    .bind(&payload.username)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(row) => {
            let username: String = row.get("username");
            let match_password = verify(&payload.password.as_bytes(), row.get("password_hash"));
            match match_password {
                Ok(true) => {}
                _ => {
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({
                            "success": false,
                            "error": "incorrect password"
                        })),
                    ));
                }
            }

            let cookie_value = create_jwt(row.get("id"), &state.jwt_secret).map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "success": false,
                        "error": "failed to create JWT"
                    })),
                )
            })?;

            let cookie = format!(
                "token={}; HttpOnly; SameSite=Strict; Path=/; Max-Age=86400",
                cookie_value
            );

            let mut response = Json(json!({
                    "success": true,
                    "message": "login successful",
                    "user":{
                    "id": row.get::<String, _>("id"),
                    "username": username,
                    "email": row.get::<String, _>("email"),
                    "role": row.get::<String, _>("role"),
                    "isAdmin": row.get::<bool, _>("is_admin"),
                    }

            }))
            .into_response();

            response.headers_mut().insert(
                SET_COOKIE,
                HeaderValue::from_str(&cookie).expect("Failed to create cookie header"),
            );

            Ok(response)
        }
        Err(_) => Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "success": false,
                "error": "no user found with the provided username"
            })),
        )),
    }
}

#[axum::debug_handler]
pub async fn logout() -> impl IntoResponse {
    let cookie = "token=; Path=/; Max-Age=0; HttpOnly; SameSite=Strict";

    let mut response = Json(json!({
        "success": true,
        "message": "Logged out successfully"
    }))
    .into_response();

    response
        .headers_mut()
        .insert(header::SET_COOKIE, HeaderValue::from_str(cookie).unwrap());

    (StatusCode::OK, response)
}

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
    }))
    .into_response()
}
