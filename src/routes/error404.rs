use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "pages/error404.html")]
pub(crate) struct Error404 {}

pub(crate) async fn handle_404() -> impl IntoResponse {
    Error404 {}
}