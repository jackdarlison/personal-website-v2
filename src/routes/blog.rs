
use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "pages/blog.html")]
struct BlogTemplate {}

pub(crate) async fn blog() -> impl IntoResponse {
    BlogTemplate {}
}
