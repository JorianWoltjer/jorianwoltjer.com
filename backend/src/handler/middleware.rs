use std::net::{IpAddr, SocketAddr};

use axum::{
    extract::ConnectInfo,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    RequestPartsExt,
};
use axum_sessions::extractors::ReadableSession;

use crate::is_production;

pub async fn auth_required_middleware<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let (mut parts, body) = req.into_parts();

    // Development mode bypasses authentication
    if !is_production() {
        return Ok(next.run(Request::from_parts(parts, body)).await);
    }

    let session = parts
        .extract::<ReadableSession>()
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !session.get::<bool>("logged_in").unwrap_or(false) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(Request::from_parts(parts, body)).await)
}

pub async fn localhost_only_middleware<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let (mut parts, body) = req.into_parts();

    // Development mode bypasses localhost-only
    if !is_production() {
        return Ok(next.run(Request::from_parts(parts, body)).await);
    }

    let ip = match parts.headers.get("x-forwarded-for") {
        Some(ip) => ip.to_str().unwrap().parse().unwrap(),
        None => parts
            .extract::<ConnectInfo<SocketAddr>>()
            .await
            .map(|h| h.ip())
            .unwrap(),
    };

    if let IpAddr::V4(ip) = ip {
        // 127.0.0.0/8, 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16
        if ip.is_loopback() || ip.is_private() {
            return Ok(next.run(Request::from_parts(parts, body)).await);
        }
    }

    // Also bypass if logged in
    auth_required_middleware(Request::from_parts(parts, body), next).await
}
