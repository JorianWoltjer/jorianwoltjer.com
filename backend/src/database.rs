use crate::{handler::VerifiedId, schema::*, AppState};

const POSTS_PER_PAGE: u32 = 5;

fn not_found_option<T>(result: Result<T, sqlx::Error>) -> Result<Option<T>, sqlx::Error> {
    match result {
        Ok(value) => Ok(Some(value)),
        Err(sqlx::Error::RowNotFound) => Ok(None),
        Err(e) => Err(e),
    }
}

pub async fn get_folders(state: &AppState) -> Result<Vec<Folder>, sqlx::Error> {
    sqlx::query_as!(
        Folder,
        "SELECT id, parent, slug, title, description, img, timestamp FROM folders ORDER BY id DESC"
    )
    .fetch_all(&state.db)
    .await
}

pub async fn get_categories(state: &AppState) -> Result<Vec<Folder>, sqlx::Error> {
    sqlx::query_as!(
        Folder,
        "SELECT id, parent, slug, title, description, img, timestamp FROM folders WHERE parent IS NULL ORDER BY id ASC"
    )
    .fetch_all(&state.db)
    .await
}

pub async fn get_featured_posts(state: &AppState) -> Result<Vec<Content>, sqlx::Error> {
    let contents_posts = sqlx::query_as!(
        Post,
        r#"SELECT p.id, folder, slug, title, description, img, points, views, featured, hidden, autorelease, timestamp, 
            array(SELECT (t.id, t.name, t.color) FROM post_tags JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
            FROM posts p WHERE NOT hidden AND featured ORDER BY timestamp DESC"#
    )
    .fetch_all(&state.db)
    .await?
    .into_iter()
    .map(Content::Post);

    let contents_links = sqlx::query_as!(
        Link,
        "SELECT id, folder, url, title, description, img, featured, timestamp FROM links 
            WHERE featured ORDER BY timestamp DESC"
    )
    .fetch_all(&state.db)
    .await?
    .into_iter()
    .map(Content::Link);

    let mut contents = contents_posts.chain(contents_links).collect::<Vec<_>>();
    contents.sort(); // Sort by timestamp
    contents.reverse(); // Newest first

    Ok(contents)
}

pub async fn get_post(state: &AppState, slug: &str) -> Result<Option<PostFull>, sqlx::Error> {
    not_found_option(sqlx::query_as!(
        PostFull,
        r#"SELECT p.id, folder, slug, title, description, img, markdown, html, points, views, featured, hidden, autorelease, timestamp, 
            array(SELECT (t.id, t.name, t.color) FROM post_tags JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
            FROM posts p WHERE NOT hidden AND slug = $1"#,
        slug
    )
    .fetch_one(&state.db)
    .await)
}

pub async fn get_post_as_admin(state: &AppState, id: i32) -> Result<Option<PostFull>, sqlx::Error> {
    not_found_option(sqlx::query_as!(
        PostFull,
        r#"SELECT p.id, folder, slug, title, description, img, markdown, html, points, views, featured, hidden, autorelease, timestamp, 
            array(SELECT (t.id, t.name, t.color) FROM post_tags JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
            FROM posts p WHERE p.id = $1"#,
        id
    )
    .fetch_one(&state.db)
    .await)
}

pub async fn get_post_hidden(
    state: &AppState,
    verified_id: VerifiedId,
) -> Result<Option<PostFull>, sqlx::Error> {
    not_found_option(sqlx::query_as!(
        PostFull,
        r#"SELECT p.id, folder, slug, title, description, img, markdown, html, points, views, featured, hidden, autorelease, timestamp, 
            array(SELECT (t.id, t.name, t.color) FROM post_tags JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
            FROM posts p WHERE p.id = $1"#,
        verified_id.0
    )
    .fetch_one(&state.db)
    .await)
}

pub async fn get_hidden_posts(state: &AppState) -> Result<Vec<HiddenPost>, sqlx::Error> {
    let posts = sqlx::query_as!(
        Post,
        r#"SELECT p.id, folder, slug, title, description, img, points, views, featured, hidden, autorelease, timestamp, 
            array(SELECT (t.id, t.name, t.color) FROM post_tags JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
            FROM posts p WHERE hidden ORDER BY timestamp DESC"#
    )
    .fetch_all(&state.db)
    .await?;

    // Add signatures
    Ok(posts
        .into_iter()
        .map(|post| HiddenPost::from_summary(post, state))
        .collect())
}

pub async fn get_post_redirect(
    state: &AppState,
    slug: &str,
) -> Result<Option<String>, sqlx::Error> {
    not_found_option(
        sqlx::query_scalar!(
            r#"SELECT p.slug FROM posts p 
                JOIN post_redirects pr ON p.id = pr.post_id 
                WHERE pr.slug = $1"#,
            slug
        )
        .fetch_one(&state.db)
        .await,
    )
}

pub async fn get_folder(state: &AppState, slug: &str) -> Result<Option<Folder>, sqlx::Error> {
    not_found_option(
        sqlx::query_as!(
            Folder,
            r#"SELECT id, parent, slug, title, description, img, timestamp 
            FROM folders WHERE slug = $1"#,
            slug
        )
        .fetch_one(&state.db)
        .await,
    )
}

pub async fn get_folder_by_id(state: &AppState, id: i32) -> Result<Option<Folder>, sqlx::Error> {
    not_found_option(
        sqlx::query_as!(
            Folder,
            r#"SELECT id, parent, slug, title, description, img, timestamp 
            FROM folders WHERE id = $1"#,
            id
        )
        .fetch_one(&state.db)
        .await,
    )
}

pub async fn get_folder_redirect(
    state: &AppState,
    slug: &str,
) -> Result<Option<String>, sqlx::Error> {
    not_found_option(
        sqlx::query_scalar!(
            r#"SELECT p.slug FROM folders p 
                JOIN folder_redirects pr ON p.id = pr.folder_id 
                WHERE pr.slug = $1"#,
            slug
        )
        .fetch_one(&state.db)
        .await,
    )
}

pub async fn get_link(state: &AppState, id: i32) -> Result<Option<Link>, sqlx::Error> {
    not_found_option(
        sqlx::query_as!(
            Link,
            "SELECT id, folder, url, title, description, img, timestamp, featured FROM links WHERE id = $1",
            id
        )
        .fetch_one(&state.db)
        .await,
    )
}

pub async fn add_view(state: &AppState, id: i32) -> Result<Option<i32>, sqlx::Error> {
    not_found_option(
        sqlx::query_scalar!(
            "UPDATE posts SET views = views + 1 WHERE id = $1 RETURNING views",
            id
        )
        .fetch_one(&state.db)
        .await,
    )
}

pub async fn get_tags(state: &AppState) -> Result<Vec<Tag>, sqlx::Error> {
    sqlx::query_as!(Tag, "SELECT id, name, color FROM tags ORDER BY name")
        .fetch_all(&state.db)
        .await
}

pub async fn search_posts(state: &AppState, query: &str) -> Result<Vec<PostFull>, sqlx::Error> {
    sqlx::query_as!(
        PostFull,
        r#"SELECT p.id, folder, slug, 
        ts_headline('english', title, query, 'StartSel={~, StopSel=~},HighlightAll=true') as "title!", 
        ts_headline('english', description, query, 'StartSel={~, StopSel=~},HighlightAll=true') as "description!", 
        ts_headline('english', plain_text, query, 
        'MaxFragments=2, MaxWords=10, MinWords=5, StartSel={~, StopSel=~}') as "markdown!", 
        '' as "html!", img, points, views, featured, hidden, autorelease, timestamp, 
        array(SELECT (t.id, t.name, t.color) FROM post_tags 
        JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
    FROM posts p JOIN websearch_to_tsquery('english', $1) query 
        ON (numnode(query) = 0 OR query @@ ts)
    WHERE NOT hidden
    ORDER BY ts_rank_cd(ts, query) DESC LIMIT 5"#,
        query
    )
    .fetch_all(&state.db)
    .await
}

pub async fn get_posts_paginated(state: &AppState, page: u32) -> Result<Vec<Post>, sqlx::Error> {
    let offset = (page - 1) * POSTS_PER_PAGE;
    sqlx::query_as!(
        Post,
        r#"SELECT p.id, folder, slug, title, description, img, points, views, featured, hidden, autorelease, timestamp, 
            array(SELECT (t.id, t.name, t.color) FROM post_tags JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
            FROM posts p WHERE NOT hidden ORDER BY timestamp DESC LIMIT $1 OFFSET $2"#,
        POSTS_PER_PAGE as i64,
        offset as i64
    )
    .fetch_all(&state.db)
    .await
}
