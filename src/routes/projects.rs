
use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "pages/projects.html")]
struct ProjectsTemplate {}

pub(crate) async fn projects() -> impl IntoResponse {
    ProjectsTemplate {}
}
