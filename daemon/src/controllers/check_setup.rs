use axum::{Json, extract::State, response::IntoResponse};
use serde::Serialize;
use sqlx::query_scalar;

use crate::state::AppState;

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
