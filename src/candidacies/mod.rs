use axum::{routing::get, Router};

pub mod crud;
pub mod models;
pub mod routes;

/// Return a router for the user module.
pub fn get_router() -> Router {
    Router::new()
        .route("/candidacies", get(routes::get_candidacies))
        .route(
            "/create_candidacy/:uuid",
            get(routes::get_create_candidacy).post(routes::post_create_candidacy),
        )
        .route("/view_candidacy/:uuid", get(routes::get_view_candidacy))
        .route(
            "/candidacies_by_offer/:uuid",
            get(routes::get_view_candidacy_by_offer),
        )
        .route(
            "/post_accept_candidacy/:uuid",
            get(routes::post_accept_candidacy),
        )
        .route(
            "/post_refuse_candidacy/:uuid",
            get(routes::post_refuse_candidacy),
        )
}
