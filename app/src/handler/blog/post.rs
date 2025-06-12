use std::{error, sync::Arc, time::Duration};

use axum::{
    body::Bytes,
    extract::{
        ws::{Message, WebSocket},
        Path, Query, State, WebSocketUpgrade,
    },
    http::{self, header, HeaderMap, StatusCode},
    response::{IntoResponse, Redirect, Response},
    Extension, Json,
};
use futures::{lock::Mutex, sink::SinkExt, stream::StreamExt};
use hmac::{Hmac, Mac};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
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
    Extension(middleware): Extension<MiddlewareData>,
    url: http::Uri,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match database::get_post(&state, &slug)
        .await
        .map_err(|e| internal_error(e).into_response())?
    {
        Some(post) => Ok(html_template(
            middleware.logged_in,
            PostTemplate {
                middleware,
                metadata: Metadata {
                    url,
                    title: post.title.clone(),
                    description: Some(post.description.clone()),
                    image: Some(format!("/img/blog/{}", post.img)),
                },
                post,
            },
        )
        .into_response()),
        None => {
            // Check if it's a redirect
            let redirect = database::get_post_redirect(&state, &slug)
                .await
                .map_err(|e| internal_error(e).into_response())?;

            match redirect {
                Some(slug) => Ok(Redirect::permanent(&format!("/blog/p/{}", slug)).into_response()),
                // If not found, return 404
                None => Err(Redirect::temporary("/blog").into_response()),
            }
        }
    }
}

#[derive(Deserialize, JsonSchema)]
pub struct HiddenRequest {
    #[serde(rename = "s")]
    signature: String,
}

pub async fn get_post_hidden(
    Extension(middleware): Extension<MiddlewareData>,
    url: http::Uri,
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(HiddenRequest { signature }): Query<HiddenRequest>,
) -> Result<impl IntoResponse, Response> {
    let verified_id = verify_signature(&slug, &signature, &state)
        .await
        .map_err(IntoResponse::into_response)?;

    let post = database::get_post_hidden(&state, verified_id)
        .await
        .map_err(|e| internal_error(e).into_response())?
        .ok_or_else(|| Redirect::temporary("/blog").into_response())?;

    if !post.hidden {
        // If the post is not hidden anymore, redirect to the regular post
        return Ok(Redirect::permanent(&format!("/blog/p/{}", post.slug)).into_response());
    }

    Ok(html_template(
        middleware.logged_in,
        PostTemplate {
            middleware,
            metadata: Metadata {
                url,
                title: post.title.clone(),
                description: Some(post.description.clone()),
                image: Some(format!("/img/blog/{}", post.img)),
            },
            post,
        },
    )
    .into_response())
}

pub async fn get_posts_hidden(
    Extension(middleware): Extension<MiddlewareData>,
    url: http::Uri,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let posts = database::get_hidden_posts(&state)
        .await
        .map_err(internal_error)?;

    html_template(
        middleware.logged_in,
        HiddenPostsTemplate {
            middleware,
            metadata: Metadata::only_title(url, "Hidden Posts"),
            posts,
        },
    )
}

pub async fn get_editor() -> impl IntoResponse {
    // Custom security headers because we skip middleware.
    // Lax CSP required due to https://github.com/microsoft/monaco-editor/issues/271
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        "default-src 'self'; worker-src blob:; style-src 'self' 'unsafe-inline'"
            .parse()
            .unwrap(),
    );
    headers.insert(header::X_FRAME_OPTIONS, "SAMEORIGIN".parse().unwrap());
    headers.insert(header::REFERRER_POLICY, "origin".parse().unwrap());
    headers.insert("Cross-Origin-Opener-Policy", "same-origin".parse().unwrap());
    headers.insert(
        "Cross-Origin-Resource-Policy",
        "same-origin".parse().unwrap(),
    );
    (headers, html_template(false, EditorTemplate {}))
}

pub async fn get_new_post(
    Extension(middleware): Extension<MiddlewareData>,
    url: http::Uri,
    State(state): State<AppState>,
    Query(ParentParam { parent }): Query<ParentParam>,
) -> Result<impl IntoResponse, StatusCode> {
    let folders = database::get_folders(&state)
        .await
        .map_err(internal_error)?;
    let all_tags = database::get_tags(&state).await.map_err(internal_error)?;
    html_template(
        middleware.logged_in,
        NewPostTemplate {
            middleware,
            metadata: Metadata::only_title(url, "New Post"),
            parent,
            existing_post: None,
            folders,
            all_tags,
        },
    )
}

pub async fn post_new_post(
    State(state): State<AppState>,
    Json(post): Json<CreatePost>,
) -> Result<Json<ResultUrl>, StatusCode> {
    let slug = extend_slug(&post.slug, post.folder, &state)
        .await
        .map_err(internal_error)?;

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

    Ok(Json(if post.hidden {
        let signature = sign(id, &state.hmac_key);
        ResultUrl::hidden(slug, signature)
    } else {
        ResultUrl::post(slug)
    }))
}

pub async fn get_new_link(
    Extension(middleware): Extension<MiddlewareData>,
    url: http::Uri,
    State(state): State<AppState>,
    Query(ParentParam { parent }): Query<ParentParam>,
) -> Result<impl IntoResponse, StatusCode> {
    let folders = database::get_folders(&state)
        .await
        .map_err(internal_error)?;
    let all_tags = database::get_tags(&state).await.map_err(internal_error)?;
    html_template(
        middleware.logged_in,
        NewLinkTemplate {
            middleware,
            metadata: Metadata::only_title(url, "New Link"),
            parent,
            existing_link: None,
            folders,
            all_tags,
        },
    )
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
    Extension(middleware): Extension<MiddlewareData>,
    url: http::Uri,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let existing_post = database::get_post_as_admin(&state, id)
        .await
        .map_err(internal_error)?
        .ok_or(StatusCode::NOT_FOUND)?;
    let folders = database::get_folders(&state)
        .await
        .map_err(internal_error)?;
    let tags = database::get_tags(&state).await.map_err(internal_error)?;

    html_template(
        middleware.logged_in,
        NewPostTemplate {
            middleware,
            metadata: Metadata::only_title(url, &format!("Edit {}", existing_post.title)),
            parent: None,
            existing_post: Some(existing_post),
            folders,
            all_tags: tags,
        },
    )
}

pub async fn put_edit_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(post): Json<CreatePost>,
) -> Result<Json<ResultUrl>, StatusCode> {
    let slug = extend_slug(&post.slug, post.folder, &state)
        .await
        .map_err(internal_error)?;

    let original_post = sqlx::query!("SELECT id, hidden, slug FROM posts WHERE id = $1", id)
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

    if original_post.hidden && !post.hidden {
        // If the post was hidden and is now visible, reset the timestamp
        sqlx::query!(
            "UPDATE posts SET timestamp = NOW() WHERE id = $1",
            original_post.id
        )
        .execute(&state.db)
        .await
        .map_err(internal_error)?;
    }

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

    Ok(Json(if post.hidden {
        let signature = sign(id, &state.hmac_key);
        ResultUrl::hidden(slug, signature)
    } else {
        ResultUrl::post(slug)
    }))
}

pub async fn get_edit_link(
    Extension(middleware): Extension<MiddlewareData>,
    url: http::Uri,
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
    html_template(
        middleware.logged_in,
        NewLinkTemplate {
            middleware,
            metadata: Metadata::only_title(url, &format!("Edit {}", existing_link.title)),
            parent: None,
            existing_link: Some(existing_link),
            folders,
            all_tags: tags,
        },
    )
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
pub async fn cron(state: &AppState) -> Result<(), sqlx::Error> {
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

pub async fn get_search(
    Extension(middleware): Extension<MiddlewareData>,
    url: http::Uri,
) -> impl IntoResponse {
    html_template(
        middleware.logged_in,
        SearchTemplate {
            middleware,
            metadata: Metadata {
                url,
                title: "Search".to_string(),
                description: Some("Find all posts and search by text.".to_string()),
                image: None,
            },
        },
    )
}

pub async fn get_search_ws(ws: WebSocketUpgrade, state: State<AppState>) -> Response {
    println!("WebSocket: Incoming connection");
    ws.on_upgrade(|socket| async move {
        println!("WebSocket: handling...");
        handle_ws_search(socket, &state).await
    })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WebSocketQuery {
    Search { query: String },
    AllPosts { page: u32 },
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WebSocketResponse {
    /// Fuzzy search for posts by query, returning the top 5 results. '{~highlight~}' is used to highlight matches.
    SearchResults(Vec<PostFull>),
    /// Returns all posts for the given page, with pagination (infinite scroll).
    AllPosts { page: u32, posts: Vec<Post> },
}

pub async fn handle_ws_search(socket: WebSocket, state: &AppState) {
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
    while let Some(Ok(msg)) = rx.next().await {
        println!("WebSocket: Received {msg:?}");
        if let Err(err) = handle_ws_one(msg, tx.clone(), state).await {
            eprintln!("WebSocket Error: {err}");
            break; // Exit on error
        }
    }
}

async fn handle_ws_one(
    msg: Message,
    tx: Arc<Mutex<futures::stream::SplitSink<WebSocket, Message>>>,
    state: &AppState,
) -> Result<(), Box<dyn error::Error>> {
    if let Message::Text(query) = msg {
        println!("           -> Query: {query}");

        let response: WebSocketResponse = match serde_json::from_str::<WebSocketQuery>(&query)? {
            WebSocketQuery::Search { query } => {
                println!("           -> Search query: {query:?}");
                WebSocketResponse::SearchResults(database::search_posts(state, &query).await?)
            }
            WebSocketQuery::AllPosts { page } => {
                println!("           -> All posts request for page {page}");
                let posts = database::get_posts_paginated(state, page).await?;
                WebSocketResponse::AllPosts { page, posts }
            }
        };

        let response = Message::Text(serde_json::to_string(&response).unwrap().into());

        let mut tx = tx.lock().await;
        if tx.send(response).await.is_err() {
            return Err("Failed to send".into());
        };
    }
    Ok(())
}
