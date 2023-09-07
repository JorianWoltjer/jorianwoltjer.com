use axum::http::StatusCode;

pub mod auth;
pub mod blog;
pub mod middleware;
pub mod projects;

pub use self::auth::*;
pub use self::blog::*;
pub use self::middleware::*;
pub use self::projects::*;

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

/// Always return successful response when reachable
pub async fn health_check() -> StatusCode {
    StatusCode::NO_CONTENT
}
