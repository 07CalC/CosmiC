use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Serialize;


#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub role: String,
    pub is_admin: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}


#[derive(Debug, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: String,
    pub tags: Vec<String>,
    pub last_deployment_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum AppType {
    Custom,
    Nextjs,
    Vite,
    Django,
    Go,
    NodeJs,
}

#[derive(Debug, Clone)]
pub struct App {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub repo_url: String,
    pub branch: String,
    pub created_by_id: String,
    pub app_type: AppType,
    pub docker_file_path: Option<String>,
    pub env_vars: Option<HashMap<String, String>>,
    pub build_command: Option<String>,
    pub run_command: Option<String>,
    pub domain: Option<String>,
    pub port: Option<u16>,
    pub auto_deploy: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}


#[derive(Debug, Clone)]
pub enum DeploymentStatus {
    Queued,
    Running,
    Success,
    Failed,
    Cancelled
}

#[derive(Debug, Clone)]
pub struct DeploymentJob {
    pub id: String,
    pub project_id: String,
    pub app_id: String,
    pub repo_url: String,
    pub branch: String,
    pub commit_hash: Option<String>,
    pub status: DeploymentStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub logs_url: Option<String>
}