use std::env;

use axum::{
    routing::{get, post},
    Router,
};
use axum_sessions::{async_session, SessionLayer};
use backend::{
    handler::{create_post, get_posts, health_check},
    AppState,
};
use rand::RngCore;
use sqlx::mysql::MySqlPoolOptions;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or(String::from("8000"));
    let database_url =
        env::var("DATABASE_URL").unwrap_or(String::from("mysql://root@127.0.0.1:3306/website"));

    // Database setup
    let db = MySqlPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
        .unwrap();

    sqlx::migrate!().run(&db).await.unwrap();

    // Session setup
    let store = async_session::MemoryStore::new();
    let mut secret = [0u8; 64];
    rand::thread_rng().fill_bytes(&mut secret);
    let _session_layer = SessionLayer::new(store, &secret);

    // Logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // Routes and layers
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/query", get(get_posts))
        .route("/create", post(create_post))
        .with_state(AppState { db })
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    println!("Listening on :{port}...");
    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
