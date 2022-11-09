use axum::{routing::get, Router};

// pub mod crud;
// pub mod models;
pub mod routes;

/// Return a router for the questionnaires module.
pub fn get_router() -> Router {
    Router::new()
        .route("/", get(routes::get_questionnaires_page))
        .route("/create", get(routes::get_create_questionnaire_page))
}
