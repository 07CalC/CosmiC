use axum::middleware;
use axum::routing::{delete, post};
use axum::{Router, routing::get};
use crate::controllers::projects::create_project::create_project;
use crate::controllers::projects::get_apps::get_apps;
use crate::middlewares::auth_middleware::auth_middleware;
use crate::state::AppState;
use crate::controllers::projects::get_projects::get_projects;
use crate::controllers::projects::delete_project::delete_project;



pub fn projects_router(
    state: AppState
) -> Router<AppState> {
    Router::new()
    .route("/getprojects", get(get_projects).layer(middleware::from_fn_with_state(state.clone(), auth_middleware)))
    .route("/create", post(create_project).layer(middleware::from_fn_with_state(state.clone(), auth_middleware)))
    .route("/{project_id}", get(get_apps).layer(middleware::from_fn_with_state(state.clone(), auth_middleware)))
    .route("/{project_id}", delete(delete_project).layer(middleware::from_fn_with_state(state.clone(), auth_middleware)))
}