use axum::{Router, routing::get};
use crate::handlers;

pub fn routes() -> Router {
    Router::new().route("/about", get(handlers::about::handle))
}
