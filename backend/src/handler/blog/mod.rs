pub mod folder;
pub mod post;

use axum::extract::State;
use axum::Json;
use chrono::Utc;
use reqwest::StatusCode;

use crate::build_slug;
use crate::schema::CreatePost;
use crate::schema::Post;
use crate::AppState;

pub use self::folder::*;
pub use self::post::*;

use super::internal_error;

pub async fn preview(
    State(state): State<AppState>,
    Json(post): Json<CreatePost>,
) -> Result<Json<Post>, StatusCode> {
    let slug = build_slug(post.folder, &post.title, &state)
        .await
        .map_err(internal_error)?;

    Ok(Json(Post {
        id: 0,
        folder: post.folder,
        slug,
        title: post.title,
        description: post.description,
        img: post.img,
        markdown: post.markdown,
        timestamp: Utc::now(),
    }))
}
