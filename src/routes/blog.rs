use std::sync::Arc;

use askama::Template;
use axum::{extract::State, response::IntoResponse};

use crate::utils::post::Post;

#[derive(Template)]
#[template(path = "pages/blog.html")]
struct BlogTemplate {}

pub(crate) async fn blog() -> impl IntoResponse {
    BlogTemplate {}
}
