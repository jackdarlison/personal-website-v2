use std::env;

use askama::Template;
use axum::{routing::get, Router, response::IntoResponse};
use routes::{asteroids::asteroids, blog::blog, projects::projects, home::home, post::post, post_list::post_list};
use rustls_acme::{caches::DirCache, AcmeConfig};
use tokio_stream::StreamExt;
use tower_http::services::ServeDir;
use tracing::{error, info, Level};
use utils::blog::{generate_posts, get_posts};

use crate::routes::life::life;

mod routes;
mod utils;



#[tokio::main]
async fn main() -> anyhow::Result<()> {

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // fetch the posts from the database
    // if no posts, attempt to generate from the posts directory
    if get_posts().await?.is_empty() {
        generate_posts().await?;
    }

    // define the website
    let hx_router = Router::new()
        .route("/post_list", axum::routing::post(post_list));

    let projects_router = Router::new()
        .route("/", get(projects))
        .route("/asteroids", get(asteroids))
        .route("/life", get(life));

    let router = Router::new()
        .route("/", get(home))
        .route("/blog", get(blog))
        .route("/blog/:title", get(post))
        .nest("/projects", projects_router)
        .nest("/hx", hx_router)
        .nest_service("/static", ServeDir::new("static"))
        .fallback(routes::error404::handle_404);
    
    let http_addr: std::net::SocketAddr = "0.0.0.0:80".parse()?;

    let http_sever = axum_server::bind(http_addr).serve(router.clone().into_make_service());

    let http_only = match env::var("HTTP_ONLY") {
        Ok(s) => s.eq_ignore_ascii_case("true"),
        Err(_) => false,
    };


    if !http_only {
        // rustls_acme is used to automatically generate certificates for HTTPS
        // https://github.com/FlorianUekermann/rustls-acme/blob/main/examples/low_level_axum.rs
        let mut acme_state = AcmeConfig::new([env::var("URL")?])
            .contact_push(format!("mailto:{}", env::var("EMAIL")?))

            .cache(DirCache::new("./acme_cache"))
            .directory_lets_encrypt(true)
            .state();
        let acme_acceptor = acme_state.axum_acceptor(acme_state.default_rustls_config());

        tokio::spawn(async move {
            loop {
                match acme_state.next().await.unwrap() {
                    Ok(ok) => info!("ACME event: {:?}", ok),
                    Err(err) => error!("ACME error: {:?}", err),
                }
            }
        });

        let https_addr: std::net::SocketAddr = "0.0.0.0:443".parse()?;
        let https_server = axum_server::bind(https_addr).acceptor(acme_acceptor).serve(router.into_make_service());

        info!("Listening for HTTP and HTTPS");
        tokio::try_join!(http_sever, https_server)?;
    } else {
        info!("Listening for HTTP Only");
        http_sever.await?;
    }

    Ok(())
}
