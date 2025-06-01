pub mod auth;
pub mod blog;
pub mod middleware;

use crate::{database, html_template, templates::*, AppState};
use axum::Extension;
use axum::{extract::State, http::StatusCode, response::IntoResponse};

pub use self::auth::*;
pub use self::blog::*;
pub use self::middleware::*;

pub fn internal_error(e: impl std::fmt::Display) -> StatusCode {
    eprintln!("500 Internal Server Error: {}", e);
    StatusCode::INTERNAL_SERVER_ERROR
}

pub fn sql_not_found(e: sqlx::Error) -> StatusCode {
    match e {
        sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
        _ => internal_error(e),
    }
}

pub async fn get_home(Extension(metadata): Extension<MiddlewareData>) -> impl IntoResponse {
    html_template(HomeTemplate { metadata })
}

// TODO: fetch from github? maybe in a script
pub async fn get_projects(
    Extension(metadata): Extension<MiddlewareData>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let projects = database::get_projects(&state)
        .await
        .map_err(internal_error)?;

    html_template(ProjectsTemplate { metadata, projects })
}

pub async fn get_contact(Extension(metadata): Extension<MiddlewareData>) -> impl IntoResponse {
    html_template(ContactTemplate { metadata })
}
