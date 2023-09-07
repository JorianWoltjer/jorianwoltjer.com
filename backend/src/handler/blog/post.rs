use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
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
            array(SELECT (t.id, t.name, t.color) FROM post_tags JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
            FROM posts p WHERE p.id = $1"#,
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
            array(SELECT (t.id, t.name, t.color) FROM post_tags JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
            FROM posts p"#
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
            array(SELECT (t.id, t.name, t.color) FROM post_tags JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
            FROM posts p WHERE p.id::varchar = $1 OR slug = $1"#,
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
            array(SELECT (t.id, t.name, t.color) FROM post_tags JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
            FROM posts p WHERE featured = true"#
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

pub async fn ws_search(ws: WebSocketUpgrade, state: State<AppState>) -> impl IntoResponse {
    println!("WebSocket: Incoming connection");
    ws.on_upgrade(|socket| async move {
        println!("WebSocket: handling...");
        handle_ws_search(socket, state).await
    })
}

/// Fuzzy search for posts by query, returning the top 5 results. '{~highlight~}' is used to highlight matches.
pub async fn handle_ws_search(mut socket: WebSocket, State(state): State<AppState>) {
    while let Some(Ok(msg)) = socket.recv().await {
        println!("WebSocket: Received {msg:?}");
        if let Message::Text(query) = msg {
            println!("           -> Query: {}", query);
            match sqlx::query_as!(Post, r#"SELECT p.id, folder, slug, 
                    ts_headline('english', title, query, 'StartSel={~, StopSel=~}') as "title!", 
                    ts_headline('english', description, query, 'StartSel={~, StopSel=~}') as "description!", 
                    ts_headline('english', plain_text, query, 'MaxFragments=2, MaxWords=10, MinWords=5, StartSel={~, StopSel=~}') as "markdown!", 
                    img, points, views, featured, timestamp, 
                    array(SELECT (t.id, t.name, t.color) FROM post_tags JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
                    FROM posts p JOIN websearch_to_tsquery('english', $1) query ON (numnode(query) = 0 OR query @@ ts)
                    ORDER BY ts_rank_cd(ts, query) DESC LIMIT 5"#,
                query)
                .fetch_all(&state.db)
                .await {
                    Ok(results) => {
                        println!("           -> Sending {} results", results.len());
                        let response = Message::Text(serde_json::to_string(&results).unwrap());
            
                        if socket.send(response).await.is_err() {
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
