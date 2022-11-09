use axum::{routing::get, Router};

pub mod crud;
pub mod models;
pub mod routes;

/// Return a router for the user module.
pub fn get_router() -> Router {
    Router::new().route("/candidacies", get(routes::plain_text))
}
