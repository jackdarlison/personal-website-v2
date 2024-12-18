
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
};

use crate::utils::{blog::get_posts, post::Post};

#[derive(Template)]
#[template(path = "pages/post.html")]
pub struct PostTemplate {
    post: Post
}

pub(crate) async fn post(
    Path(title): Path<String>,
) -> impl IntoResponse {

    let db_response = get_posts().await;

    let Ok(posts) = db_response else { 
        return (StatusCode::INTERNAL_SERVER_ERROR, "Cannot find posts").into_response()
    };

    match posts.into_iter().filter(|post| post.title == title).next() {
        None => (StatusCode::NOT_FOUND, "404 not found").into_response(),
        Some(post) => {
            PostTemplate { post }.into_response()
        }
    }
}
