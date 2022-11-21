use askama::Template;
use axum::response::{IntoResponse, Redirect, Response};

use crate::users::models::AuthUser;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomePageTemplate {
    auth_user: Option<AuthUser>,
}

/// GET handler that simply return the home page.
pub async fn get_home_page(user: Option<AuthUser>) -> Response {
    if user.is_some() {
        return Redirect::to("/offers").into_response();
    }

    HomePageTemplate { auth_user: None }.into_response()
}
