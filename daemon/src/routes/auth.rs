use axum::middleware;
use axum::routing::post;
use axum::{Router, routing::get};

use crate::controllers::check_setup::check_setup;
use crate::controllers::login::login;
use crate::controllers::logout::logout;
// use crate::controllers::logout::logout;
use crate::controllers::me::me;
use crate::controllers::setup_admin::setup_admin;
use crate::middlewares::auth_middleware::auth_middleware;
use crate::state::AppState;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
    .route("/setup", get(check_setup).post(setup_admin))
    .route("/login", post(login))
    .route("/me", get(me).layer(middleware::from_fn(auth_middleware)))
    .route("/logout", get(logout))
}