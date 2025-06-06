use axum::{Router, routing::get};
use routes::{auth::auth_routes, project::projects_router, users::users_routes, ws::ws_routes};
use services::docker::{init::docker, list_containers, list_images::list_images};
use state::AppState;
use std::net::SocketAddr;
use tower_http::{cors::{Any, CorsLayer}, services::ServeDir};
use tracing_subscriber::EnvFilter;
mod controllers;
mod extra;
mod routes;
mod state;
mod utils;
mod middlewares;
mod ws;
mod services;

#[tokio::main]
async fn main() {

    let state = AppState::new("sqlite:///var/lib/cosmic/cosmic.db").await.unwrap();

    tracing_subscriber::fmt()
    .with_env_filter(
        EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("axum_tracing_example=error,tower_http=warn"))
        .unwrap()
    )
    .init();

    let cors_layer = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);

    let static_files = ServeDir::new("./dashboard");

    let app = Router::new()
        .fallback_service(static_files)
        .route("/health", get(|| async { "Http server working fine" }))
        .nest("/api/auth", auth_routes(state.clone()))
        .nest("/api/ws", ws_routes(state.clone()))
        .nest("/api/users", users_routes(state.clone()))
        .nest("/api/projects", projects_router(state.clone()))
        .with_state(state)
        .layer(cors_layer);

    let addr = SocketAddr::from(([0, 0, 0, 0], 4269));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("\n axum server running on port 4269 \n");

    // list_images().await;

    axum::serve(listener, app).await.unwrap()
}
