use axum::{
    routing::{get},
    Router,
};

pub mod crud;
pub mod models;
pub mod routes;

/// Return a router for the offer module.
pub fn get_router() -> Router {
    Router::new()
        .route(
            "/offers",
            get(routes::get_offer),
        )
        .route(
            "/create_offer",
            get(routes::create_offer),
        )
        .route(
            "/view_offer",
            get(routes::view_offer),
        )
}