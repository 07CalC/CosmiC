use axum::{routing::get, Router};

use crate::state::AppState;
use crate::ws::stats::stats_ws_handler;


pub fn ws_routes() -> Router<AppState> {
    Router::new()
    .route("/stats",get(super::ws::stats_ws_handler))
}