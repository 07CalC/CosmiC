use axum::{routing::get, Router};

use crate::controllers::check_setup::check_setup;
use crate::controllers::setup_admin::setup_admin;
use crate::state::AppState;


pub fn auth_routes () -> Router<AppState> {
    Router::new()
    .route("/setup", get(check_setup).post(setup_admin))
}