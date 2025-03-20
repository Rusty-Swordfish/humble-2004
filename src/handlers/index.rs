use crate::views::index::IndexTemplate;
use askama::Template;

pub fn handle() {
    let template = IndexTemplate { title: "Home" };
    println!("{}", template.render().unwrap());
}
