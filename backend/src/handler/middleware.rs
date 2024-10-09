use std::env;

use axum::{
    http::{Request, StatusCode},
    body::Body,
    middleware::Next,
    response::Response,
    RequestPartsExt,
};
// use axum_sessions::extractors::ReadableSession;
use tower_sessions::Session;

use crate::is_production;

pub async fn auth_required_middleware<B>(
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let (mut parts, body) = req.into_parts();

    // Development mode bypasses authentication
    if !is_production() {
        return Ok(next.run(Request::from_parts(parts, body)).await);
    }

    let session = parts
        .extract::<Session>()
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !session.get::<bool>("logged_in").await.unwrap_or_default().unwrap() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(Request::from_parts(parts, body)).await)
}

pub async fn internal_only_middleware<B>(
    req: Request<B>,
    next: Next,
) -> Result<Response, StatusCode> {
    let (parts, body) = req.into_parts();

    // Development mode bypasses
    if !is_production() {
        return Ok(next.run(Request::from_parts(parts, body)).await);
    }

    // X-Internal header is set to "false" by nginx, only internal requests can set it to the correct token
    let is_internal = match parts.headers.get("X-Internal") {
        Some(header_value) => header_value.to_str().unwrap_or_default() == env::var("INTERNAL_TOKEN").unwrap(),
        None => false,
    };
    if is_internal {
        return Ok(next.run(Request::from_parts(parts, body)).await);
    }

    // Also bypass if logged in
    auth_required_middleware(Request::from_parts(parts, body), next).await
}
