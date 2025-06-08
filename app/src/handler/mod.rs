pub mod auth;
pub mod blog;
pub mod middleware;

use std::sync::LazyLock;

use crate::{database, html_template, templates::*, AppState};
use askama::Template;
use axum::extract::{Path, State};
use axum::http::{self, header, HeaderMap};
use axum::response::Redirect;
use axum::Extension;
use axum::{http::StatusCode, response::IntoResponse};
use chrono::Utc;
use fancy_regex::Regex;
use grass::InputSyntax;

pub use self::auth::*;
pub use self::blog::*;
pub use self::middleware::*;

pub static COMPILED_CSS: LazyLock<String> = LazyLock::new(|| {
    let scss = include_str!("../../static/assets/css/style.css");
    // Unfortunately have to compile here because older iOS devices don't support CSS nesting (https://caniuse.com/css-nesting)
    grass::from_string(
        scss,
        &grass::Options::default().input_syntax(InputSyntax::Scss),
    )
    .unwrap()
});
pub static IMG_SRC_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9\-_/]+\.[a-zA-Z0-9]+$").unwrap());

pub fn internal_error(e: impl std::fmt::Display) -> StatusCode {
    eprintln!("500 Internal Server Error: {}", e);
    StatusCode::INTERNAL_SERVER_ERROR
}

pub fn sql_not_found(e: sqlx::Error) -> StatusCode {
    match e {
        sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
        _ => internal_error(e),
    }
}

pub async fn get_home(
    Extension(middleware): Extension<MiddlewareData>,
    url: http::Uri,
) -> impl IntoResponse {
    html_template(
        middleware.logged_in,
        HomeTemplate {
            middleware,
            metadata: Metadata {
                url,
                title: "Home".to_string(),
                description: Some(
                    "My personal website. Here you can read the blog, find links, and more."
                        .to_string(),
                ),
                image: None,
            },
        },
    )
}

pub async fn get_about(
    Extension(middleware): Extension<MiddlewareData>,
    url: http::Uri,
) -> impl IntoResponse {
    html_template(
        middleware.logged_in,
        AboutTemplate {
            middleware,
            metadata: Metadata {
                url,
                title: "About".to_string(),
                description: Some(
                    "Some backstory about who I am and how I got started, as well as this website."
                        .to_string(),
                ),
                image: None,
            },
        },
    )
}

pub async fn get_contact(
    Extension(middleware): Extension<MiddlewareData>,
    url: http::Uri,
) -> impl IntoResponse {
    html_template(
        middleware.logged_in,
        ContactTemplate {
            middleware,
            metadata: Metadata {
                url,
                title: "Contact".to_string(),
                description: Some(
                    "Links to my social media profiles in order to get in contact with me."
                        .to_string(),
                ),
                image: None,
            },
        },
    )
}

pub async fn error_404(
    Extension(middleware): Extension<MiddlewareData>,
    url: http::Uri,
) -> impl IntoResponse {
    html_template(
        middleware.logged_in,
        Error404Template {
            middleware,
            metadata: Metadata::only_title(url.clone(), "404 Not Found"),
            url: url.to_string(),
        },
    )
}

pub async fn get_style_css() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
    (headers, COMPILED_CSS.to_string())
}

/// Mock for if Cloudflare isn't available
pub async fn get_cdn_image(
    Path((_options, path)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    if !IMG_SRC_REGEX.is_match(&path).unwrap() {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(Redirect::temporary(&format!("/img/blog/{path}")))
}

pub async fn get_sitemap(State(state): State<AppState>) -> Result<(HeaderMap, String), StatusCode> {
    let posts = database::get_posts(&state).await.map_err(internal_error)?;
    let folder = database::get_folders(&state)
        .await
        .map_err(internal_error)?;
    let mut urls = vec![];
    let latest_timestamp = posts
        .iter()
        .map(|post| post.timestamp)
        .chain(folder.iter().map(|f| f.timestamp))
        .max()
        .unwrap_or_else(Utc::now);
    for path in &["/", "/blog", "/about", "/contact", "/blog/search"] {
        urls.push(SitemapUrl {
            path: path.to_string(),
            timestamp: latest_timestamp,
        });
    }
    for post in posts {
        urls.push(SitemapUrl {
            path: format!("/blog/p/{}", post.slug),
            timestamp: post.timestamp,
        });
    }
    for folder in folder {
        urls.push(SitemapUrl {
            path: format!("/blog/f/{}", folder.slug),
            timestamp: folder.timestamp,
        });
    }

    let template = SitemapTemplate { urls };
    let xml = template.render().map_err(internal_error)?;
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/rss+xml".parse().unwrap());

    Ok((headers, xml))
}
