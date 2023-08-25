use axum::{extract::State, http::StatusCode, Json};
use axum_sessions::extractors::WritableSession;

use crate::{internal_error, schema::*, AppState};

pub async fn login_check() -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn login(
    mut session: WritableSession,
    State(state): State<AppState>,
    Json(login): Json<Login>,
) -> Result<StatusCode, StatusCode> {
    let password_hash = sqlx::query!("SELECT password_hash FROM users")
        .fetch_one(&state.db)
        .await
        .map_err(internal_error)?
        .password_hash;

    match bcrypt::verify(login.password, &password_hash) {
        Ok(true) => {
            session.insert("logged_in", true).map_err(internal_error)?;
            Ok(StatusCode::NO_CONTENT)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn logout(mut session: WritableSession) -> StatusCode {
    session.destroy();
    StatusCode::NO_CONTENT
}
