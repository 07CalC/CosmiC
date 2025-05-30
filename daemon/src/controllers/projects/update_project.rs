use axum::{extract::{Path, State}, response::{IntoResponse, Response}, Extension, Json};
use hyper::StatusCode;
use serde::Deserialize;
use serde_json::json;
use sqlx::query;

use crate::utils::types::User;


#[derive(Deserialize)]
pub struct UpdateProjectPayload {
    pub name: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}


#[axum::debug_handler]
pub async fn update_project(
    State(state): State<crate::state::AppState>,
    Path(project_id): Path<String>,
    Extension(user): Extension<User>,
    Json(updated_project): Json<UpdateProjectPayload>
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    if project_id.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"success": false, "error": "Invalid project ID"})),
        ));
    }
    
    if !user.is_admin {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"success": false, "error": "Only admins can update projects"})),
        ));
    }

    let tags = updated_project.tags.unwrap_or_default();
    let tags_str = tags.join(",");

    let db_result = query("UPDATE projects SET name = COALESCE($1, name), description = COALESCE($2, description), tags = COALESCE($3, tags) WHERE id = $4")
        .bind(&updated_project.name)
        .bind(&updated_project.description)
        .bind(tags_str)
        .bind(&project_id)
        .execute(&state.db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"success": false, "error": "Failed to update project"})),
            )
        })?;
    if db_result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"success": false, "error": "Project not found"})),
        ));
    }
    Ok(Json(json!({"success": true, "message": "Project updated successfully"})).into_response())
    
}