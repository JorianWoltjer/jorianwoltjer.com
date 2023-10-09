use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    build_slug,
    handler::{internal_error, sql_not_found},
    schema::*,
    slugify, AppState, RevalidationRequest, Slug,
};

pub async fn get_folders(State(state): State<AppState>) -> Result<Json<Vec<Folder>>, StatusCode> {
    sqlx::query_as!(
        Folder,
        "SELECT id, parent, slug, title, description, img, timestamp FROM folders ORDER BY timestamp DESC"
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
    let folder = sqlx::query_as!(
                Folder,
                "SELECT id, parent, slug, title, description, img, timestamp FROM folders WHERE id::varchar = $1 OR slug = $1",
                slug_or_id
            )
            .fetch_one(&state.db)
            .await;

    if let Ok(folder) = folder {
        Ok(FolderContents::from_folder(folder, &state)
            .await
            .map(Json)
            .map_err(internal_error)?)
    } else {
        Ok(Json(
            FolderContents::from_folder(
                sqlx::query_as!(
                    Folder,
                    "SELECT f.id, parent, f.slug, title, description, img, timestamp 
                    FROM folders f
                    JOIN folder_redirects fr ON f.id = fr.folder_id 
                    WHERE fr.slug = $1",
                    slug_or_id
                )
                .fetch_one(&state.db)
                .await
                .map_err(sql_not_found)?,
                &state,
            )
            .await
            .map_err(internal_error)?,
        ))
    }
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

    RevalidationRequest {
        slugs: vec![Slug::Folder { slug: slug.clone() }],
    }
    .execute()
    .await
    .map_err(internal_error)?;

    sqlx::query_as!(
        Folder,
        "SELECT id, parent, slug, title, description, img, timestamp FROM folders WHERE slug = $1",
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
    let original_folder = sqlx::query_as!(
        Folder,
        "SELECT id, parent, slug, title, description, img, timestamp FROM folders WHERE id::varchar = $1 OR slug = $1",
        slug_or_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)?;

    let slug = match folder.parent {
        Some(parent) => build_slug(parent, &folder.title, &state)
            .await
            .map_err(internal_error)?,
        None => slugify(&folder.title),
    };

    let mut revalidations = vec![
        Slug::Folder {
            slug: original_folder.slug.clone(),
        },
        Slug::Folder { slug: slug.clone() },
    ];

    if original_folder.slug != slug {
        let old_slug_full = original_folder.slug.clone() + "/";
        let new_slug_full = slug.clone() + "/";

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

        // Create revalidations for NextJS of what was updated
        let post_revalidations = sqlx::query!(r#"SELECT slug as "slug!" FROM post_redirects WHERE post_id IN (SELECT id FROM posts WHERE POSITION($1 IN slug) = 1) UNION SELECT slug FROM posts WHERE POSITION($1 IN slug) = 1"#, 
            new_slug_full
        ).fetch_all(&state.db).await.map_err(internal_error)?;
        revalidations.extend(
            post_revalidations
                .into_iter()
                .map(|record| Slug::Post { slug: record.slug }),
        );

        let folder_revalidations = sqlx::query!(r#"SELECT slug as "slug!" FROM folder_redirects WHERE folder_id IN (SELECT id FROM folders WHERE POSITION($1 IN slug) = 1) UNION SELECT slug FROM folders WHERE POSITION($1 IN slug) = 1"#, 
            new_slug_full
        ).fetch_all(&state.db).await.map_err(internal_error)?;
        revalidations.extend(
            folder_revalidations
                .into_iter()
                .map(|record| Slug::Folder { slug: record.slug }),
        );
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

    RevalidationRequest {
        slugs: revalidations,
    }
    .execute()
    .await
    .map_err(internal_error)?;

    sqlx::query_as!(
        Folder,
        "SELECT id, parent, slug, title, description, img, timestamp FROM folders WHERE id = $1",
        original_folder.id
    )
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)
    .map(Json)
}
