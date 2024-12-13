use std::env;

use axum::{routing::get, Router};
use routes::{blog::blog, home::home, post::post, post_list::post_list};
use rustls_acme::{caches::DirCache, AcmeConfig};
use tokio_stream::StreamExt;
use tower_http::services::ServeDir;
use tracing::{info, Level};
use tracing_appender::rolling;
use utils::blog::{generate_posts, get_posts};

mod routes;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    tracing_subscriber::fmt()
        .with_writer(rolling::daily("logs", "website.log"))
        .with_max_level(Level::DEBUG)
        .with_ansi(false)
        .init();

    // fetch the posts from the database
    // if no posts, attempt to generate from the posts directory
    if get_posts().await?.is_empty() {
        generate_posts().await?;
    }

    // define the website
    let api_router = Router::new();

    let router = Router::new()
        .route("/", get(home))
        .route("/blog", get(blog))
        .route("/post/:title", get(post))
        .route("/post_list", axum::routing::post(post_list))
        .nest("api/", api_router)
        .nest_service("/static", ServeDir::new("static"));
    
    let http_addr: std::net::SocketAddr = "0.0.0.0:80".parse()?;

    let http_sever = axum_server::bind(http_addr).serve(router.clone().into_make_service());

    let http_only = match env::var("HTTP_ONLY") {
        Ok(s) => s.eq_ignore_ascii_case("true"),
        Err(_) => false,
    };


    if !http_only {
        // rustls_acme is used to automatically generate certificates for HTTPS
        // https://github.com/FlorianUekermann/rustls-acme/blob/main/examples/low_level_axum.rs
        let mut acme_state = AcmeConfig::new(["jackdarlison.uk"])
            .contact_push(format!("mailto:{}", env::var("EMAIL")?))

            .cache(DirCache::new("./acme_cache"))
            .directory_lets_encrypt(true)
            .state();
        let acme_acceptor = acme_state.axum_acceptor(acme_state.default_rustls_config());

        tokio::spawn(async move {
            loop {
                match acme_state.next().await.unwrap() {
                    Ok(ok) => println!("ACME event: {:?}", ok),
                    Err(err) => eprintln!("ACME error: {:?}", err),
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
