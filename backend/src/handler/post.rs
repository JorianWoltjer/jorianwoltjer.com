use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{build_slug, internal_error, revalidate, schema::*, sql_not_found, AppState, Slug};

pub async fn get_posts(
    State(state): State<AppState>,
) -> Result<Json<Vec<PostSummary>>, StatusCode> {
    sqlx::query_as!(
        PostSummary,
        "SELECT id, folder, slug, title, description, img, timestamp FROM posts"
    )
    .fetch_all(&state.db)
    .await
    .map_err(internal_error)
    .map(Json)
}

pub async fn get_post(
    State(state): State<AppState>,
    Path(slug_or_id): Path<String>,
) -> Result<Json<Post>, StatusCode> {
    sqlx::query_as!(
        Post,
        "SELECT id, folder, slug, title, description, img, markdown, timestamp FROM posts WHERE id = ? OR slug = ?",
        slug_or_id,
        slug_or_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(sql_not_found)
    .map(Json)
}

pub async fn create_post(
    State(state): State<AppState>,
    Json(post): Json<CreatePost>,
) -> Result<Json<PostSummary>, StatusCode> {
    let slug = build_slug(post.folder, &post.title, &state)
        .await
        .map_err(internal_error)?;

    sqlx::query!(
        "INSERT INTO posts (folder, title, slug, description, img, markdown) VALUES (?, ?, ?, ?, ?, ?)",
        post.folder,
        post.title,
        slug,
        post.description,
        post.img,
        post.markdown
    )
    .execute(&state.db)
    .await
    .map_err(internal_error)?;

    revalidate(Slug::Post { slug: slug.clone() })
        .await
        .map_err(internal_error)?;

    sqlx::query_as!(
        PostSummary,
        "SELECT id, folder, slug, title, description, img, timestamp FROM posts WHERE slug = ?",
        slug
    )
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)
    .map(Json)
}

pub async fn edit_post(
    State(state): State<AppState>,
    Path(slug_or_id): Path<String>,
    Json(post): Json<CreatePost>,
) -> Result<Json<PostSummary>, StatusCode> {
    let slug = build_slug(post.folder, &post.title, &state)
        .await
        .map_err(internal_error)?;

    sqlx::query!(
        "UPDATE posts SET folder = ?, title = ?, slug = ?, description = ?, img = ?, markdown = ? WHERE id = ? OR slug = ?",
        post.folder,
        post.title,
        slug,
        post.description,
        post.img,
        post.markdown,
        slug_or_id,
        slug_or_id
    )
    .execute(&state.db)
    .await
    .map_err(internal_error)?;

    revalidate(Slug::Post { slug: slug.clone() })
        .await
        .map_err(internal_error)?;

    sqlx::query_as!(
        PostSummary,
        "SELECT id, folder, slug, title, description, img, timestamp FROM posts WHERE slug = ?",
        slug
    )
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)
    .map(Json)
}
