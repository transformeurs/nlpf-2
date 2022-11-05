use askama::Template;

use crate::users::models::AuthUser;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomePageTemplate {
    auth_user: Option<AuthUser>,
}

/// GET handler that simply return the home page.
pub async fn get_home_page(user: Option<AuthUser>) -> HomePageTemplate {
    // TODO: if connected return another template (offers)
    HomePageTemplate { auth_user: user }
}
