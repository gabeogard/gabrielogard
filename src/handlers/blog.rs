use crate::{AppState, templates::renderer::render_template};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::Html,
};
use std::sync::Arc;

pub async fn index(State(state): State<Arc<AppState>>, headers: HeaderMap) -> Html<String> {
    let posts: Vec<_> = state.blog_posts.values().collect();
    let context = serde_json::json!({
        "posts": posts,
        "page_title": "Blog"
    });

    println!("Serving blog");

    if headers.get("hx-request").is_some() {
        Html(render_template("blog_content.html", &context))
    } else {
        Html(render_template("blog.html", &context))
    }
}

pub async fn post(
    Path(slug): Path<String>,
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Html<String> {
    if let Some(post) = state.blog_posts.get(&slug) {
        let context = serde_json::json!({
            "post": post,
            "page_title": &post.title
        });

        println!("Serving blog content");

        if headers.get("hx-request").is_some() {
            Html(render_template("blog_post_content.html", &context))
        } else {
            Html(render_template("blog_post.html", &context))
        }
    } else {
        Html("<h1>Post not found</h1>".to_string())
    }
}
