use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};

use crate::utils::types::DeploymentJob;

const DEPLOYMENT_QUEUE_CAPACITY: usize = 5;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub jwt_secret: String,
    pub deployment_tx: mpsc::Sender<DeploymentJob>,
    pub deployment_rx: Arc<Mutex<mpsc::Receiver<DeploymentJob>>>,
}

impl AppState {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let db = SqlitePool::connect(db_url).await?;
        let _ = sqlx::migrate!("./migrations").run(&db).await;
        let (deployment_tx, deployment_rx) =
            mpsc::channel::<DeploymentJob>(DEPLOYMENT_QUEUE_CAPACITY);

        Ok(Self {
            db,
            jwt_secret: "cosmic_secret".to_string(),
            deployment_tx,
            deployment_rx: Arc::new(Mutex::new(deployment_rx)),
        })
    }
}
