use regex::Regex;
use reqwest::StatusCode;
use serde::Serialize;
use sqlx::MySqlPool;

pub mod handler;
pub mod schema;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
}

pub fn internal_error(e: impl std::fmt::Display) -> StatusCode {
    eprintln!("500 Internal Server Error: {}", e);
    StatusCode::INTERNAL_SERVER_ERROR
}

pub fn slugify(title: &str) -> String {
    Regex::new(r"((&.*?;)|[^\w])+")
        .unwrap()
        .replace_all(title, "-")
        .trim_matches('-')
        .to_string()
        .to_lowercase()
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Slug {
    Post { slug: String },
    Folder { slug: String },
}

/// Revalidate a slug in NextJS static pages
pub async fn revalidate(slug: Slug) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .post("http://frontend/api/revalidate")
        .json(&slug)
        .send()
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("!Hello World!"), "hello-world");
        assert_eq!(slugify("Hello, World!"), "hello-world");
        assert_eq!(
            slugify("!!! Something, <u> with &gt; is back..."),
            "something-u-with-is-back"
        );
    }
}
