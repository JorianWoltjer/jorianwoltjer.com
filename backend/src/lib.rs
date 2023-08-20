use reqwest::StatusCode;
use sqlx::MySqlPool;

pub mod handler;
pub mod schema;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
}

pub fn internal_error(e: impl std::fmt::Display) -> StatusCode {
    eprintln!("Internal server error: {}", e);
    StatusCode::INTERNAL_SERVER_ERROR
}

/// Revalidate a path in NextJS static pages
pub async fn revalidate(path: &str) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .post("http://frontend/api/revalidate")
        .form(&[("path", path)])
        .send()
        .await?;

    Ok(())
}
