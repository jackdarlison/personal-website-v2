use anyhow::Ok;
use axum::{routing::get, Router};
use routes::home::home;
use tower_http::services::ServeDir;

mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let api_router = Router::new();

    let router = Router::new()
        .route("/", get(home))
        .nest("api/", api_router)
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    println!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, router).await?;

    Ok(())
}
