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

async fn get_post_summary(id: i32, state: &AppState) -> Result<PostSummary, sqlx::Error> {
    sqlx::query_as!(
        PostSummary,
        r#"SELECT p.id, folder, slug, title, description, img, points, views, featured, timestamp, 
            array_agg((t.id, t.name, t.color)) as "tags!: Vec<Tag>" FROM posts p 
            JOIN post_tags pt on pt.post_id = p.id JOIN tags t ON t.id = pt.tag_id WHERE p.id = $1
            GROUP BY p.id"#,
        id
    )
    .fetch_one(&state.db)
    .await
}

pub async fn get_posts(
    State(state): State<AppState>,
) -> Result<Json<Vec<PostSummary>>, StatusCode> {
    sqlx::query_as!(
        PostSummary,
        r#"SELECT p.id, folder, slug, title, description, img, points, views, featured, timestamp, 
            array_agg((t.id, t.name, t.color)) as "tags!: Vec<Tag>" FROM posts p
            JOIN post_tags pt on pt.post_id = p.id JOIN tags t ON t.id = pt.tag_id
            GROUP BY p.id"#
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
        r#"SELECT p.id, folder, slug, title, description, img, markdown, points, views, featured, timestamp, 
            array_agg((t.id, t.name, t.color)) as "tags!: Vec<Tag>" FROM posts p 
            JOIN post_tags pt on pt.post_id = p.id JOIN tags t ON t.id = pt.tag_id WHERE p.id::varchar = $1 OR slug = $1
            GROUP BY p.id"#,
        slug_or_id
    )
    .fetch_one(&state.db)
    .await {
        Ok(PostResponse::Post(Json(post)))
    } else {
        sqlx::query!(
            "SELECT p.slug FROM posts p JOIN post_redirects pr ON p.id = pr.post_id WHERE pr.slug = $1",
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

    let id = sqlx::query!(
        "INSERT INTO posts (folder, title, slug, description, img, points, featured, markdown) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id",
        post.folder,
        post.title,
        slug,
        post.description,
        post.img,
        post.points,
        post.featured,
        post.markdown
    )
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)?
    .id;

    let tag_ids = post.tags.iter().map(|tag| tag.id).collect::<Vec<_>>();
    sqlx::query!(
        "INSERT INTO post_tags (post_id, tag_id) SELECT $1, id FROM tags WHERE id = ANY($2)",
        id,
        &tag_ids
    )
    .execute(&state.db)
    .await
    .map_err(internal_error)?;

    // Everything has changed now, revalidate NextJS
    RevalidationRequest {
        slugs: vec![Slug::Post { slug: slug.clone() }],
    }
    .execute()
    .await
    .map_err(internal_error)?;

    get_post_summary(id, &state)
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

    let original_post = sqlx::query!(
        "SELECT id, slug FROM posts WHERE id::varchar = $1 OR slug = $1",
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
            "INSERT INTO post_redirects (slug, post_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            original_post.slug,
            original_post.id
        )
        .execute(&state.db)
        .await
        .map_err(internal_error)?;

        let post_revalidations = sqlx::query!(
            "SELECT slug FROM post_redirects WHERE post_id = $1",
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
        "UPDATE posts SET folder = $1, title = $2, slug = $3, description = $4, img = $5, points = $6, featured = $7, markdown = $8 WHERE id = $9",
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

    sqlx::query!("DELETE FROM post_tags WHERE post_id = $1", original_post.id)
        .execute(&state.db)
        .await
        .map_err(internal_error)?;
    let tag_ids = post.tags.iter().map(|tag| tag.id).collect::<Vec<_>>();
    sqlx::query!(
        "INSERT INTO post_tags (post_id, tag_id) SELECT $1, id FROM tags WHERE id = ANY($2)",
        original_post.id,
        &tag_ids
    )
    .execute(&state.db)
    .await
    .map_err(internal_error)?;

    // Everything has changed now, revalidate NextJS
    RevalidationRequest {
        slugs: revalidations,
    }
    .execute()
    .await
    .map_err(internal_error)?;

    get_post_summary(original_post.id, &state)
        .await
        .map_err(internal_error)
        .map(Json)
}

pub async fn get_featured_posts(
    State(state): State<AppState>,
) -> Result<Json<Vec<PostSummary>>, StatusCode> {
    sqlx::query_as!(
        PostSummary,
        r#"SELECT p.id, folder, slug, title, description, img, points, views, featured, timestamp, 
            array_agg((t.id, t.name, t.color)) as "tags!: Vec<Tag>" FROM posts p
            JOIN post_tags pt on pt.post_id = p.id JOIN tags t ON t.id = pt.tag_id
            WHERE featured = true
            GROUP BY p.id"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(internal_error)
    .map(Json)
}

pub async fn get_tags(State(state): State<AppState>) -> Result<Json<Vec<Tag>>, StatusCode> {
    sqlx::query_as!(Tag, "SELECT * FROM tags")
        .fetch_all(&state.db)
        .await
        .map_err(internal_error)
        .map(Json)
}
