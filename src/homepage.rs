use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomePageTemplate {}

/// GET handler that simply return the home page.
pub async fn get_home_page() -> HomePageTemplate {
    HomePageTemplate {}
}
