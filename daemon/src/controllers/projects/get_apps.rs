use axum::{
    extract::{Path, State}, response::{IntoResponse, Response}, Extension, Json
};
use hyper::StatusCode;
use serde_json::json;
use sqlx::{Row, query};

use crate::{
    state::AppState,
    utils::types::{Project, User},
};

#[axum::debug_handler]
pub async fn get_apps(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let project = query(
        "SELECT p.*, 
         u.id as owner_id, u.username as owner_username, 
         u.email as owner_email, u.is_admin as owner_is_admin,
         u.role as owner_role
         FROM projects p 
         LEFT JOIN users u ON p.owner_id = u.id
         WHERE p.id = $1"
    )
    .bind(&project_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": "Failed to fetch project details"
            }))
        )
    })?;

    let project = match project {
        Some(row) => {
            let tags_str: String = row.get("tags");
            let tags: Vec<String> = tags_str
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

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
                "created_at": row.get::<String, _>("created_at"),
                "updated_at": row.get::<String, _>("updated_at")
            })
        }
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({
                    "success": false,
                    "error": "Project not found"
                }))
            ));
        }
    };

    let apps = query(
        "SELECT a.id, a.name, a.repo_url, a.branch, a.app_type, a.domain, a.port, a.updated_at,
         u.id as creator_id, u.username as creator_username, 
         u.email as creator_email, u.is_admin as creator_is_admin,
         u.role as creator_role
         FROM apps a
         LEFT JOIN users u ON a.created_by_id = u.id
         WHERE a.project_id = $1"
    )
    .bind(&project_id)
    .fetch_all(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": "Failed to fetch apps"
            }))
        )
    })?;

    let apps = apps
        .into_iter()
        .map(|row| {
            json!({
                "id": row.get::<String, _>("id"),
                "name": row.get::<String, _>("name"),
                "repoUrl": row.get::<String, _>("repo_url"),
                "branch": row.get::<String, _>("branch"),
                "app_type": row.get::<String, _>("app_type"),
                "domain": row.get::<Option<String>, _>("domain"),
                "port": row.get::<Option<i32>, _>("port"),
                "createdBy": {
                    "id": row.get::<String, _>("creator_id"),
                    "username": row.get::<String, _>("creator_username"),
                    "email": row.get::<String, _>("creator_email"),
                    "is_admin": row.get::<bool, _>("creator_is_admin"),
                    "role": row.get::<String, _>("creator_role")
                },
                "updated_at": row.get::<String, _>("updated_at")
            })
        })
        .collect::<Vec<_>>();

    Ok(Json(json!({
        "success": true,
        "project": project,
        "apps": apps
    })).into_response())
}
