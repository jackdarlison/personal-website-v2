use std::sync::Arc;

use askama::Template;
use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse};
use sqlx::{prelude::FromRow, types::time::Date};

#[derive(Template, FromRow, Debug, Clone)]
#[template(path = "pages/post.html")]
pub(crate) struct PostTemplate {
    pub post_title: String,
    post_date: Date,
    post_body: String,
}


pub(crate) async fn post(Path(title): Path<String>, State(state): State<Arc<Vec<PostTemplate>>>) -> impl IntoResponse  {

    let post: Option<&PostTemplate> = state.iter().filter(|post| post.post_title == title).next();

    match post {
        None => (StatusCode::NOT_FOUND, "404 not found").into_response(),
        Some(p) => p.clone().into_response(),
    }
}