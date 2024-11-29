use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::types::time::Date;

use crate::utils::post::Post;

#[derive(Template)]
#[template(path = "pages/post.html")]
pub struct PostTemplate {
    pub post_title: String,
    post_date: Date,
    post_body: String,
}

impl Into<PostTemplate> for Post {
    fn into(self) -> PostTemplate {
        return PostTemplate {
            post_title: self.title,
            post_date: self.date,
            post_body: self.body,
        };
    }
}

pub(crate) async fn post(
    Path(title): Path<String>,
    State(state): State<Arc<Vec<Post>>>,
) -> impl IntoResponse {
    let post: Option<&Post> = state.iter().filter(|post| post.title == title).next();

    match post {
        None => (StatusCode::NOT_FOUND, "404 not found").into_response(),
        Some(p) => {
            let post_template: PostTemplate = p.clone().into();
            post_template.into_response()
        }
    }
}
