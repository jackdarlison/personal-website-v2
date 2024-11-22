use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "pages/home.html")]
struct HomeTemplate {}

pub(crate) async fn home() -> impl IntoResponse {
    HomeTemplate {}
}
