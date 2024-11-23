use std::env;

use aide::{
    axum::{
        routing::{get, post, put},
        ApiRouter, IntoApiResponse,
    },
    openapi::{Info, OpenApi, Server},
};
use axum::{Extension, Json};
use backend::{handler::*, is_production, AppState};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::{self, TraceLayer},
};
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tracing::Level;

async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}

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
    let mut app = ApiRouter::new()
        // Only api_route() routes will be included in documentation
        .route("/swagger.json", get(serve_api))
        .nest_service("/", ServeDir::new("static"))
        .merge(
            ApiRouter::new() // Public
                .route("/login", post(login))
                .route("/logout", get(logout))
                .api_route("/projects", get(get_projects))
                .api_route("/blog/folders", get(get_folders))
                .api_route("/blog/posts", get(get_posts))
                .api_route("/blog/folder/*slug_or_id", get(get_folder))
                .api_route("/blog/post/*slug_or_id", get(get_post))
                .api_route("/blog/hidden/*slug_or_id", get(get_hidden_post))
                .route("/blog/add_view", post(add_view))
                .api_route("/blog/featured", get(get_featured_posts))
                .api_route("/blog/tags", get(get_tags))
                .api_route("/blog/search", get(ws_search)),
        )
        .merge(
            ApiRouter::new() // Internal-only
                .route("/render", post(render))
                .route("/blog/revalidate", post(revalidate))
                .route_layer(axum::middleware::from_fn(internal_only_middleware)),
        )
        .merge(
            ApiRouter::new() // Authentication required
                .route("/check", get(login_check).post(login_check))
                .route("/blog/preview", post(preview))
                .route("/blog/folders", post(create_folder))
                .route("/blog/posts", post(create_post))
                .route("/blog/hidden", get(get_hidden_posts))
                .route("/blog/folder/*slug_or_id", put(edit_folder))
                .route("/blog/post/*slug_or_id", put(edit_post))
                .route_layer(axum::middleware::from_fn(auth_required_middleware)),
        )
        .with_state(AppState { db, hmac_key })
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(session_layer);

    let mut api = OpenApi {
        info: Info {
            description: Some("Blog API (auto-generated)".to_string()),
            ..Info::default()
        },
        servers: vec![
            Server {
                url: String::from("https://jorianwoltjer.com/api"),
                ..Server::default()
            },
            Server {
                url: String::from("http://localhost/api"),
                ..Server::default()
            },
        ],
        ..OpenApi::default()
    };

    if !is_production() {
        println!("WARNING: Running in development mode, disabling security features.");
        app = app.layer(
            CorsLayer::new()
                .allow_methods(Any)
                .allow_headers(Any)
                .allow_origin(Any),
        );
    }

    println!("Listening on :{port}...");
    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("Failed to bind to port");
    axum::serve(
        listener,
        app.finish_api(&mut api)
            .layer(Extension(api))
            .into_make_service(),
    )
    .await
    .unwrap();
}
