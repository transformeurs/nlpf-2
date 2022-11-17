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
        .route("/offers", get(routes::get_offer))
        .route("/offers/:company_email", get(routes::get_offer_company))
        .route(
            "/create_offer",
            get(routes::get_create_offer).post(routes::post_create_offer),
        )
    //.route("/view_offer/:uuid", get(routes::get_view_offer))
}
