use axum::response::Html;
use crate::views::index::IndexTemplate;
use askama::Template;

pub async fn handle() -> Html<String> {
    let template = IndexTemplate { title: "Home" };
    Html(template.render().unwrap())
}
