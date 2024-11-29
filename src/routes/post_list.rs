use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Form, State},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::utils::post::Post;

#[derive(Template)]
#[template(path = "components/post_list.html")]
pub struct PostListTemplate {
    posts: Vec<Post>,
}

#[derive(Deserialize, Debug)]
pub struct PostListForm {
    contains: String,
}

pub async fn post_list(
    State(state): State<Arc<Vec<Post>>>,
    Form(search): Form<PostListForm>,
) -> impl IntoResponse {
    let posts: Vec<Post> = state
        .iter()
        .filter(|p| p.title.contains(&search.contains))
        .cloned()
        .collect();

    PostListTemplate { posts }.into_response()
}
