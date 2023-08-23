use axum::{
    extract::{Path, State},
    http::{Request, StatusCode},
    response::Response,
    Json, RequestPartsExt,
};
use axum_sessions::extractors::{ReadableSession, WritableSession};

use crate::{internal_error, revalidate, schema::*, slugify, AppState, Slug};

pub async fn health_check() -> StatusCode {
    StatusCode::NO_CONTENT
}

// Authentication

pub async fn auth_required_middleware<B>(
    req: Request<B>,
    next: axum::middleware::Next<B>,
) -> Result<Response, StatusCode> {
    let (mut parts, body) = req.into_parts();

    let session = parts
        .extract::<ReadableSession>()
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !session.get::<bool>("logged_in").unwrap_or(false) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(Request::from_parts(parts, body)).await)
}

pub async fn login_check() -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn login(
    mut session: WritableSession,
    State(state): State<AppState>,
    Json(login): Json<Login>,
) -> Result<StatusCode, StatusCode> {
    let password_hash = sqlx::query!("SELECT password_hash FROM users")
        .fetch_one(&state.db)
        .await
        .map_err(internal_error)?
        .password_hash;

    match bcrypt::verify(login.password, &password_hash) {
        Ok(true) => {
            session.insert("logged_in", true).map_err(internal_error)?;
            Ok(StatusCode::NO_CONTENT)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn logout(mut session: WritableSession) -> StatusCode {
    session.destroy();
    StatusCode::NO_CONTENT
}

// Blog

pub async fn get_posts(
    State(state): State<AppState>,
) -> Result<Json<Vec<PostSummary>>, StatusCode> {
    sqlx::query_as!(
        PostSummary,
        "SELECT id, slug, title, description, img, timestamp FROM posts"
    )
    .fetch_all(&state.db)
    .await
    .map_err(internal_error)
    .map(Json)
}

pub async fn get_post_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Post>, StatusCode> {
    let post = sqlx::query_as!(
        Post,
        "SELECT id, slug, title, description, img, markdown, timestamp FROM posts WHERE slug = ?",
        slug
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
        _ => internal_error(e),
    })?;

    Ok(post.into())
}

pub async fn create_post(
    State(state): State<AppState>,
    Json(post): Json<CreatePost>,
) -> Result<Json<PostSummary>, StatusCode> {
    let parent_slug = sqlx::query!("SELECT slug FROM folders WHERE id = ?", post.folder)
        .fetch_one(&state.db)
        .await
        .map_err(internal_error)?
        .slug;

    let slug = format!("{parent_slug}/{}", slugify(&post.title));

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

    Ok(sqlx::query_as!(
        PostSummary,
        "SELECT id, slug, title, description, img, timestamp FROM posts WHERE slug = ?",
        slug
    )
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)?
    .into())
}

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

pub async fn get_folder_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<FolderContents>, StatusCode> {
    let folder = sqlx::query_as!(
        Folder,
        "SELECT id, parent, slug, title, description, img, timestamp FROM folders WHERE slug = ?",
        slug
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
        _ => internal_error(e),
    })?;

    let contents_folders = sqlx::query_as!(
        Folder,
        "SELECT id, parent, slug, title, description, img, timestamp FROM folders WHERE parent = ?",
        folder.id
    )
    .fetch_all(&state.db)
    .await
    .map_err(internal_error)?;

    let contents_posts = sqlx::query_as!(
        PostSummary,
        "SELECT id, slug, title, description, img, timestamp FROM posts WHERE folder = ?",
        folder.id
    )
    .fetch_all(&state.db)
    .await
    .map_err(internal_error)?;

    Ok(Json(FolderContents {
        id: folder.id,
        slug: folder.slug,
        title: folder.title,
        description: folder.description,
        img: folder.img,
        timestamp: folder.timestamp,
        folders: contents_folders,
        posts: contents_posts,
    }))
}

pub async fn create_folder(
    State(state): State<AppState>,
    Json(folder): Json<CreateFolder>,
) -> Result<Json<Folder>, StatusCode> {
    let slug = match folder.parent {
        Some(parent) => {
            let parent_slug = sqlx::query!("SELECT slug FROM folders WHERE id = ?", parent)
                .fetch_one(&state.db)
                .await
                .map_err(internal_error)?
                .slug;
            format!("{parent_slug}/{}", slugify(&folder.title))
        }
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

    Ok(sqlx::query_as!(
        Folder,
        "SELECT id, parent, slug, title, description, img, timestamp FROM folders WHERE slug = ?",
        slug
    )
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)?
    .into())
}
