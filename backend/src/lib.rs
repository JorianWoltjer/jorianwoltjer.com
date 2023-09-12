use std::env;

use regex::Regex;
use serde::Serialize;
use sqlx::PgPool;

pub mod handler;
pub mod render;
pub mod schema;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub hmac_key: [u8; 32],
}

pub fn slugify(title: &str) -> String {
    Regex::new(r"((<.*?>)|(&.*?;)|[^\w])+")
        .unwrap()
        .replace_all(title, "-")
        .trim_matches('-')
        .to_string()
        .to_lowercase()
}

pub async fn build_slug(
    folder_id: i32,
    title: &str,
    state: &AppState,
) -> Result<String, sqlx::Error> {
    let parent_slug = sqlx::query!("SELECT slug FROM folders WHERE id = $1", folder_id)
        .fetch_one(&state.db)
        .await?
        .slug;

    Ok(format!("{parent_slug}/{}", slugify(title)))
}

pub fn is_production() -> bool {
    env::var("PRODUCTION")
        .unwrap_or(String::from("false"))
        .parse()
        .unwrap()
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Slug {
    Post { slug: String },
    Folder { slug: String },
}
#[derive(Serialize)]
pub struct RevalidationRequest {
    pub slugs: Vec<Slug>,
}
impl RevalidationRequest {
    /// Revalidate a slug in NextJS static pages
    pub async fn execute(self) -> Result<(), reqwest::Error> {
        let frontend = env::var("FRONTEND").unwrap_or(String::from("http://localhost:3000"));
        reqwest::Client::new()
            .post(format!("{frontend}/api/revalidate"))
            .json(&self.slugs)
            .send()
            .await?;

        Ok(())
    }
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
