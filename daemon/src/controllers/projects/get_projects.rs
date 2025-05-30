use axum::{extract::State, response::{IntoResponse, Response}, Json};
use hyper::StatusCode;
use sqlx::{query, Row};
use serde_json::json;

use crate::{state::AppState};

#[axum::debug_handler]
pub async fn get_projects(
    State(state): State<AppState>,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let projects = query(
        "SELECT p.*, 
         u.id as owner_id, u.username as owner_username, 
         u.email as owner_email, u.is_admin as owner_is_admin,
         u.role as owner_role
         FROM projects p 
         LEFT JOIN users u ON p.owner_id = u.id"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": "Failed to fetch projects"
            }))
        )
    })?;

    let projects = projects
        .into_iter()
        .map(|row| {
            let tags_str: String = row.get("tags");
            let tags: Vec<String> = tags_str
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            let last_deployment = row.get::<Option<String>, _>("last_deployment_at")
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc));
            
            let created_at = chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now());
                
            let updated_at = chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now());

            json!({
                "id": row.get::<String, _>("id"),
                "name": row.get::<String, _>("name"),
                "description": row.get::<Option<String>, _>("description"),
                "tags": tags,
                "owner": {
                    "id": row.get::<String, _>("owner_id"),
                    "username": row.get::<String, _>("owner_username"),
                    "email": row.get::<String, _>("owner_email"),
                    "is_admin": row.get::<bool, _>("owner_is_admin"),
                    "role": row.get::<String, _>("owner_role")
                },
                "last_deployment_at": last_deployment,
                "created_at": created_at,
                "updated_at": updated_at
            })
        })
        .collect::<Vec<_>>();

    Ok(Json(json!({
        "success": true,
        "projects": projects
    })).into_response())
}