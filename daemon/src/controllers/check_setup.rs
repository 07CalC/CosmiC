use axum::{extract::State, response::IntoResponse, Json};
use sqlx::query_scalar;
use serde::Serialize;

use crate::state::{self, AppState};


#[derive(Serialize)]
pub struct check_setup_response {
    pub needs_setup : bool
}


#[axum::debug_handler]
pub async fn check_setup (State(state): State<AppState>) -> impl IntoResponse {
    let count: i64 = query_scalar("SELECT COUNT(*) FROM users").fetch_one(&state.db)
    .await
    .unwrap_or(1);

    Json(check_setup_response {
        needs_setup: count == 0
    })
}