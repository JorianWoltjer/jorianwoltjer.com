use std::env;

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

pub async fn extend_slug(
    slug: &str,
    folder_id: i32,
    state: &AppState,
) -> Result<String, sqlx::Error> {
    match sqlx::query!("SELECT slug FROM folders WHERE id = $1", folder_id)
        .fetch_one(&state.db)
        .await
    {
        Ok(parent) => Ok(format!("{}/{slug}", parent.slug)),
        Err(sqlx::Error::RowNotFound) => Ok(slug.to_string()),
        Err(e) => Err(e),
    }
}

pub fn is_production() -> bool {
    env::var("PRODUCTION")
        .unwrap_or(String::from("false"))
        .parse()
        .unwrap()
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum Slug {
    Post { slug: String },
    Folder { slug: String },
}
#[derive(Serialize, Debug)]
pub struct RevalidationRequest {
    pub slugs: Vec<Slug>,
    pub views_only: bool,
}
impl RevalidationRequest {
    pub fn new(slugs: Vec<Slug>) -> Self {
        Self {
            slugs,
            views_only: false,
        }
    }

    /// Revalidate a slug in NextJS static pages
    pub async fn execute(self) -> Result<(), reqwest::Error> {
        if !self.slugs.is_empty() {
            let frontend = env::var("FRONTEND").unwrap_or(String::from("http://localhost:3000"));
            reqwest::Client::new()
                .post(format!("{frontend}/api/revalidate"))
                .header("X-Internal", env::var("INTERNAL_TOKEN").unwrap())
                .json(&self)
                .send()
                .await?;
        }

        Ok(())
    }
}
