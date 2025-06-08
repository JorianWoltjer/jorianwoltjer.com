use std::{env, time::Duration};

use app::{
    handler::{folder::*, post::*, *},
    AppState,
};
use axum::{
    handler::HandlerWithoutStateExt,
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use tokio::{net::TcpListener, time};
use tower::ServiceBuilder;
use tower_http::{
    services::ServeDir,
    trace::{self, TraceLayer},
};
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tracing::Level;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    println!("Starting server...");
    let port = env::var("PORT").unwrap_or(String::from("8000"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Database setup
    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!().run(&db).await.unwrap();

    let hmac_key = hex::decode(
        sqlx::query!("SELECT value FROM secrets WHERE name = 'hmac_key'")
            .fetch_one(&db)
            .await
            .expect("Failed to retrieve hmac_key")
            .value,
    )
    .expect("Invalid hex string for hmac_key")
    .try_into()
    .expect("Should be 32 bytes (64 hex chars)");

    sqlx::query!("UPDATE posts SET cached_views = views")
        .execute(&db)
        .await
        .unwrap();

    // Session setup
    let store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(store).with_name("session");

    // Logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // Router setup
    let state = AppState { db, hmac_key };
    let app = Router::new()
        .merge(
            Router::new() // Public
                .route("/", get(get_home))
                .route("/login", get(get_login).post(post_login))
                .route("/about", get(get_about))
                .route("/contact", get(get_contact))
                .route("/blog", get(get_blog))
                .route("/blog/f/{*slug}", get(get_folder))
                .route("/blog/p/{*slug}", get(get_post))
                .route("/blog/h/{*slug}", get(get_post_hidden))
                .route("/blog/search", get(get_search))
                .route("/blog/search_ws", get(get_search_ws))
                .route("/blog/add_view/{id}", post(post_add_view))
                .route("/blog/rss.xml", get(get_rss))
                .route("/sitemap.xml", get(get_sitemap)),
        )
        .merge(
            Router::new() // Authentication required
                .route("/logout", post(post_logout))
                .route("/blog/admin/preview", post(post_preview))
                .route("/blog/admin/hidden", get(get_posts_hidden))
                .route(
                    "/blog/admin/folder",
                    get(get_new_folder).post(post_new_folder),
                )
                .route("/blog/admin/post", get(get_new_post).post(post_new_post))
                .route("/blog/admin/link", get(get_new_link).post(post_new_link))
                .route(
                    "/blog/admin/folder/{id}",
                    get(get_edit_folder).put(put_edit_folder),
                )
                .route(
                    "/blog/admin/post/{id}",
                    get(get_edit_post).put(put_edit_post),
                )
                .route(
                    "/blog/admin/link/{id}",
                    get(get_edit_link).put(put_edit_link),
                )
                .route_layer(axum::middleware::from_fn(auth_required_middleware)),
        )
        .with_state(state.clone())
        .route("/assets/css/style.css", get(get_style_css))
        .route(
            "/cdn-cgi/image/{options}/img/blog/{*path}",
            get(get_cdn_image),
        )
        .fallback_service(ServeDir::new("static").not_found_service(error_404.into_service()))
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                        .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
                )
                .layer(session_layer)
                .layer(axum::middleware::from_fn(generic_middleware)),
        )
        // Routes without middleware
        .route("/blog/admin/editor", get(get_editor));

    // Start background task
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            if let Err(err) = cron(&state).await {
                tracing::error!("Cron job failed: {err}");
            };
        }
    });
    // Start server
    println!("Listening on :{port}...");
    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("Failed to bind to port");
    axum::serve(listener, app).await.unwrap();
}
