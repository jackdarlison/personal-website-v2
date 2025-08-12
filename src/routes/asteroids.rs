use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "pages/projects/asteroids.html")]
struct AsteroidsTemplate {}

pub(crate) async fn asteroids() -> impl IntoResponse {
    AsteroidsTemplate {}
}
