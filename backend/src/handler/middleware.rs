use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
    middleware::Next,
    response::Response,
    RequestPartsExt,
};
use rand::RngCore;
use tower_sessions::Session;

use crate::{handler::internal_error, is_production};

pub async fn auth_required_middleware(
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

    if !session
        .get::<bool>("logged_in")
        .await
        .map_err(internal_error)?
        .unwrap_or(false)
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // CSRF protection (disallow non-GET requests not from same-origin)
    let sec_fetch_site = parts
        .headers
        .get("Sec-Fetch-Site")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("none");
    if parts.method != Method::GET && ["cross-site", "same-site"].contains(&sec_fetch_site) {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(Request::from_parts(parts, body)).await)
}

pub async fn response_headers_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Generate CSP nonce
    let (mut parts, body) = req.into_parts();
    let is_same_origin = parts
        .headers
        .get("Sec-Fetch-Site")
        .and_then(|v| v.to_str().ok())
        == Some("same-origin");
    let mut bytes = [0; 16];
    rand::rng().fill_bytes(&mut bytes);
    let nonce = hex::encode(bytes);
    parts.extensions.insert(nonce.clone());
    let req = Request::from_parts(parts, body);

    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    // Set security headers
    headers.insert("X-Content-Type-Options", "nosniff".parse().unwrap());
    headers.insert("X-Frame-Options", "DENY".parse().unwrap());
    headers.insert("Referrer-Policy", "origin".parse().unwrap());
    if !is_same_origin {
        // Set conditionally, because page transitions don't support it
        headers.insert("Cross-Origin-Opener-Policy", "same-origin".parse().unwrap());
    }
    headers.insert(
        "Cross-Origin-Resource-Policy",
        "same-origin".parse().unwrap(),
    );
    headers.insert(
            "Content-Security-Policy",
            format!("\
    default-src 'self'; \
    script-src 'self' 'nonce-{nonce}'; \
    style-src 'self' https://fonts.googleapis.com; \
    object-src 'none'; \
    connect-src 'self' https://fonts.googleapis.com https://fonts.gstatic.com; \
    font-src 'self' https://fonts.gstatic.com; \
    img-src 'self' data:; \
    frame-src https://www.youtube-nocookie.com https://yeswehack.github.io/Dom-Explorer/dom-explorer/frame; \
    frame-ancestors 'none'; \
    base-uri 'self'; \
    form-action 'self'")
                .parse()
                .unwrap(),
        );

    Ok(response)
}
