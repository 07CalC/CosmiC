use axum::{extract::State, response::{IntoResponse, Response}, Extension, Json};
use hyper::StatusCode;
use serde::Deserialize;
use serde_json::json;
use sqlx::{query, query_scalar};

use crate::{state::AppState, utils::types::User};


#[derive(Deserialize)]
pub struct CreateProjectPayload {
    pub name: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}


#[axum::debug_handler]
pub async fn create_project(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateProjectPayload>
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {

    if !user.is_admin {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "success": false,
                "error": "Only admins can create projects"
            })),
        ));
        
    }

    if payload.name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "Project name cannot be empty"
            })),
        ));
    }

    let name_exists: i64 = query_scalar(
        "SELECT COUNT(*) FROM projects WHERE name = $1"
    )
    .bind(&payload.name)
    .fetch_one(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": "Failed to check project name"
            }))
        )
    })?;
    if name_exists > 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "Project name already exists"
            })),
        ));
    }

    let project_id = uuid::Uuid::new_v4().to_string();
    let tags = payload.tags.unwrap_or_default();
    let tags_str = tags.join(",");
    let now = chrono::Utc::now();
    
    let db_result = query("
        INSERT INTO projects (id, name, description, tags, owner_id, last_deployment_at, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(project_id.clone())
        .bind(&payload.name)
        .bind(&payload.description)
        .bind(&tags_str)
        .bind(&user.id)
        .bind(now.to_string())
        .bind(now.to_string())
        .bind(now.to_string())
        .execute(&state.db)
        .await;
    match db_result {
        Ok(_) => {
            let response = json!({
                "success": true,
                "message": "Project created successfully",
            });
            Ok((StatusCode::CREATED, Json(response)).into_response())
        },
        Err(error) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("Failed to create project: {}", error)
                }))
            ))
        }
    }
}