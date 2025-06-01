use std::{sync::Arc, time::Duration};

use axum::{
    body::Bytes,
    extract::{
        ws::{Message, WebSocket},
        Path, Query, State, WebSocketUpgrade,
    },
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect, Response},
    Extension, Json,
};
use futures::{lock::Mutex, sink::SinkExt, stream::StreamExt};
use hmac::{Hmac, Mac};
use schemars::JsonSchema;
use serde::Deserialize;
use sha2::Sha256;
use tokio::time;

use crate::{
    database, extend_slug,
    handler::{internal_error, sql_not_found, MiddlewareData},
    html_template,
    render::markdown_to_html,
    schema::*,
    templates::*,
    AppState,
};

use super::ParentParam;

pub fn sign(id: i32, hmac_key: &[u8]) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(hmac_key).unwrap();
    mac.update(id.to_string().as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

pub struct VerifiedId(pub i32);

pub async fn verify_signature(
    slug_or_id: &str,
    signature: &str,
    state: &AppState,
) -> Result<VerifiedId, StatusCode> {
    let signature = hex::decode(signature).map_err(|_| StatusCode::BAD_REQUEST)?;
    let mut mac = Hmac::<Sha256>::new_from_slice(&state.hmac_key).unwrap();

    let id = match slug_or_id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            sqlx::query!(
                r#"SELECT id as "id!" FROM posts WHERE slug = $1 UNION 
                    SELECT post_id FROM post_redirects WHERE slug = $1"#,
                slug_or_id
            )
            .fetch_one(&state.db)
            .await
            .map_err(sql_not_found)?
            .id
        }
    };

    mac.update(id.to_string().as_bytes());

    if mac.verify_slice(&signature).is_ok() {
        return Ok(VerifiedId(id));
    }

    Err(StatusCode::UNAUTHORIZED)
}

// Routes

pub async fn get_post(
    Extension(metadata): Extension<MiddlewareData>,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    match database::get_post(&state, &slug)
        .await
        .map_err(internal_error)?
    {
        Some(post) => Ok(html_template(PostTemplate { metadata, post }).into_response()),
        None => {
            // Check if it's a redirect
            let redirect = database::get_post_redirect(&state, &slug)
                .await
                .map_err(internal_error)?;

            match redirect {
                Some(slug) => Ok(Redirect::permanent(&format!("/blog/p/{}", slug)).into_response()),
                None => Err(StatusCode::NOT_FOUND),
            }
        }
    }
}

#[derive(Deserialize, JsonSchema)]
pub struct HiddenRequest {
    signature: String,
}

pub async fn get_post_hidden(
    Extension(metadata): Extension<MiddlewareData>,
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(HiddenRequest { signature }): Query<HiddenRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let verified_id = verify_signature(&slug, &signature, &state).await?;

    let post = database::get_post_hidden(&state, verified_id)
        .await
        .map_err(internal_error)?
        .ok_or(StatusCode::NOT_FOUND)?;

    html_template(PostTemplate { metadata, post })
}

pub async fn get_posts_hidden(
    Extension(metadata): Extension<MiddlewareData>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let posts = database::get_hidden_posts(&state)
        .await
        .map_err(internal_error)?;

    html_template(HiddenPostsTemplate { metadata, posts })
}

pub async fn get_editor() -> impl IntoResponse {
    // Custom security headers because we skip middleware.
    // Lax CSP required due to https://github.com/microsoft/monaco-editor/issues/271
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Security-Policy",
        "default-src 'self'; worker-src blob:; style-src 'self' 'unsafe-inline'"
            .parse()
            .unwrap(),
    );
    headers.insert("X-Frame-Options", "SAMEORIGIN".parse().unwrap());
    headers.insert("Referrer-Policy", "origin".parse().unwrap());
    headers.insert("Cross-Origin-Opener-Policy", "same-origin".parse().unwrap());
    headers.insert(
        "Cross-Origin-Resource-Policy",
        "same-origin".parse().unwrap(),
    );
    (headers, html_template(EditorTemplate {}))
}

pub async fn get_new_post(
    Extension(metadata): Extension<MiddlewareData>,
    State(state): State<AppState>,
    Query(ParentParam { parent }): Query<ParentParam>,
) -> Result<impl IntoResponse, StatusCode> {
    let folders = database::get_folders(&state)
        .await
        .map_err(internal_error)?;
    let all_tags = database::get_tags(&state).await.map_err(internal_error)?;
    html_template(NewPostTemplate {
        metadata,
        parent,
        existing_post: None,
        folders,
        all_tags,
    })
}

pub async fn post_new_post(
    State(state): State<AppState>,
    Json(post): Json<CreatePost>,
) -> Result<Json<ResultUrl>, StatusCode> {
    dbg!(&post);
    let slug = extend_slug(&post.slug, post.folder, &state)
        .await
        .map_err(internal_error)?;
    dbg!(&slug);

    // TODO: create script to re-render all markdown to html column
    let html = markdown_to_html(&post.markdown).map_err(internal_error)?;

    let id = sqlx::query!(
        "INSERT INTO posts (folder, title, slug, description, img, points, featured, hidden, autorelease, markdown, html) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id",
        post.folder,
        post.title,
        slug,
        post.description,
        post.img,
        post.points,
        post.featured,
        post.hidden,
        post.autorelease,
        post.markdown,
        html
    )
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)?
    .id;

    sqlx::query!(
        "INSERT INTO post_tags (post_id, tag_id) SELECT $1, id FROM tags WHERE id = ANY($2)",
        id,
        &post.tags
    )
    .execute(&state.db)
    .await
    .map_err(internal_error)?;

    Ok(Json(ResultUrl::post(slug)))
}

pub async fn get_new_link(
    Extension(metadata): Extension<MiddlewareData>,
    State(state): State<AppState>,
    Query(ParentParam { parent }): Query<ParentParam>,
) -> Result<impl IntoResponse, StatusCode> {
    let folders = database::get_folders(&state)
        .await
        .map_err(internal_error)?;
    let all_tags = database::get_tags(&state).await.map_err(internal_error)?;
    html_template(NewLinkTemplate {
        metadata,
        parent,
        existing_link: None,
        folders,
        all_tags,
    })
}

pub async fn post_new_link(
    State(state): State<AppState>,
    Json(link): Json<CreateLink>,
) -> Result<Json<ResultUrl>, StatusCode> {
    sqlx::query!(
        "INSERT INTO links (folder, url, title, description, img, featured) VALUES ($1, $2, $3, $4, $5, $6)",
        link.folder,
        link.url,
        link.title,
        link.description,
        link.img,
        link.featured
    )
    .execute(&state.db)
    .await
    .map_err(internal_error)?;

    // Get the folder slug
    let folder_slug = sqlx::query!("SELECT slug FROM folders WHERE id = $1", link.folder)
        .fetch_one(&state.db)
        .await
        .map_err(internal_error)?
        .slug;

    Ok(Json(ResultUrl::folder(folder_slug)))
}

pub async fn get_edit_post(
    Extension(metadata): Extension<MiddlewareData>,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let existing_post = database::get_post_by_id(&state, id)
        .await
        .map_err(internal_error)?
        .ok_or(StatusCode::NOT_FOUND)?;
    let folders = database::get_folders(&state)
        .await
        .map_err(internal_error)?;
    let tags = database::get_tags(&state).await.map_err(internal_error)?;
    dbg!(&existing_post);
    html_template(NewPostTemplate {
        metadata,
        parent: None,
        existing_post: Some(existing_post),
        folders,
        all_tags: tags,
    })
}

pub async fn put_edit_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(post): Json<CreatePost>,
) -> Result<Json<ResultUrl>, StatusCode> {
    let slug = extend_slug(&post.slug, post.folder, &state)
        .await
        .map_err(internal_error)?;

    let original_post = sqlx::query!("SELECT id, slug FROM posts WHERE id = $1", id)
        .fetch_one(&state.db)
        .await
        .map_err(internal_error)?;

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
    }

    let html = markdown_to_html(&post.markdown).map_err(internal_error)?;

    sqlx::query!(
        "UPDATE posts SET folder = $1, title = $2, slug = $3, description = $4, img = $5, points = $6, featured = $7, hidden = $8, autorelease = $9, markdown = $10, html = $11 WHERE id = $12",
        post.folder,
        post.title,
        slug,
        post.description,
        post.img,
        post.points,
        post.featured,
        post.hidden,
        post.autorelease,
        post.markdown,
        html,
        original_post.id
    )
    .execute(&state.db)
    .await
    .map_err(internal_error)?;

    sqlx::query!("DELETE FROM post_tags WHERE post_id = $1", original_post.id)
        .execute(&state.db)
        .await
        .map_err(internal_error)?;
    sqlx::query!(
        "INSERT INTO post_tags (post_id, tag_id) SELECT $1, id FROM tags WHERE id = ANY($2)",
        original_post.id,
        &post.tags
    )
    .execute(&state.db)
    .await
    .map_err(internal_error)?;

    Ok(Json(ResultUrl::post(slug)))
}

pub async fn get_edit_link(
    Extension(metadata): Extension<MiddlewareData>,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let existing_link = database::get_link(&state, id)
        .await
        .map_err(internal_error)?
        .ok_or(StatusCode::NOT_FOUND)?;
    let folders = database::get_folders(&state)
        .await
        .map_err(internal_error)?;
    let tags = database::get_tags(&state).await.map_err(internal_error)?;
    html_template(NewLinkTemplate {
        metadata,
        parent: None,
        existing_link: Some(existing_link),
        folders,
        all_tags: tags,
    })
}

pub async fn put_edit_link(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(link): Json<CreateLink>,
) -> Result<Json<ResultUrl>, StatusCode> {
    sqlx::query!(
        "UPDATE links SET folder = $1, title = $2, url = $3, description = $4, img = $5, featured = $6 WHERE id = $7",
        link.folder,
        link.title,
        link.url,
        link.description,
        link.img,
        link.featured,
        id
    )
    .execute(&state.db)
    .await
    .map_err(internal_error)?;

    // Get the folder slug
    let folder_slug = sqlx::query!("SELECT slug FROM folders WHERE id = $1", link.folder)
        .fetch_one(&state.db)
        .await
        .map_err(internal_error)?
        .slug;

    Ok(Json(ResultUrl::folder(folder_slug)))
}

pub async fn post_add_view(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(), StatusCode> {
    database::add_view(&state, id)
        .await
        .map_err(internal_error)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(())
}

/// Called every minute. This currently only auto-releases posts
pub async fn cron(State(state): State<AppState>) -> Result<(), sqlx::Error> {
    // Auto-Release posts
    let released_posts = sqlx::query!(
        "UPDATE posts SET hidden = false, timestamp = autorelease, autorelease = NULL WHERE autorelease <= NOW() AND hidden RETURNING slug"
    )
    .fetch_all(&state.db)
    .await?;
    if !released_posts.is_empty() {
        for post in &released_posts {
            println!("Cron: Releasing post {:?}", post.slug);
        }
    } else {
        println!("Cron: No posts to release");
    }
    Ok(())
}

pub async fn get_search_ws(ws: WebSocketUpgrade, state: State<AppState>) -> Response {
    println!("WebSocket: Incoming connection");
    ws.on_upgrade(|socket| async move {
        println!("WebSocket: handling...");
        handle_ws_search(socket, state).await
    })
}

pub async fn get_search(Extension(metadata): Extension<MiddlewareData>) -> impl IntoResponse {
    html_template(SearchTemplate { metadata })
}

/// Fuzzy search for posts by query, returning the top 5 results. '{~highlight~}' is used to highlight matches.
pub async fn handle_ws_search(socket: WebSocket, State(state): State<AppState>) {
    let (tx, mut rx) = socket.split();
    let tx = Arc::new(Mutex::new(tx));

    // Send ping every 10 seconds in background thread
    let tx_ping = tx.clone();
    tokio::spawn(async move {
        loop {
            time::sleep(Duration::from_secs(10)).await;
            println!("WebSocket: Sending ping");

            let mut tx = tx_ping.lock().await;
            if tx.send(Message::Ping(Bytes::new())).await.is_err() {
                break; // Connection closed
            };
        }
    });

    // Respond to incoming messages
    let tx_search = tx.clone();
    while let Some(Ok(msg)) = rx.next().await {
        println!("WebSocket: Received {msg:?}");
        if let Message::Text(query) = msg {
            println!("           -> Query: {}", query);
            match sqlx::query_as!(
                PostFull,
                r#"SELECT p.id, folder, slug, 
        ts_headline('english', title, query, 'StartSel={~, StopSel=~}') as "title!", 
        ts_headline('english', description, query, 'StartSel={~, StopSel=~}') as "description!", 
        ts_headline('english', plain_text, query, 
        'MaxFragments=2, MaxWords=10, MinWords=5, StartSel={~, StopSel=~}') as "markdown!", 
        '' as "html!", img, points, views, featured, hidden, autorelease, timestamp, 
        array(SELECT (t.id, t.name, t.color) FROM post_tags 
        JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
    FROM posts p JOIN websearch_to_tsquery('english', $1) query 
        ON (numnode(query) = 0 OR query @@ ts)
    WHERE NOT hidden
    ORDER BY ts_rank_cd(ts, query) DESC LIMIT 5"#,
                query.to_string()
            )
            .fetch_all(&state.db)
            .await
            {
                Ok(results) => {
                    println!("           -> Sending {} results", results.len());
                    let response = Message::Text(serde_json::to_string(&results).unwrap().into());

                    let mut tx = tx_search.lock().await;
                    if tx.send(response).await.is_err() {
                        break; // Connection closed
                    };
                }
                Err(e) => {
                    eprintln!("WebSocket Error: {}", e);
                    break;
                }
            }
        }
    }
}
