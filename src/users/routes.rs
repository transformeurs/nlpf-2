use std::collections::HashMap;

use askama::Template;
use axum::{
    extract::{ContentLengthLimit, Multipart},
    http::StatusCode,
    Extension,
};

use crate::{
    users::{crud::create_candidate, models::Candidate},
    SharedState,
};

#[derive(Template)]
#[template(path = "signup/index.html")]
pub struct SignupTemplate {}

/// GET handler that simply return the signup form page.
pub async fn get_signup_page() -> SignupTemplate {
    SignupTemplate {}
}

#[derive(Template)]
#[template(path = "signup/success.html")]
pub struct SignupSuccessTemplate {}

/// POST handler for signup form submission
/// First, we extract the multipart form data from the request and create a
/// hash for each field. For the file input, upload the file to S3 and get the
/// URL. Then, create a new candidate/company in the database.
pub async fn post_signup_page(
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            10 * 1024 * 1024 /* 10 MB */
        },
    >,
    Extension(state): Extension<SharedState>,
) -> Result<SignupSuccessTemplate, (StatusCode, String)> {
    let mut form_fields = HashMap::new();

    // Fill the hashmap wigth the form fields
    while let Some(field) = multipart
        .next_field()
        .await
        // Return HTTP 400 Bad Request in case of any error
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
    {
        if let Some(name) = field.name() {
            // Handling the file upload to generate an URL from S3
            if name == "fileinput" {
                // TODO
                let name = String::from("photo_url");
                let uri = String::from("https://www.google.com");
                form_fields.insert(name, uri);
            }
            // Other fields
            else {
                let name = name.to_string();
                let content = field.text().await.unwrap().clone();
                form_fields.insert(name, content);
            }
        }
    }

    // Create a new candidate or a new company by reading the userRole field
    if let Some(role) = form_fields.get("userRole") {
        if role == "candidate" {
            let candidate = Candidate::from_hash_map(form_fields);
            create_candidate(candidate, state).await.map_err(|err| {
                tracing::error!("Error creating candidate: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error creating candidate".to_string(),
                )
            })?;
        } else if role == "company" {
            // TODO
            return Err((
                StatusCode::NOT_IMPLEMENTED,
                "Company signup not implemented".to_string(),
            ));
        } else {
            return Err((StatusCode::BAD_REQUEST, "Invalid role.".to_string()));
        }
    }
    // If the "userRole" field was not in the form...
    else {
        tracing::error!("No role found");
        return Err((StatusCode::BAD_REQUEST, "No role found".to_string()));
    }

    // Return success page!
    Ok(SignupSuccessTemplate {})
}
