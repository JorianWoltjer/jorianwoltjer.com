use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Deserialize)]
pub struct Login {
    pub password: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Tag {
    pub name: String,
    pub color: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Post {
    pub id: i32,
    pub folder: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub img: String,
    pub markdown: String,
    pub points: i32,
    pub views: i32,
    pub featured: bool,
    pub timestamp: DateTime<Utc>,
    pub tags: Vec<Tag>,
}
#[derive(Deserialize, Serialize)]
pub struct PostSummary {
    pub id: i32,
    pub folder: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub img: String,
    pub points: i32,
    pub views: i32,
    pub featured: bool,
    pub timestamp: DateTime<Utc>,
    pub tags: Vec<Tag>,
}
#[derive(Deserialize, Serialize)]
pub struct CreatePost {
    pub folder: i32,
    pub title: String,
    pub description: String,
    pub img: String,
    pub points: i32,
    pub featured: bool,
    pub markdown: String,
    pub tags: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
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
    pub parent: Option<i32>,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub img: String,
    pub timestamp: DateTime<Utc>,
    pub folders: Vec<Folder>,
    pub posts: Vec<PostSummary>,
}
impl FolderContents {
    pub async fn from_folder(folder: Folder, state: &AppState) -> Result<Self, sqlx::Error> {
        let contents_folders = sqlx::query_as!(
            Folder,
            "SELECT id, parent, slug, title, description, img, timestamp FROM folders WHERE parent = ?",
            folder.id
        )
        .fetch_all(&state.db)
        .await?;

        let contents_posts = sqlx::query_as!(
            PostSummary,
            "SELECT id, folder, slug, title, description, img, points, views, featured as `featured: bool`, timestamp FROM posts WHERE folder = ?",
            folder.id
        )
        .fetch_all(&state.db)
        .await?;

        Ok(Self {
            id: folder.id,
            parent: folder.parent,
            slug: folder.slug,
            title: folder.title,
            description: folder.description,
            img: folder.img,
            timestamp: folder.timestamp,
            folders: contents_folders,
            posts: contents_posts,
        })
    }
}
#[derive(Deserialize, Serialize)]
pub struct CreateFolder {
    pub parent: Option<i32>,
    pub title: String,
    pub description: String,
    pub img: String,
}
