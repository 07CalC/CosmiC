use axum::{Router, routing::get};
use routes::auth::auth_routes;
use state::AppState;
use std::net::SocketAddr;
use tower_http::{cors::{Any, CorsLayer}};
use tracing_subscriber::EnvFilter;
mod controllers;
mod extra;
mod routes;
mod state;
mod utils;
mod middlewares;

#[tokio::main]
async fn main() {

    let state = AppState::new("sqlite:///var/lib/cosmic/cosmic.db").await;

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

    let app = Router::new()
        .route("/health", get(|| async { "Http server working fine" }))
        .nest("/api/auth", auth_routes())
        .with_state(state.unwrap())
        .layer(cors_layer);

    let addr = SocketAddr::from(([0, 0, 0, 0], 4269));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("\n axum server running on port 4269 \n");
    axum::serve(listener, app).await.unwrap()
}
