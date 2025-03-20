mod handlers;
mod views;

fn main() {
    let path = "/"; // In a real app, you'd get this from the request

    match path {
        "/" => handlers::index::handle(),
        "/about" => handlers::about::handle(),
        _ => println!("404 Not Found"),
    }
}
