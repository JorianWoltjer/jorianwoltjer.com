use std::{env, net::SocketAddr};

use axum::{
    http::HeaderValue,
    routing::{get, post, put},
    Router,
};
use axum_sessions::{async_session::MemoryStore, SessionLayer};
use backend::{handler::*, AppState};
use rand::Rng;

use reqwest::Method;
use sqlx::mysql::MySqlPoolOptions;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{self, TraceLayer},
};
use tracing::Level;

#[tokio::main]
async fn main() {
    println!("Starting server...");
    let port = env::var("PORT").unwrap_or(String::from("8000"));
    let database_url =
        env::var("DATABASE_URL").unwrap_or(String::from("mysql://root@127.0.0.1:3306/website"));

    // Database setup
    let db = MySqlPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!().run(&db).await.unwrap();

    // Session setup
    let mut secret = [0; 64];
    rand::thread_rng().fill(&mut secret);
    let store = MemoryStore::new();
    let session_layer = SessionLayer::new(store, &secret);

    // Logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // Router setup
    let app = Router::new()
        .merge(
            Router::new() // Public
                .route("/health", get(health_check))
                .route("/login", post(login))
                .route("/logout", get(logout))
                .route("/blog/posts", get(get_posts))
                .route("/blog/folders", get(get_folders))
                .route("/blog/post/*slug_or_id", get(get_post))
                .route("/blog/folder/*slug_or_id", get(get_folder)),
        )
        .merge(
            Router::new() // Localhost only
                .route("/render", post(render))
                .route_layer(axum::middleware::from_fn(localhost_only_middleware)),
        )
        .merge(
            Router::new() // Authentication required
                .route("/login", get(login_check))
                .route("/blog/posts", post(create_post))
                .route("/blog/folders", post(create_folder))
                .route("/blog/post/*slug_or_id", put(edit_post))
                .route("/blog/folder/*slug_or_id", put(edit_folder))
                .route_layer(axum::middleware::from_fn(auth_required_middleware)),
        )
        .with_state(AppState { db })
        .layer(
            // for local development
            CorsLayer::new()
                .allow_methods(vec![Method::GET, Method::POST])
                .allow_headers(Any)
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap()),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(session_layer);

    println!("Listening on :{port}...");
    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
