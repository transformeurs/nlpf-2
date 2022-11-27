use axum::{routing::get, Router};

pub mod crud;
pub mod models;
pub mod routes;

/// Return a router for the user module.
pub fn get_router() -> Router {
    Router::new()
        .route("/candidacies", get(routes::get_candidacy))
        .route(
            "/create_candidacy",
            get(routes::get_create_candidacy).post(routes::post_create_candidacy),
        )
        .route(
            "/candidacies_by_candidate/",
            get(routes::get_candidacy_candidate),
        )
        .route("/view_candidacy/:uuid", get(routes::get_view_candidacy))
}
