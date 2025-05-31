pub mod folder;
pub mod post;

use super::internal_error;
use crate::{
    database::*, extend_slug, html_template, render::markdown_to_html, schema::*, templates::*,
    AppState,
};
use axum::Extension;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;

pub use self::folder::*;
pub use self::post::*;

pub async fn get_blog(
    Extension(nonce): Extension<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let featured_posts = get_featured_posts(&state).await.map_err(internal_error)?;
    let categories = get_categories(&state).await.map_err(internal_error)?;

    dbg!(&featured_posts);

    html_template(BlogTemplate {
        nonce,
        featured_posts,
        categories,
    })
}

pub async fn post_preview(
    Extension(nonce): Extension<String>,
    State(state): State<AppState>,
    Json(post): Json<CreatePost>,
) -> Result<impl IntoResponse, StatusCode> {
    let slug = extend_slug(&post.slug, post.folder, &state)
        .await
        .map_err(internal_error)?;

    let tag_ids = post.tags.iter().map(|tag| tag.id).collect::<Vec<_>>();
    let tags = sqlx::query_as!(Tag, "SELECT * FROM tags WHERE id = ANY($1)", &tag_ids)
        .fetch_all(&state.db)
        .await
        .map_err(internal_error)?;

    let post = PostFull {
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
    };
    html_template(PreviewPostTemplate { nonce, post })
}

/// Render Markdown to HTML (returns text/plain)
pub async fn render(markdown: String) -> Result<String, String> {
    markdown_to_html(&markdown)
}
