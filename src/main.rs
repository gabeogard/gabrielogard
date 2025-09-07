use axum::{
    routing::{Router, get},
    serve,
};
use rust_portfolio::{AppState, handlers};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::new().await);

    let app = Router::new()
        .route("/", get(handlers::home::index))
        .route("/blog", get(handlers::blog::index))
        .route("/blog/:slug", get(handlers::blog::post))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:42069").await.unwrap();
    println!("Server running on http://localhost:42069");
    serve(listener, app).await.unwrap();
}
