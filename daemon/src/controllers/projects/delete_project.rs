use axum::{extract::{Path, State}, response::{IntoResponse, Response}, Extension, Json};
use hyper::StatusCode;
use serde_json::json;
use sqlx::query;

use crate::{state::AppState, utils::types::User};





#[axum::debug_handler]
pub async fn delete_project(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(project_id): Path<String>,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {

    if !user.is_admin {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"success": false, "error": "Only admins can delete projects"})),
        ));
    }
    if project_id.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"success": false, "error": "Invalid project ID"})),
        ));
    }

    let result = query("DELETE FROM projects WHERE id = ?")
        .bind(project_id)
        .execute(&state.db)
        .await
        .map_err(|_| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"success": false, "error": "failed to delete project"})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            axum::http::StatusCode::NOT_FOUND,
            Json(json!({"success": false, "error": "project not found"})),
        ));
    }

    Ok(Json(json!({"success": true, "message": "project deleted successfully"})).into_response())
}