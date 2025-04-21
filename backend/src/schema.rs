use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{handler::sign, AppState};

#[derive(Deserialize, JsonSchema)]
pub struct Login {
    pub password: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub img: String,
    pub href: String,
    pub category: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema, sqlx::Type)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub color: String,
}

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct PostFull {
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
    pub hidden: bool,
    pub autorelease: Option<DateTime<Utc>>,
    pub timestamp: DateTime<Utc>,
    pub tags: Vec<Tag>,
}
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct Link {
    pub id: i32,
    pub folder: i32,
    pub url: String,
    pub title: String,
    pub description: String,
    pub img: String,
    pub featured: bool,
    pub timestamp: DateTime<Utc>,
}
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema)]
pub enum Content {
    Folder(Folder),
    Post(Post),
    Link(Link),
}
impl Content {
    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            Content::Folder(folder) => folder.timestamp,
            Content::Post(post) => post.timestamp,
            Content::Link(link) => link.timestamp,
        }
    }
}
impl Ord for Content {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp().cmp(&other.timestamp())
    }
}
impl PartialOrd for Content {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct Post {
    pub id: i32,
    pub folder: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub img: String,
    pub points: i32,
    pub views: i32,
    pub featured: bool,
    pub hidden: bool,
    pub autorelease: Option<DateTime<Utc>>,
    pub timestamp: DateTime<Utc>,
    pub tags: Vec<Tag>,
}
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct HiddenPost {
    pub id: i32,
    pub folder: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub img: String,
    pub points: i32,
    pub views: i32,
    pub featured: bool,
    pub hidden: bool,
    pub autorelease: Option<DateTime<Utc>>,
    pub timestamp: DateTime<Utc>,
    pub tags: Vec<Tag>,
    pub signature: Option<String>,
}
impl HiddenPost {
    pub fn from_summary(post: Post, state: &AppState) -> Self {
        // let signature = sign(post.id, &state.hmac_key);
        let signature = match post.hidden {
            true => Some(sign(post.id, &state.hmac_key)),
            false => None,
        };
        Self {
            id: post.id,
            folder: post.folder,
            slug: post.slug,
            title: post.title,
            description: post.description,
            img: post.img,
            points: post.points,
            views: post.views,
            featured: post.featured,
            hidden: post.hidden,
            autorelease: post.autorelease,
            timestamp: post.timestamp,
            tags: post.tags,
            signature,
        }
    }
}
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct CreatePost {
    pub folder: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub img: String,
    pub points: i32,
    pub featured: bool,
    pub hidden: bool,
    pub autorelease: Option<DateTime<Utc>>,
    pub markdown: String,
    pub tags: Vec<Tag>, // Only ids are used
}
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct CreateLink {
    pub folder: i32,
    pub url: String,
    pub title: String,
    pub description: String,
    pub img: String,
    pub featured: bool,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct Folder {
    pub id: i32,
    pub parent: Option<i32>, // May be None for root folder
    pub slug: String,
    pub title: String,
    pub description: String,
    pub img: String,
    pub timestamp: DateTime<Utc>,
}
#[derive(Serialize, JsonSchema)]
pub struct FolderContents {
    pub id: i32,
    pub parent: Option<i32>,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub img: String,
    pub timestamp: DateTime<Utc>,
    pub contents: Vec<Content>,
}
impl FolderContents {
    pub async fn from_folder(folder: Folder, state: &AppState) -> Result<Self, sqlx::Error> {
        let contents_folders = sqlx::query_as!(
            Folder,
            "SELECT id, parent, slug, title, description, img, timestamp FROM folders WHERE parent = $1 ORDER BY timestamp DESC",
            folder.id
        )
        .fetch_all(&state.db)
        .await?
        .into_iter()
        .map(Content::Folder);

        let contents_posts = sqlx::query_as!(
            Post,
            r#"SELECT p.id, folder, slug, title, description, img, points, views, featured, hidden, autorelease, timestamp, 
                array(SELECT (t.id, t.name, t.color) FROM post_tags JOIN tags t ON t.id = tag_id WHERE post_id = p.id) as "tags!: Vec<Tag>"
                FROM posts p WHERE NOT hidden AND (folder = $1) ORDER BY timestamp DESC"#,
            folder.id
        )
        .fetch_all(&state.db)
        .await?
        .into_iter()
        .map(Content::Post);

        let contents_links = sqlx::query_as!(
            Link,
            "SELECT id, folder, url, title, description, img, featured, timestamp FROM links WHERE folder = $1 ORDER BY timestamp DESC",
            folder.id
        )
        .fetch_all(&state.db)
        .await?
        .into_iter()
        .map(Content::Link);

        let mut contents = contents_folders
            .chain(contents_posts)
            .chain(contents_links)
            .collect::<Vec<_>>();
        contents.sort(); // Sort by timestamp
        contents.reverse(); // Newest first

        Ok(Self {
            id: folder.id,
            parent: folder.parent,
            slug: folder.slug,
            title: folder.title,
            description: folder.description,
            img: folder.img,
            timestamp: folder.timestamp,
            contents,
        })
    }
}
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct CreateFolder {
    pub parent: Option<i32>,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub img: String,
}
