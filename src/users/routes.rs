use std::collections::HashMap;

use askama::Template;
use async_session::{MemoryStore, Session, SessionStore};
use axum::{
    async_trait,
    extract::{
        rejection::TypedHeaderRejectionReason, ContentLengthLimit, FromRequest, Multipart,
        RequestParts,
    },
    headers,
    http::{header, header::SET_COOKIE, HeaderMap, StatusCode},
    response::{IntoResponse, Redirect},
    Extension, Form, TypedHeader,
};
use serde::{Deserialize, Serialize};

use super::crud::get_candidate_by_email;
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

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    user_role: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    user_role: String,
    email: String,
}

static COOKIE_NAME: &str = "SESSION";

/// POST handler for login
pub async fn post_login_page(
    Form(input): Form<LoginForm>,
    Extension(state): Extension<SharedState>,
    Extension(store): Extension<MemoryStore>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if input.user_role == "candidate" {
        if let Some(candidate) =
            get_candidate_by_email(input.email, state)
                .await
                .map_err(|err| {
                    tracing::error!("Error getting candidate: {:?}", err);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Error getting candidate".to_string(),
                    )
                })?
        {
            if bcrypt::verify(input.password, &candidate.password)
                .map_err(|_err| (StatusCode::INTERNAL_SERVER_ERROR, "Error".to_string()))?
            {
                // User successfully authenticated
                // TODO create a session
                let mut session = Session::new();
                session
                    .insert(
                        "user",
                        AuthUser {
                            user_role: input.user_role,
                            email: candidate.email,
                        },
                    )
                    .unwrap();

                let cookie = store.store_session(session).await.unwrap().unwrap();

                let cookie = format!("{}={}; Secure; HttpOnly", COOKIE_NAME, cookie);

                // Set cookie
                let mut headers = HeaderMap::new();
                headers.insert(SET_COOKIE, cookie.parse().unwrap());

                return Ok((headers, Redirect::to("/infos")));
            }
        }
    } else if input.user_role == "company" {
        return Err((StatusCode::NOT_IMPLEMENTED, "Invalid role.".to_string()));
    } else {
        return Err((StatusCode::BAD_REQUEST, "Invalid role.".to_string()));
    }

    Err((StatusCode::UNAUTHORIZED, "Invalid credentials.".to_string()))
}

#[async_trait]
impl<B> FromRequest<B> for AuthUser
where
    B: Send,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(store) = Extension::<MemoryStore>::from_request(req)
            .await
            .expect("`MemoryStore` extension is missing");

        let cookies = TypedHeader::<headers::Cookie>::from_request(req)
            .await
            .map_err(|e| match *e.name() {
                header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => StatusCode::UNAUTHORIZED,
                    _ => panic!("unexpected error getting Cookie header(s): {}", e),
                },
                _ => panic!("unexpected error getting cookies: {}", e),
            })?;
        let session_cookie = cookies.get(COOKIE_NAME).ok_or(StatusCode::UNAUTHORIZED)?;

        let session = store
            .load_session(session_cookie.to_string())
            .await
            .unwrap()
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let user = session
            .get::<AuthUser>("user")
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(user)
    }
}

#[derive(Template)]
#[template(path = "candidate_infos.html")]
pub struct InfosTemplate {
    candidate: Candidate,
}

/// GET handler for showing infos
pub async fn get_infos_page(
    Extension(state): Extension<SharedState>,
    user: AuthUser,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if user.user_role == "candidate" {
        let candidate = get_candidate_by_email(user.email, state)
            .await
            .map_err(|err| {
                tracing::error!("Error getting candidate: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error getting candidate".to_string(),
                )
            })?
            .ok_or((StatusCode::NOT_FOUND, "Candidate not found".to_string()))?;

        Ok(InfosTemplate { candidate })
    } else if user.user_role == "company" {
        return Err((StatusCode::NOT_IMPLEMENTED, "Invalid role.".to_string()));
    } else {
        return Err((StatusCode::BAD_REQUEST, "Invalid role.".to_string()));
    }
}
