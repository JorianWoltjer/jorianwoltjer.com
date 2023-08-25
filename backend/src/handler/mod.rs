use axum::http::StatusCode;

use crate::render::markdown_to_html;

pub mod auth;
pub mod folder;
pub mod middleware;
pub mod post;

pub use self::auth::*;
pub use self::folder::*;
pub use self::middleware::*;
pub use self::post::*;

/// Always return successful response when reachable
pub async fn health_check() -> StatusCode {
    StatusCode::NO_CONTENT
}

/// Render Markdown to HTML (returns text/plain)
pub async fn render(markdown: String) -> Result<String, String> {
    markdown_to_html(&markdown)
}
