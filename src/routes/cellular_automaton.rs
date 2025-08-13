use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "pages/projects/cellular_automaton.html")]
struct CellularAutomatonTemplate {}

pub(crate) async fn cellular_automaton() -> impl IntoResponse {
    CellularAutomatonTemplate {}
}