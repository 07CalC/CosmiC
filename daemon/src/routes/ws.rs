use axum::{routing::get, Router};

use crate::middlewares::auth_middleware::auth_middleware;
use crate::state::AppState;
use crate::ws::stats::stats_ws_handler;
use crate::ws::terminal::terminal_ws_handler;

pub fn ws_routes() -> Router<AppState> {
    Router::new()
    .route("/stats",get(super::ws::stats_ws_handler).layer(axum::middleware::from_fn(auth_middleware)))
    .route("/terminal", get(super::ws::terminal_ws_handler))
}