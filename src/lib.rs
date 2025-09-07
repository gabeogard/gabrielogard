pub mod handlers;
pub mod models;
pub mod templates;
pub mod utils;

use models::{blog_post::BlogPost, resume::Resume};
use std::collections::HashMap;

pub struct AppState {
    pub resume: Resume,
    pub blog_posts: HashMap<String, BlogPost>,
}

impl AppState {
    pub async fn new() -> Self {
        let resume = Resume::load_from_file("resume.json").await;
        let blog_posts = BlogPost::load_all_from_dir("content/blog").await;

        Self { resume, blog_posts }
    }
}
