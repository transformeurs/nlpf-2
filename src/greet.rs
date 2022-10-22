use askama::Template;
use axum::extract;

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate {
    name: String,
}

pub async fn greet_template(extract::Path(name): extract::Path<String>) -> HelloTemplate {
    HelloTemplate { name }
}
