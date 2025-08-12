use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "pages/projects/life.html")]
struct LifeTemplate {}

pub(crate) async fn life() -> impl IntoResponse {
    LifeTemplate {}
}