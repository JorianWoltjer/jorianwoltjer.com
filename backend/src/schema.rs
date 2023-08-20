use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreatePost {
    pub title: String,
    pub body: String,
}
