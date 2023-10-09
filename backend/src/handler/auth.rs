use aide::NoApi;
use axum::{extract::State, http::StatusCode, Json};
use axum_sessions::extractors::WritableSession;

use crate::{handler::internal_error, schema::*, AppState};

pub async fn login_check() -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn login(
    NoApi(mut session): NoApi<WritableSession>,
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
            session.insert("logged_in", true).map_err(internal_error)?;
            Ok(StatusCode::NO_CONTENT)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn logout(NoApi(mut session): NoApi<WritableSession>) -> StatusCode {
    session.destroy();
    StatusCode::NO_CONTENT
}
