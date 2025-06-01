use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Extension, Json,
};

use crate::{
    database, extend_slug,
    handler::{internal_error, MiddlewareData},
    html_template,
    schema::*,
    templates::*,
    AppState,
};

use super::ParentParam;

pub async fn get_folder(
    Extension(metadata): Extension<MiddlewareData>,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    match database::get_folder(&state, &slug)
        .await
        .map_err(internal_error)?
    {
        Some(folder) => {
            // Add contents to the folder
            let folder = FolderContents::from_folder(folder, &state)
                .await
                .map_err(internal_error)?;
            dbg!(&folder);
            // TODO: render admin interface if needed
            Ok(html_template(FolderTemplate { metadata, folder }).into_response())
        }
        None => {
            // Check if it's a redirect
            let redirect = database::get_folder_redirect(&state, &slug)
                .await
                .map_err(internal_error)?;

            match redirect {
                Some(slug) => Ok(Redirect::permanent(&format!("/blog/f/{}", slug)).into_response()),
                None => Err(StatusCode::NOT_FOUND),
            }
        }
    }
}

pub async fn get_new_folder(
    Extension(metadata): Extension<MiddlewareData>,
    State(state): State<AppState>,
    Query(ParentParam { parent }): Query<ParentParam>,
) -> Result<impl IntoResponse, StatusCode> {
    let folders = database::get_folders(&state)
        .await
        .map_err(internal_error)?;
    html_template(NewFolderTemplate {
        metadata,
        parent,
        existing_folder: None,
        folders,
    })
}

pub async fn post_new_folder(
    State(state): State<AppState>,
    Json(folder): Json<CreateFolder>,
) -> Result<Json<ResultUrl>, StatusCode> {
    let slug = match folder.parent {
        Some(parent) => extend_slug(&folder.slug, parent, &state)
            .await
            .map_err(internal_error)?,
        None => folder.slug.clone(),
    };

    sqlx::query!(
        "INSERT INTO folders (title, slug, description, img, parent) VALUES ($1, $2, $3, $4, $5)",
        folder.title,
        slug,
        folder.description,
        folder.img,
        folder.parent
    )
    .execute(&state.db)
    .await
    .map_err(internal_error)?;

    Ok(Json(ResultUrl::folder(slug)))
}

pub async fn get_edit_folder(
    Extension(metadata): Extension<MiddlewareData>,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let existing_folder = database::get_folder_by_id(&state, id)
        .await
        .map_err(internal_error)?
        .ok_or(StatusCode::NOT_FOUND)?;
    let folders = database::get_folders(&state)
        .await
        .map_err(internal_error)?;
    html_template(NewFolderTemplate {
        metadata,
        parent: None,
        existing_folder: Some(existing_folder),
        folders,
    })
}

pub async fn put_edit_folder(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(folder): Json<CreateFolder>,
) -> Result<Json<ResultUrl>, StatusCode> {
    let original_folder = sqlx::query_as!(
        Folder,
        "SELECT id, parent, slug, title, description, img, timestamp FROM folders WHERE id = $1",
        id
    )
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)?;

    let slug = extend_slug(&folder.slug, folder.parent.unwrap_or(-1), &state)
        .await
        .map_err(internal_error)?;

    if original_folder.slug != slug {
        let old_slug_full = original_folder.slug.clone() + "/";

        // Add backups to redirects table
        sqlx::query!(
            "INSERT INTO post_redirects (slug, post_id) SELECT slug, id FROM posts WHERE POSITION($1 IN slug) = 1 ON CONFLICT DO NOTHING",
            old_slug_full
        ).execute(&state.db).await.map_err(internal_error)?;

        sqlx::query!(
            "INSERT INTO folder_redirects (slug, folder_id) SELECT slug, id FROM folders WHERE POSITION($1 IN slug) = 1 OR slug = $2 ON CONFLICT DO NOTHING",
            old_slug_full,
            original_folder.slug
        ).execute(&state.db).await.map_err(internal_error)?;

        // Replace slug in posts and folders
        sqlx::query!(
            "UPDATE posts SET slug=$1 || substring(slug, POSITION($2 IN slug)+length($2)) WHERE POSITION($3 IN slug) = 1",
            slug,
            original_folder.slug,
            old_slug_full
        ).execute(&state.db).await.map_err(internal_error)?;

        sqlx::query!(
            "UPDATE folders SET slug=$1 || substring(slug, POSITION($2 IN slug)+length($2)) WHERE POSITION($3 IN slug) = 1",
            slug,
            original_folder.slug,
            old_slug_full
        ).execute(&state.db).await.map_err(internal_error)?;
    }

    // Update the post
    sqlx::query!(
        "UPDATE folders SET title = $1, slug = $2, description = $3, img = $4, parent = $5 WHERE id = $6",
        folder.title,
        slug,
        folder.description,
        folder.img,
        folder.parent,
        original_folder.id
    )
    .execute(&state.db)
    .await
    .map_err(internal_error)?;

    Ok(Json(ResultUrl::folder(slug)))
}
