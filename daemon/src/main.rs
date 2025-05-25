mod state;
mod utils;

#[tokio::main]
async fn main() {
    let db = sqlx::SqlitePool::connect("sqlite:///var/lib/cosmic/cosmic.db")
        .await
        .expect("Failed to connect to the database");
    let _ = sqlx::migrate!("./migrations")
    .run(&db)
    .await;
}