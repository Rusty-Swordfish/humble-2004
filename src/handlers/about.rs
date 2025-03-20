use crate::views::about::AboutTemplate;
use askama::Template;

pub fn handle() {
    let template = AboutTemplate { title: "About Us" };
    println!("{}", template.render().unwrap());
}
