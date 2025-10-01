use axum::{
    body::Body,
    http::{header, Method, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
    Extension, RequestPartsExt,
};
use rand::RngCore;
use tower_sessions::Session;

use crate::handler::internal_error;

#[derive(Clone, Debug)]
pub struct MiddlewareData {
    pub logged_in: bool,
    pub nonce: String,
}
impl MiddlewareData {
    pub fn new(logged_in: bool, nonce: String) -> Self {
        MiddlewareData { logged_in, nonce }
    }
}

pub async fn generic_middleware(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let (mut parts, body) = req.into_parts();

    // Extract parts for later
    let is_same_origin = parts
        .headers
        .get("Sec-Fetch-Site")
        .and_then(|v| v.to_str().ok())
        == Some("same-origin");
    let uri = parts.uri.to_string();
    // Generate CSP nonce
    let mut bytes = [0; 16];
    rand::rng().fill_bytes(&mut bytes);
    let nonce = hex::encode(bytes);
    let logged_in = match parts.extract::<Session>().await {
        Ok(session) => session
            .get::<bool>("logged_in")
            .await
            .map_err(internal_error)?
            .unwrap_or(false),
        Err(_) => false,
    };
    let middleware_data = MiddlewareData::new(logged_in, nonce.clone());
    parts.extensions.insert(middleware_data.clone());

    let req = Request::from_parts(parts, body);

    let mut response = next.run(req).await;
    let logged_in = response.extensions().get::<bool>().copied();
    let headers = response.headers_mut();

    // Set security headers
    headers.insert(header::X_CONTENT_TYPE_OPTIONS, "nosniff".parse().unwrap());
    headers.insert(header::X_FRAME_OPTIONS, "DENY".parse().unwrap());
    headers.insert(header::REFERRER_POLICY, "same-origin".parse().unwrap()); // Required for view transitions (Navigation API)
    if !is_same_origin {
        // Set conditionally, because page transitions don't support it
        headers.insert("Cross-Origin-Opener-Policy", "same-origin".parse().unwrap());
    }
    headers.insert(
        "Cross-Origin-Resource-Policy",
        "same-origin".parse().unwrap(),
    );
    // Cross-Origin-Embedder-Policy is not possible due to third-party iframes and lacking support for `credentialless`
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        format!(
            "\
default-src 'none'; \
script-src 'self' 'nonce-{nonce}'; \
style-src 'self'; \
connect-src 'self'; \
img-src 'self' data:; \
font-src 'self'; \
frame-src 'self' https://www.youtube-nocookie.com https://yeswehack.github.io/Dom-Explorer/frame; \
media-src 'self'; \
manifest-src 'self'; \
frame-ancestors 'none'; \
base-uri 'none'; \
form-action 'self'; \
require-trusted-types-for 'script'"
        )
        .parse()
        .unwrap(),
    );
    headers.insert(
        "Permissions-Policy",
        "display-capture=(), geolocation=(), camera=(), microphone=(), payment=(), bluetooth=(), clipboard-read=()"
            .parse()
            .unwrap(),
    );
    headers.insert(
        header::CACHE_CONTROL,
        match logged_in {
            Some(true) => "private, max-age=0".parse().unwrap(),
            Some(false) => "public, max-age=60".parse().unwrap(),
            None => "public, max-age=3600".parse().unwrap(),
        },
    );

    // Redirect to login if 401
    if response.status() == StatusCode::UNAUTHORIZED && uri.split('?').next().unwrap() != "/login" {
        let redirect = format!("/login?back={}", urlencoding::encode(&uri));
        response = Redirect::temporary(&redirect).into_response()
    }

    Ok(response)
}

pub async fn auth_required_middleware(
    Extension(middleware): Extension<MiddlewareData>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let (parts, body) = req.into_parts();

    // Check if user is logged in
    if !middleware.logged_in {
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

    let mut response = next.run(Request::from_parts(parts, body)).await;
    response.extensions_mut().insert(middleware.logged_in);

    Ok(response)
}
