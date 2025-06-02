use askama::Template;

use crate::{handler::MiddlewareData, schema::*};

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomeTemplate {
    pub metadata: MiddlewareData,
}

#[derive(Template)]
#[template(path = "contact.html")]
pub struct ContactTemplate {
    pub metadata: MiddlewareData,
}

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {
    pub metadata: MiddlewareData,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub metadata: MiddlewareData,
}

#[derive(Template)]
#[template(path = "blog.html")]
pub struct BlogTemplate {
    pub metadata: MiddlewareData,
    pub featured_posts: Vec<Content>,
    pub categories: Vec<Folder>,
}

#[derive(Template)]
#[template(path = "search.html")]
pub struct SearchTemplate {
    pub metadata: MiddlewareData,
}

#[derive(Template)]
#[template(path = "folder.html")]
pub struct FolderTemplate {
    pub metadata: MiddlewareData,
    pub folder: FolderContents,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    pub metadata: MiddlewareData,
    pub post: PostFull,
}

#[derive(Template)]
#[template(path = "hidden_posts.html")]
pub struct HiddenPostsTemplate {
    pub metadata: MiddlewareData,
    pub posts: Vec<HiddenPost>,
}

#[derive(Template)]
#[template(path = "editor.html")]
pub struct EditorTemplate {}

#[derive(Template)]
#[template(path = "new_post.html")]
pub struct NewPostTemplate {
    pub metadata: MiddlewareData,
    pub parent: Option<i32>,
    pub existing_post: Option<PostFull>,
    pub folders: Vec<Folder>,
    pub all_tags: Vec<Tag>,
}

#[derive(Template)]
#[template(path = "new_link.html")]
pub struct NewLinkTemplate {
    pub metadata: MiddlewareData,
    pub parent: Option<i32>,
    pub existing_link: Option<Link>,
    pub folders: Vec<Folder>,
    pub all_tags: Vec<Tag>,
}

#[derive(Template)]
#[template(path = "new_folder.html")]
pub struct NewFolderTemplate {
    pub metadata: MiddlewareData,
    pub parent: Option<i32>,
    pub existing_folder: Option<Folder>,
    pub folders: Vec<Folder>,
}
