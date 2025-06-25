use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::middlewares::auth_middleware::auth_middleware;
use crate::{
    controllers::userController::{create_user, get_users},
    state::AppState,
};

pub fn users_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/getusers",
            get(get_users).layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/create",
            post(create_user).layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
}

