use std::{env, sync::Arc};

use anyhow::Ok;
use axum::{routing::get, Router};
use routes::{blog::blog, home::home, post::post, post_list::post_list};
use tower_http::services::ServeDir;
use utils::blog::{generate_posts, get_posts};

mod routes;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // if we have no posts, generate from the posts directory
    if get_posts().await?.is_empty() {
        generate_posts().await?;
    }

    let api_router = Router::new();

    let router = Router::new()
        .route("/", get(home))
        .route("/blog", get(blog))
        .route("/post/:title", get(post))
        .route("/post_list", axum::routing::post(post_list))
        .nest("api/", api_router)
        .nest_service("/static", ServeDir::new("static"));

    let port = env::var("WEBSITE_PORT")?;

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    println!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, router).await?;

    Ok(())
}
