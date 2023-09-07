use axum::{extract::State, http::StatusCode, response::Json};

use crate::{schema::*, AppState};

use super::internal_error;

pub async fn get_projects(State(state): State<AppState>) -> Result<Json<Vec<Project>>, StatusCode> {
    let projects = sqlx::query_as!(Project, "SELECT * FROM projects")
        .fetch_all(&state.db)
        .await
        .map_err(internal_error)?;

    Ok(Json(projects))
}
