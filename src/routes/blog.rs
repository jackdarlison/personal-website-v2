use std::sync::Arc;

use askama::Template;
use axum::{extract::State, response::IntoResponse};

use super::post::PostTemplate;


#[derive(Template)]
#[template(path = "pages/blog.html")]
struct BlogTemplate {
    titles: Vec<String>
}

pub(crate) async fn blog(State(state): State<Arc<Vec<PostTemplate>>>) -> impl IntoResponse  {

    let titles: Vec<String> = state.iter().map(|p| p.post_title.clone()).collect();

    BlogTemplate {
        titles
    }
}