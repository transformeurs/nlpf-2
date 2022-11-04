use axum::{
    routing::{get, post},
    Router,
};

pub mod crud;
pub mod models;
pub mod routes;

/// Return a router for the user module.
pub fn get_router() -> Router {
    Router::new()
        .route(
            "/signup",
            get(routes::get_signup_page).post(routes::post_signup_page),
        )
        .route("/login", post(routes::post_login_page))
        .route("/infos", get(routes::get_infos_page))
}
