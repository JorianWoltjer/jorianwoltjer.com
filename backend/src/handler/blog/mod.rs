pub mod folder;
pub mod post;

use super::{internal_error, MiddlewareData};
use crate::render::markdown_to_html;
use crate::{database::*, extend_slug, html_template, schema::*, templates::*, AppState};
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum::{Extension, Form};
use chrono::Utc;
use serde::Deserialize;

pub use self::folder::*;
pub use self::post::*;

#[derive(Deserialize)]
pub struct ParentParam {
    pub parent: Option<i32>,
}

pub async fn get_blog(
    Extension(metadata): Extension<MiddlewareData>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let featured_posts = get_featured_posts(&state).await.map_err(internal_error)?;
    let categories = get_categories(&state).await.map_err(internal_error)?;

    dbg!(&featured_posts);

    html_template(BlogTemplate {
        metadata,
        featured_posts,
        categories,
    })
}

#[derive(Deserialize)]
pub struct PreviewJson {
    pub json: String,
}

pub async fn post_preview(
    Extension(metadata): Extension<MiddlewareData>,
    State(state): State<AppState>,
    Form(form): Form<PreviewJson>,
) -> Result<impl IntoResponse, StatusCode> {
    // Send JSON as form data because browser can't send it top-level otherwise
    let post: CreatePost = serde_json::from_str(&form.json).map_err(internal_error)?;

    let slug = extend_slug(&post.slug, post.folder, &state)
        .await
        .map_err(internal_error)?;

    let html = markdown_to_html(&post.markdown).map_err(internal_error)?;

    let tags = sqlx::query_as!(Tag, "SELECT * FROM tags WHERE id = ANY($1)", &post.tags)
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
        html,
        points: post.points,
        views: 0,
        featured: post.featured,
        hidden: post.hidden,
        autorelease: post.autorelease,
        timestamp: Utc::now(),
        tags,
    };
    html_template(PostTemplate { metadata, post })
}
