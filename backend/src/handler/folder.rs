use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    build_slug, internal_error, revalidate, schema::*, slugify, sql_not_found, AppState, Slug,
};

pub async fn get_folders(State(state): State<AppState>) -> Result<Json<Vec<Folder>>, StatusCode> {
    sqlx::query_as!(
        Folder,
        "SELECT id, parent, slug, title, description, img, timestamp FROM folders"
    )
    .fetch_all(&state.db)
    .await
    .map_err(internal_error)
    .map(Json)
}

pub async fn get_folder(
    State(state): State<AppState>,
    Path(slug_or_id): Path<String>,
) -> Result<Json<FolderContents>, StatusCode> {
    let folder = match slug_or_id.parse::<i32>() {
        Ok(id) => sqlx::query_as!(
                Folder,
                "SELECT id, parent, slug, title, description, img, timestamp FROM folders WHERE id = ?",
                id
            )
            .fetch_one(&state.db)
            .await,
        Err(_) => sqlx::query_as!(
                Folder,
                "SELECT id, parent, slug, title, description, img, timestamp FROM folders WHERE slug = ?",
                slug_or_id
            ).fetch_one(&state.db).await
    }.map_err(sql_not_found)?;

    FolderContents::from_folder(folder, &state)
        .await
        .map(Json)
        .map_err(internal_error)
}

pub async fn create_folder(
    State(state): State<AppState>,
    Json(folder): Json<CreateFolder>,
) -> Result<Json<Folder>, StatusCode> {
    let slug = match folder.parent {
        Some(parent) => build_slug(parent, &folder.title, &state)
            .await
            .map_err(internal_error)?,
        None => slugify(&folder.title),
    };

    sqlx::query!(
        "INSERT INTO folders (title, slug, description, img, parent) VALUES (?, ?, ?, ?, ?)",
        folder.title,
        slug,
        folder.description,
        folder.img,
        folder.parent
    )
    .execute(&state.db)
    .await
    .map_err(internal_error)?;

    revalidate(Slug::Folder { slug: slug.clone() })
        .await
        .map_err(internal_error)?;

    sqlx::query_as!(
        Folder,
        "SELECT id, parent, slug, title, description, img, timestamp FROM folders WHERE slug = ?",
        slug
    )
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)
    .map(Json)
}

pub async fn edit_folder(
    State(state): State<AppState>,
    Path(slug_or_id): Path<String>,
    Json(folder): Json<CreateFolder>,
) -> Result<Json<Folder>, StatusCode> {
    // NOTE: Slug doesn't get updated, because it's used in many post URLs
    sqlx::query!(
        "UPDATE folders SET title = ?, description = ?, img = ?, parent = ? WHERE id = ? OR slug = ?",
        folder.title,
        folder.description,
        folder.img,
        folder.parent,
        slug_or_id,
        slug_or_id
    )
    .execute(&state.db)
    .await
    .map_err(internal_error)?;

    let folder = sqlx::query_as!(
        Folder,
        "SELECT id, parent, slug, title, description, img, timestamp FROM folders WHERE id = ? OR slug = ?",
        slug_or_id,
        slug_or_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)?;

    revalidate(Slug::Folder {
        slug: folder.slug.clone(),
    })
    .await
    .map_err(internal_error)?;

    Ok(Json(folder))
}
