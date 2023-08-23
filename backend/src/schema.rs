use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Login {
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub img: String,
    pub markdown: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct PostSummary {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub img: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct CreatePost {
    pub folder: i32,
    pub title: String,
    pub description: String,
    pub img: String,
    pub markdown: String,
}

#[derive(Deserialize, Serialize)]
pub struct Folder {
    pub id: i32,
    pub parent: Option<i32>,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub img: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct FolderContents {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub img: String,
    pub timestamp: DateTime<Utc>,
    pub folders: Vec<Folder>,
    pub posts: Vec<PostSummary>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateFolder {
    pub parent: Option<i32>,
    pub title: String,
    pub description: String,
    pub img: String,
}
