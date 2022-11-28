use askama::Template;
use axum::{
    extract::Query,
    response::{IntoResponse, Redirect, Response},
};
use serde::Deserialize;

use crate::users::models::AuthUser;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomePageTemplate {
    auth_user: Option<AuthUser>,
    signin_error: bool,
}

#[derive(Deserialize)]
pub struct HomePageQueryParams {
    error: Option<bool>,
}

/// GET handler that simply return the home page.
pub async fn get_home_page(user: Option<AuthUser>, params: Query<HomePageQueryParams>) -> Response {
    if user.is_some() {
        return Redirect::to("/offers").into_response();
    }

    HomePageTemplate {
        auth_user: None,
        signin_error: params.error.unwrap_or(false),
    }
    .into_response()
}
