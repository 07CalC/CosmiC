use axum::{routing::get, Router};
use extra::hash_password::hash_password;
use routes::auth::auth_routes;
use state::AppState;
use std::net::SocketAddr;

mod state;
mod utils;
mod routes;
mod controllers;
mod extra;

#[tokio::main]
async fn main() {
    
    let state = AppState::new("sqlite:///var/lib/cosmic/cosmic.db").await;

    let app = Router::new()
    .route("/health", get(|| async {"Http server working fine"}))
    .nest("/api/auth", auth_routes())
    .with_state(state.unwrap());

    let addr = SocketAddr::from(([0,0,0,0], 4269));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("\n axum server running on port 4269 \n");
    axum::serve(listener, app).await.unwrap()

}