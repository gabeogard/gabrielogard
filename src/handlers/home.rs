use crate::{AppState, templates::renderer::render_template};
use axum::{extract::State, http::HeaderMap, response::Html};
use std::sync::Arc;

pub async fn index(State(state): State<Arc<AppState>>, headers: HeaderMap) -> Html<String> {
    let context = serde_json::json!({
        "resume": state.resume,
        "page_title": "Home"
    });

    if headers.get("hx-request").is_some() {
        Html(render_template("home_content.html", &context))
    } else {
        Html(render_template("home.html", &context))
    }
}
