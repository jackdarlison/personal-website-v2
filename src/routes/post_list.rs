
use askama::Template;
use axum::{
    extract::Form, http::StatusCode, response::IntoResponse
};
use serde::Deserialize;

use crate::utils::{blog::get_posts, post::Post};

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
    Form(search): Form<PostListForm>,
) -> impl IntoResponse {

    let db_response = get_posts().await;

    let Ok(posts) = db_response else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Cannot find posts").into_response();
    };

    PostListTemplate {
        posts: posts.into_iter()
            .filter(|p| p.title.contains(&search.contains))
            .collect()
    }.into_response()
}
