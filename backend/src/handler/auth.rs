use aide::NoApi;
use axum::{extract::State, http::StatusCode, Json};
use tower_sessions::Session;

use crate::{handler::internal_error, schema::*, AppState};

pub async fn login_check() -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn login(
    NoApi(session): NoApi<Session>,
    State(state): State<AppState>,
    Json(login): Json<Login>,
) -> Result<StatusCode, StatusCode> {
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
            Ok(StatusCode::NO_CONTENT)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn logout(NoApi(session): NoApi<Session>) -> Result<StatusCode, StatusCode> {
    session.delete().await.map_err(internal_error)?;
    Ok(StatusCode::NO_CONTENT)
}
