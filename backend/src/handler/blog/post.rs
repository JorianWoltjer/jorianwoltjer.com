use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    Json,
};

use crate::{
    build_slug,
    handler::{internal_error, sql_not_found},
    schema::*,
    AppState, RevalidationRequest, Slug,
};

pub enum PostResponse {
    Post(Json<Post>),
    Redirect(Redirect),
}
impl IntoResponse for PostResponse {
    fn into_response(self) -> Response {
        match self {
            PostResponse::Post(post) => post.into_response(),
            PostResponse::Redirect(redirect) => redirect.into_response(),
        }
    }
}

pub async fn get_posts(
    State(state): State<AppState>,
) -> Result<Json<Vec<PostSummary>>, StatusCode> {
    sqlx::query_as!(
        PostSummary,
        "SELECT id, folder, slug, title, description, img, points, views, featured as `featured: bool`, timestamp FROM posts"
    )
    .fetch_all(&state.db)
    .await
    .map_err(internal_error)
    .map(Json)
}

pub async fn get_post(
    State(state): State<AppState>,
    Path(slug_or_id): Path<String>,
) -> Result<PostResponse, StatusCode> {
    if let Ok(post) = sqlx::query_as!(
        Post,
        "SELECT id, folder, slug, title, description, img, markdown, points, views, featured as `featured: bool`, timestamp FROM posts WHERE id = ? OR slug = ?",
        slug_or_id,
        slug_or_id
    )
    .fetch_one(&state.db)
    .await {
        Ok(PostResponse::Post(Json(post)))
    } else {
        sqlx::query!(
            "SELECT p.slug FROM posts p JOIN post_redirects pr ON p.id = pr.post_id WHERE pr.slug = ?",
            slug_or_id
        )
        .fetch_one(&state.db)
        .await
        .map_err(sql_not_found)
        .map(|record| PostResponse::Redirect(Redirect::permanent(&format!("/blog/post/{}", record.slug))))
    }
}

pub async fn create_post(
    State(state): State<AppState>,
    Json(post): Json<CreatePost>,
) -> Result<Json<PostSummary>, StatusCode> {
    let slug = build_slug(post.folder, &post.title, &state)
        .await
        .map_err(internal_error)?;

    sqlx::query!(
        "INSERT INTO posts (folder, title, slug, description, img, points, featured, markdown) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        post.folder,
        post.title,
        slug,
        post.description,
        post.img,
        post.points,
        post.featured,
        post.markdown
    )
    .execute(&state.db)
    .await
    .map_err(internal_error)?;

    RevalidationRequest {
        slugs: vec![Slug::Post { slug: slug.clone() }],
    }
    .execute()
    .await
    .map_err(internal_error)?;

    sqlx::query_as!(
        PostSummary,
        "SELECT id, folder, slug, title, description, img, points, views, featured as `featured: bool`, timestamp FROM posts WHERE slug = ?",
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

    let original_post = sqlx::query_as!(
        Post,
        "SELECT id, folder, slug, title, description, img, markdown, points, views, featured as `featured: bool`, timestamp FROM posts WHERE id = ? OR slug = ?",
        slug_or_id,
        slug_or_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)?;

    let mut revalidations = vec![
        Slug::Post {
            slug: original_post.slug.clone(),
        },
        Slug::Post { slug: slug.clone() },
    ];

    if original_post.slug != slug {
        // Add old to redirects table
        sqlx::query!(
            "INSERT IGNORE INTO post_redirects (slug, post_id) VALUES (?, ?)",
            original_post.slug,
            original_post.id
        )
        .execute(&state.db)
        .await
        .map_err(internal_error)?;

        let post_revalidations = sqlx::query!(
            "SELECT slug FROM post_redirects WHERE post_id = ?",
            original_post.id
        )
        .fetch_all(&state.db)
        .await
        .map_err(internal_error)?;
        revalidations.extend(
            post_revalidations
                .into_iter()
                .map(|record| Slug::Post { slug: record.slug }),
        );
    }

    sqlx::query!(
        "UPDATE posts SET folder = ?, title = ?, slug = ?, description = ?, img = ?, points = ?, featured = ?, markdown = ? WHERE id = ?",
        post.folder,
        post.title,
        slug,
        post.description,
        post.img,
        post.points,
        post.featured,
        post.markdown,
        original_post.id
    )
    .execute(&state.db)
    .await
    .map_err(internal_error)?;

    RevalidationRequest {
        slugs: revalidations,
    }
    .execute()
    .await
    .map_err(internal_error)?;

    sqlx::query_as!(
        PostSummary,
        "SELECT id, folder, slug, title, description, img, points, views, featured as `featured: bool`, timestamp FROM posts WHERE id = ?",
        original_post.id
    )
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)
    .map(Json)
}
