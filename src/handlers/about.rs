use axum::response::Html;
use crate::views::about::AboutTemplate;
use askama::Template;

pub async fn handle() -> Html<String> {
    let template = AboutTemplate { title: "About Us" };
    Html(template.render().unwrap())
}
