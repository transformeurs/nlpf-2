use axum::{
    routing::{delete, get, post},
    Router,
};

pub mod crud;
pub mod models;
pub mod routes;

/// Return a router for the questionnaires module.
pub fn get_router() -> Router {
    Router::new()
        .route("/", get(routes::get_questionnaires_page))
        .route("/", post(routes::post_questionnaire_page))
        .route("/:id", delete(routes::delete_questionnaire_page))
        .route("/create", get(routes::get_create_questionnaire_page))
}
