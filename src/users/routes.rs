use askama::Template;

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignupTemplate {}

pub async fn get_signup_page() -> SignupTemplate {
    SignupTemplate {}
}
