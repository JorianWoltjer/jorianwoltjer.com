pub mod auth;
pub mod blog;
pub mod middleware;

use crate::{html_template, templates::*};
use axum::Extension;
use axum::{http::StatusCode, response::IntoResponse};

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

pub async fn get_about(Extension(metadata): Extension<MiddlewareData>) -> impl IntoResponse {
    html_template(AboutTemplate { metadata })
}

pub async fn get_contact(Extension(metadata): Extension<MiddlewareData>) -> impl IntoResponse {
    html_template(ContactTemplate { metadata })
}
