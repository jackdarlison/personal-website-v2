use std::sync::Arc;

use anyhow::Ok;
use axum::{routing::get, Router};
use routes::{blog::blog, home::home, post::post, post_list::post_list};
use tower_http::services::ServeDir;
use utils::blog::get_posts;

mod routes;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let posts = get_posts().await?;

    println!(
        "Found posts: {:?}",
        posts
            .iter()
            .map(|p| p.post_title.as_str())
            .collect::<Vec<&str>>()
    );

    let state = Arc::new(posts);

    let api_router = Router::new();

    let router = Router::new()
        .route("/", get(home))
        .route("/blog", get(blog))
        .route("/post/:title", get(post))
        .route("/post_list", axum::routing::post(post_list))
        .nest("api/", api_router)
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    println!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, router).await?;

    Ok(())
}
