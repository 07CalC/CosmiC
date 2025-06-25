use axum::extract::State;
use axum::middleware;
use axum::routing::post;
use axum::{Router, routing::get};

use crate::controllers::authController::{check_setup, login, logout, me, setup_admin};
use crate::middlewares::auth_middleware::auth_middleware;
use crate::state::AppState;

pub fn auth_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/setup", get(check_setup).post(setup_admin))
        .route("/login", post(login))
        .route(
            "/me",
            get(me).layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route("/logout", get(logout))
}

