pub mod folder;
pub mod post;

use axum::extract::State;
use axum::Json;
use chrono::Utc;
use reqwest::StatusCode;

use crate::extend_slug;
use crate::render::markdown_to_html;
use crate::schema::CreatePost;
use crate::schema::PostFull;
use crate::schema::Tag;
use crate::AppState;

pub use self::folder::*;
pub use self::post::*;

use super::internal_error;

pub async fn preview(
    State(state): State<AppState>,
    Json(post): Json<CreatePost>,
) -> Result<Json<PostFull>, StatusCode> {
    let slug = extend_slug(&post.slug, post.folder, &state)
        .await
        .map_err(internal_error)?;

    let tag_ids = post.tags.iter().map(|tag| tag.id).collect::<Vec<_>>();
    let tags = sqlx::query_as!(Tag, "SELECT * FROM tags WHERE id = ANY($1)", &tag_ids)
        .fetch_all(&state.db)
        .await
        .map_err(internal_error)?;

    Ok(Json(PostFull {
        id: 0,
        folder: post.folder,
        slug,
        title: post.title,
        description: post.description,
        img: post.img,
        markdown: post.markdown,
        points: post.points,
        views: 0,
        featured: post.featured,
        hidden: post.hidden,
        autorelease: post.autorelease,
        timestamp: Utc::now(),
        tags,
    }))
}

/// Render Markdown to HTML (returns text/plain)
pub async fn render(markdown: String) -> Result<String, String> {
    markdown_to_html(&markdown)
}
