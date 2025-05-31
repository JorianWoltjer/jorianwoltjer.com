use askama::Template;

use crate::schema::*;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomeTemplate {
    pub nonce: String,
}

#[derive(Template)]
#[template(path = "projects.html")]
pub struct ProjectsTemplate {
    pub nonce: String,
    pub projects: Vec<Project>,
}

#[derive(Template)]
#[template(path = "contact.html")]
pub struct ContactTemplate {
    pub nonce: String,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub nonce: String,
}

#[derive(Template)]
#[template(path = "blog.html")]
pub struct BlogTemplate {
    pub nonce: String,
    pub featured_posts: Vec<Content>,
    pub categories: Vec<Folder>,
}

#[derive(Template)]
#[template(path = "search.html")]
pub struct SearchTemplate {
    pub nonce: String,
}

#[derive(Template)]
#[template(path = "folder.html")]
pub struct FolderTemplate {
    pub nonce: String,
    pub folder: FolderContents,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    pub nonce: String,
    pub post: PostFull,
}

#[derive(Template)]
#[template(path = "hidden_posts.html")]
pub struct HiddenPostsTemplate {
    pub nonce: String,
    pub posts: Vec<HiddenPost>,
}

#[derive(Template)]
#[template(path = "new_post.html")]
pub struct NewPostTemplate {
    pub nonce: String,
    pub existing_post: Option<PostFull>,
    pub folders: Vec<Folder>,
    pub tags: Vec<Tag>,
}

#[derive(Template)]
#[template(path = "new_link.html")]
pub struct NewLinkTemplate {
    pub nonce: String,
    pub existing_link: Option<Link>,
    pub folders: Vec<Folder>,
    pub tags: Vec<Tag>,
}

#[derive(Template)]
#[template(path = "new_folder.html")]
pub struct NewFolderTemplate {
    pub nonce: String,
    pub existing_folder: Option<Folder>,
    pub folders: Vec<Folder>,
}

#[derive(Template)]
#[template(path = "preview_post.html")]
pub struct PreviewPostTemplate {
    pub nonce: String,
    pub post: PostFull,
}
