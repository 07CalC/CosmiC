use axum::{extract::State, middleware, routing::{get, post}, Router};

use crate::{controllers::users::{create_user::create_user, get_users::get_users}, middlewares::auth_middleware::auth_middleware, state::AppState};





pub fn users_routes(
    state: AppState
) -> Router<AppState> {
    Router::new()
        .route("/getusers", get(get_users).layer(middleware::from_fn_with_state(state.clone(), auth_middleware)))
        .route("/create", post(create_user).layer(middleware::from_fn_with_state(state.clone(), auth_middleware)))
}