use axum::Router;

mod index;
mod about;

pub fn create_router() -> Router {
    Router::new()
        .merge(index::routes())
        .merge(about::routes())
}
