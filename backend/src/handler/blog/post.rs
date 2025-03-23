use std::{sync::Arc, time::Duration};

use aide::axum::IntoApiResponse;
use axum::{
    body::Bytes, extract::{
        ws::{Message, WebSocket},
        Path, Query, State, WebSocketUpgrade,
    }, http::StatusCode, Json
};
use futures::{lock::Mutex, sink::SinkExt, stream::StreamExt};
use hmac::{Hmac, Mac};
use schemars::JsonSchema;
use serde::Deserialize;
use sha2::Sha256;
use tokio::time;

use crate::{
    extend_slug,
    handler::{internal_error, sql_not_found},
    schema::*,
    AppState, RevalidationRequest, Slug,
};

async fn get_hidden_post_summary(id: i32, state: &AppState) -> Result<HiddenPost, sqlx::Error> {
    sqlx::query_as!(
        PostSummary,
        r#"SELECT p.id, folder, slug, title, description, img, points, views, featured, hidden, autorelease, timestamp, 
            array(SELECT (t.id, t.name, t.color) FROM post_tags JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
            FROM posts p WHERE p.id = $1"#,
        id
    )
    .fetch_one(&state.db)
    .await
    .map(|post| HiddenPost::from_summary(post, state))
}

pub fn sign(id: i32, hmac_key: &[u8]) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(hmac_key).unwrap();
    mac.update(id.to_string().as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

pub async fn verify_signature(
    slug_or_id: &str,
    signature: &str,
    state: &AppState,
) -> Result<i32, StatusCode> {
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
        return Ok(id);
    }

    Err(StatusCode::UNAUTHORIZED)
}

// Routes

pub async fn get_posts(
    State(state): State<AppState>,
) -> Result<Json<Vec<PostSummary>>, StatusCode> {
    sqlx::query_as!(
        PostSummary,
        r#"SELECT p.id, folder, slug, title, description, img, points, views, featured, hidden, autorelease, timestamp, 
            array(SELECT (t.id, t.name, t.color) FROM post_tags JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
            FROM posts p WHERE NOT hidden ORDER BY timestamp DESC"#
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
    if let Ok(post) = sqlx::query_as!(
        Post,
        r#"SELECT p.id, folder, slug, title, description, img, markdown, points, views, featured, hidden, autorelease, timestamp, 
            array(SELECT (t.id, t.name, t.color) FROM post_tags JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
            FROM posts p WHERE NOT hidden AND (p.id::varchar = $1 OR slug = $1)"#,
        slug_or_id
    )
    .fetch_one(&state.db)
    .await {
        Ok(Json(post))
    } else {
        Ok(Json(
            sqlx::query_as!(
                Post,
                r#"SELECT p.id, folder, p.slug, title, description, img, markdown, points, views, featured, hidden, autorelease, timestamp, 
                    array(SELECT (t.id, t.name, t.color) FROM post_tags JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
                    FROM posts p 
                    JOIN post_redirects pr ON p.id = pr.post_id 
                    WHERE NOT hidden AND (pr.slug = $1)"#,
                slug_or_id
            )
            .fetch_one(&state.db)
            .await
            .map_err(sql_not_found)?,
        ))
    }
}

pub async fn get_hidden_posts(
    State(state): State<AppState>,
) -> Result<Json<Vec<HiddenPost>>, StatusCode> {
    sqlx::query_as!(
        PostSummary,
        r#"SELECT p.id, folder, slug, title, description, img, points, views, featured, hidden, autorelease, timestamp, 
            array(SELECT (t.id, t.name, t.color) FROM post_tags JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
            FROM posts p WHERE hidden ORDER BY timestamp DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(internal_error)
    .map(|posts| posts.into_iter().map(|post| HiddenPost::from_summary(post, &state)).collect())
    .map(Json)
}

#[derive(Deserialize, JsonSchema)]
pub struct HiddenRequest {
    signature: String,
}

pub async fn get_hidden_post(
    State(state): State<AppState>,
    Path(slug_or_id): Path<String>,
    Query(HiddenRequest { signature }): Query<HiddenRequest>,
) -> Result<Json<Post>, StatusCode> {
    let id = verify_signature(&slug_or_id, &signature, &state).await?;

    sqlx::query_as!(
        Post,
        r#"SELECT p.id, folder, slug, title, description, img, markdown, points, views, featured, hidden, autorelease, timestamp, 
            array(SELECT (t.id, t.name, t.color) FROM post_tags JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
            FROM posts p WHERE p.id = $1"#,
        id
    )
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)
    .map(Json)
}

pub async fn create_post(
    State(state): State<AppState>,
    Json(post): Json<CreatePost>,
) -> Result<Json<HiddenPost>, StatusCode> {
    let slug = extend_slug(&post.slug, post.folder, &state)
        .await
        .map_err(internal_error)?;

    let id = sqlx::query!(
        "INSERT INTO posts (folder, title, slug, description, img, points, featured, hidden, autorelease, markdown) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
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
    RevalidationRequest::new(vec![Slug::Post { slug }])
        .execute()
        .await
        .map_err(internal_error)?;

    get_hidden_post_summary(id, &state)
        .await
        .map_err(internal_error)
        .map(Json)
}

pub async fn edit_post(
    State(state): State<AppState>,
    Path(slug_or_id): Path<String>,
    Json(post): Json<CreatePost>,
) -> Result<Json<HiddenPost>, StatusCode> {
    let slug = extend_slug(&post.slug, post.folder, &state)
        .await
        .map_err(internal_error)?;

    let original_post = sqlx::query!(
        "SELECT id, slug FROM posts WHERE id::varchar = $1 OR slug = $1",
        slug_or_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)?;

    let mut revalidations = RevalidationRequest::new(vec![
        Slug::Post {
            slug: original_post.slug.clone(),
        },
        Slug::Post { slug: slug.clone() },
    ]);

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
        revalidations.slugs.extend(
            post_revalidations
                .into_iter()
                .map(|record| Slug::Post { slug: record.slug }),
        );
    }

    sqlx::query!(
        "UPDATE posts SET folder = $1, title = $2, slug = $3, description = $4, img = $5, points = $6, featured = $7, hidden = $8, autorelease = $9, markdown = $10 WHERE id = $11",
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
    revalidations.execute().await.map_err(internal_error)?;

    get_hidden_post_summary(original_post.id, &state)
        .await
        .map_err(internal_error)
        .map(Json)
}

pub async fn get_featured_posts(
    State(state): State<AppState>,
) -> Result<Json<Vec<PostSummary>>, StatusCode> {
    sqlx::query_as!(
        PostSummary,
        r#"SELECT p.id, folder, slug, title, description, img, points, views, featured, hidden, autorelease, timestamp, 
            array(SELECT (t.id, t.name, t.color) FROM post_tags JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
            FROM posts p WHERE NOT hidden AND (featured) ORDER BY timestamp DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(internal_error)
    .map(Json)
}

#[derive(Deserialize, JsonSchema)]
pub struct AddViewRequest {
    id: i32,
    signature: Option<String>,
}

pub async fn add_view(
    State(state): State<AppState>,
    Json(AddViewRequest { id, signature }): Json<AddViewRequest>,
) -> Result<(), StatusCode> {
    let signature_valid = match signature {
        Some(signature) => verify_signature(&id.to_string(), &signature, &state).await? == id,
        None => false,
    };

    sqlx::query!(
        "UPDATE posts SET views = views + 1 WHERE id = $1 AND (NOT hidden OR $2)",
        id,
        signature_valid
    )
    .execute(&state.db)
    .await
    .map_err(internal_error)?;

    Ok(())
}

/// Called every minute by a cron job. This does:
/// 1. Revalidate views
/// 2. Auto-Release posts
pub async fn revalidate(State(state): State<AppState>) -> Result<StatusCode, StatusCode> {
    // Revalidate views
    let mut revalidations =
        sqlx::query!("SELECT slug FROM posts WHERE NOT hidden AND (views != cached_views)")
            .fetch_all(&state.db)
            .await
            .map_err(internal_error)
            .map(|records| {
                RevalidationRequest::new(
                    records
                        .into_iter()
                        .map(|record| Slug::Post { slug: record.slug })
                        .collect(),
                )
            })?;

    if !revalidations.slugs.is_empty() {
        sqlx::query!("UPDATE posts SET cached_views = views")
            .execute(&state.db)
            .await
            .map_err(internal_error)?;
    }

    // Auto-Release posts
    let released_posts = sqlx::query!(
        "UPDATE posts SET hidden = false, timestamp = autorelease, autorelease = NULL WHERE autorelease <= NOW() AND hidden RETURNING slug"
    )
    .fetch_all(&state.db)
    .await
    .map_err(internal_error)?;

    revalidations
        .slugs
        .extend(released_posts.iter().map(|record| Slug::Post {
            slug: record.slug.clone(),
        }));

    if released_posts.is_empty() {
        revalidations.views_only = true;
    }

    println!("Revalidating {} posts", revalidations.slugs.len());
    revalidations.execute().await.map_err(internal_error)?;

    Ok(StatusCode::OK)
}

pub async fn get_tags(State(state): State<AppState>) -> Result<Json<Vec<Tag>>, StatusCode> {
    sqlx::query_as!(Tag, "SELECT * FROM tags ORDER BY id")
        .fetch_all(&state.db)
        .await
        .map_err(internal_error)
        .map(Json)
}

pub async fn ws_search(ws: WebSocketUpgrade, state: State<AppState>) -> impl IntoApiResponse {
    println!("WebSocket: Incoming connection");
    ws.on_upgrade(|socket| async move {
        println!("WebSocket: handling...");
        handle_ws_search(socket, state).await
    })
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
            match sqlx::query_as!(Post, r#"SELECT p.id, folder, slug, 
                    ts_headline('english', title, query, 'StartSel={~, StopSel=~}') as "title!", 
                    ts_headline('english', description, query, 'StartSel={~, StopSel=~}') as "description!", 
                    ts_headline('english', plain_text, query, 'MaxFragments=2, MaxWords=10, MinWords=5, StartSel={~, StopSel=~}') as "markdown!", 
                    img, points, views, featured, hidden, autorelease, timestamp, 
                    array(SELECT (t.id, t.name, t.color) FROM post_tags JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
                    FROM posts p JOIN websearch_to_tsquery('english', $1) query ON (numnode(query) = 0 OR query @@ ts)
                    WHERE NOT hidden
                    ORDER BY ts_rank_cd(ts, query) DESC LIMIT 5"#,
                query.to_string())
                .fetch_all(&state.db)
                .await {
                    Ok(results) => {
                        println!("           -> Sending {} results", results.len());
                        let response = Message::Text(serde_json::to_string(&results).unwrap().into());
            
                        let mut tx = tx_search.lock().await;
                        if tx.send(response).await.is_err() {
                            break; // Connection closed
                        };
                    },
                    Err(e) => {
                        eprintln!("WebSocket Error: {}", e);
                        break;
                    }
                }
        }
    }
}
