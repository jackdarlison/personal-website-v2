
use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "pages/games.html")]
struct GamesTemplate {}

pub(crate) async fn games() -> impl IntoResponse {
    GamesTemplate {}
}
