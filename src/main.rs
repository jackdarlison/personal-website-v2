use std::{env, sync::Arc};

use anyhow::Ok;
use axum::{routing::get, Router};
use dotenvy::dotenv;
use routes::{blog::blog, home::home, post::{post, PostTemplate}};
use sqlx::postgres::PgPoolOptions;
use tower_http::services::ServeDir;

mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    dotenv().expect("Cannot read .env file");

    let db_url = env::var("DB_URL").expect("DB_URL environment variable not found!");

    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(&db_url)
        .await?;

    let posts = sqlx::query_as::<_, PostTemplate>("SELECT post_title, post_date, post_body FROM myposts")
        .fetch_all(&pool)
        .await?;

    println!("Found posts: {:?}", posts.iter().map(|p| p.post_title.as_str()).collect::<Vec<&str>>());

    let state = Arc::new(posts);

    let api_router = Router::new();

    let router = Router::new()
        .route("/", get(home))
        .route("/blog", get(blog))
        .route("/post/:title", get(post))
        .nest("api/", api_router)
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    println!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, router).await?;

    Ok(())
}
