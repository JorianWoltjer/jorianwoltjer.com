use askama::Template;
use axum::http;
use chrono::{DateTime, Utc};

use crate::{handler::MiddlewareData, schema::*};

pub struct Metadata {
    pub url: http::Uri,
    pub title: String,
    pub description: Option<String>,
    pub image: Option<String>,
}
impl Metadata {
    pub fn only_title(url: http::Uri, title: &str) -> Self {
        Self {
            url,
            title: title.to_string(),
            description: None,
            image: None,
        }
    }
}
pub struct SitemapUrl {
    pub path: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomeTemplate {
    pub middleware: MiddlewareData,
    pub metadata: Metadata,
}

#[derive(Template)]
#[template(path = "contact.html")]
pub struct ContactTemplate {
    pub middleware: MiddlewareData,
    pub metadata: Metadata,
}

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {
    pub middleware: MiddlewareData,
    pub metadata: Metadata,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub middleware: MiddlewareData,
    pub metadata: Metadata,
}

#[derive(Template)]
#[template(path = "blog.html")]
pub struct BlogTemplate {
    pub middleware: MiddlewareData,
    pub metadata: Metadata,
    pub featured_posts: Vec<Content>,
    pub categories: Vec<Folder>,
}

#[derive(Template)]
#[template(path = "search.html")]
pub struct SearchTemplate {
    pub middleware: MiddlewareData,
    pub metadata: Metadata,
}

#[derive(Template)]
#[template(path = "folder.html")]
pub struct FolderTemplate {
    pub middleware: MiddlewareData,
    pub metadata: Metadata,
    pub folder: FolderContents,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    pub middleware: MiddlewareData,
    pub metadata: Metadata,
    pub post: PostFull,
}

#[derive(Template)]
#[template(path = "hidden_posts.html")]
pub struct HiddenPostsTemplate {
    pub middleware: MiddlewareData,
    pub metadata: Metadata,
    pub posts: Vec<HiddenPost>,
}

#[derive(Template)]
#[template(path = "editor.html")]
pub struct EditorTemplate {}

#[derive(Template)]
#[template(path = "new_post.html")]
pub struct NewPostTemplate {
    pub middleware: MiddlewareData,
    pub metadata: Metadata,
    pub parent: Option<i32>,
    pub existing_post: Option<PostFull>,
    pub folders: Vec<Folder>,
    pub all_tags: Vec<Tag>,
}

#[derive(Template)]
#[template(path = "new_link.html")]
pub struct NewLinkTemplate {
    pub middleware: MiddlewareData,
    pub metadata: Metadata,
    pub parent: Option<i32>,
    pub existing_link: Option<Link>,
    pub folders: Vec<Folder>,
    pub all_tags: Vec<Tag>,
}

#[derive(Template)]
#[template(path = "new_folder.html")]
pub struct NewFolderTemplate {
    pub middleware: MiddlewareData,
    pub metadata: Metadata,
    pub parent: Option<i32>,
    pub existing_folder: Option<Folder>,
    pub folders: Vec<Folder>,
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct Error404Template {
    pub middleware: MiddlewareData,
    pub metadata: Metadata,
    pub url: String,
}

#[derive(Template)]
#[template(path = "rss.xml")]
pub struct RssTemplate {
    pub latest_posts: Vec<ContentFull>,
}

#[derive(Template)]
#[template(path = "sitemap.xml")]
pub struct SitemapTemplate {
    pub urls: Vec<SitemapUrl>,
}
