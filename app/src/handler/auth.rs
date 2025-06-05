use axum::{
    extract::State,
    http::{self, HeaderMap, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use tower_sessions::Session;

use crate::{handler::internal_error, html_template, schema::*, templates::*, AppState};

use super::MiddlewareData;

pub async fn login_check() -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn get_login(
    Extension(middleware): Extension<MiddlewareData>,
    url: http::Uri,
) -> impl IntoResponse {
    html_template(
        middleware.logged_in,
        LoginTemplate {
            middleware,
            metadata: Metadata::only_title(url, "Login"),
        },
    )
}

pub async fn post_login(
    session: Session,
    State(state): State<AppState>,
    Json(login): Json<Login>,
) -> Result<HeaderMap, StatusCode> {
    let password_hash = sqlx::query!("SELECT value FROM secrets WHERE name = 'password_hash'")
        .fetch_one(&state.db)
        .await
        .map_err(internal_error)?
        .value;

    match bcrypt::verify(login.password, &password_hash) {
        Ok(true) => {
            session
                .insert("logged_in", true)
                .await
                .map_err(internal_error)?;
            let mut headers = HeaderMap::new();
            headers.insert("Clear-Site-Data", "\"cache\"".parse().unwrap());
            Ok(headers)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn post_logout(session: Session) -> Result<HeaderMap, StatusCode> {
    session.delete().await.map_err(internal_error)?;
    let mut headers = HeaderMap::new();
    headers.insert("Clear-Site-Data", "\"*\"".parse().unwrap());
    Ok(headers)
}
