use axum::{extract::State, http::StatusCode, Json};

use crate::{internal_error, revalidate, schema::*, AppState};

pub async fn health_check() -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn get_posts(State(state): State<AppState>) -> Json<Vec<Post>> {
    sqlx::query_as!(Post, "SELECT * FROM posts")
        .fetch_all(&state.db)
        .await
        .unwrap()
        .into()
}

pub async fn create_post(
    State(state): State<AppState>,
    Json(post): Json<CreatePost>,
) -> Result<(), StatusCode> {
    sqlx::query!(
        "INSERT INTO posts (title, body) VALUES (?, ?)",
        post.title,
        post.body
    )
    .execute(&state.db)
    .await
    .map_err(internal_error)?;

    revalidate("/").await.map_err(internal_error)?;
    Ok(())
}
