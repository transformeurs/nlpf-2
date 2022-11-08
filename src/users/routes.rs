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
    response::{IntoResponse, Redirect, Response},
    Extension, Form, TypedHeader,
};
use serde::Deserialize;

use super::{
    crud::{create_company, get_candidate_by_email, get_company_by_email},
    models::{AuthUser, Company},
};
use crate::{
    users::{crud::create_candidate, models::Candidate},
    utils::s3::upload_bytes_to_s3,
    SharedState,
};

#[derive(Template)]
#[template(path = "signup/index.html")]
pub struct SignupTemplate {
    auth_user: Option<AuthUser>,
}

/// GET handler that simply return the signup form page.
pub async fn get_signup_page() -> SignupTemplate {
    SignupTemplate { auth_user: None }
}

#[derive(Template)]
#[template(path = "signup/success.html")]
pub struct SignupSuccessTemplate {
    auth_user: Option<AuthUser>,
}

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
                let content_type = field.content_type().unwrap().to_string();
                let key = field.file_name().unwrap().to_string();
                let bytes = field.bytes().await.unwrap();
                let uri = upload_bytes_to_s3(
                    bytes,
                    content_type,
                    "profile-pictures".to_string(),
                    key,
                    state.clone(),
                )
                .await
                .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

                let name = String::from("photo_url");
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
            let company = Company::from_hash_map(form_fields);
            create_company(company, state).await.map_err(|err| {
                tracing::error!("Error creating company: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error creating company".to_string(),
                )
            })?;
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
    Ok(SignupSuccessTemplate { auth_user: None })
}

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    user_role: String,
    email: String,
    password: String,
}

static COOKIE_NAME: &str = "SESSION";

async fn validate_login(
    email: String,
    password: String,
    hashed_password: String,
    user_role: String,
    store: MemoryStore,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if bcrypt::verify(password, &hashed_password)
        .map_err(|_err| (StatusCode::INTERNAL_SERVER_ERROR, "Error".to_string()))?
    {
        // User successfully authenticated
        let mut session = Session::new();
        session
            .insert("user", AuthUser { user_role, email })
            .unwrap();

        let cookie = store.store_session(session).await.unwrap().unwrap();

        let cookie = format!("{}={}; Secure; HttpOnly", COOKIE_NAME, cookie);

        // Set cookie
        let mut headers = HeaderMap::new();
        headers.insert(SET_COOKIE, cookie.parse().unwrap());

        return Ok((headers, Redirect::to("/infos")));
    } else {
        // User not authenticated
        return Err((
            StatusCode::UNAUTHORIZED,
            "Invalid email or password".to_string(),
        ));
    }
}

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
            return validate_login(
                candidate.email,
                input.password,
                candidate.password,
                input.user_role,
                store,
            )
            .await;
        }
    } else if input.user_role == "company" {
        if let Some(company) = get_company_by_email(input.email, state)
            .await
            .map_err(|err| {
                tracing::error!("Error getting candidate: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error getting company".to_string(),
                )
            })?
        {
            return validate_login(
                company.email,
                input.password,
                company.password,
                input.user_role,
                store,
            )
            .await;
        }
    } else {
        return Err((StatusCode::BAD_REQUEST, "Invalid role.".to_string()));
    }

    Err((
        StatusCode::UNAUTHORIZED,
        "Invalid email or password".to_string(),
    ))
}

/// GET handler for logout (not POST because too difficult in HTML)
pub async fn get_logout_page(
    Extension(store): Extension<MemoryStore>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let cookie = cookies
        .get(COOKIE_NAME)
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "No cookie found".to_string()))?;

    let session = match store.load_session(cookie.to_string()).await.unwrap() {
        Some(s) => s,
        // No session active, just redirect
        None => return Ok(Redirect::to("/")),
    };

    store.destroy_session(session).await.map_err(|err| {
        tracing::error!("Error clearing session: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error clearing session".to_string(),
        )
    })?;

    Ok(Redirect::to("/"))
}

pub struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::to("/").into_response()
    }
}

#[async_trait]
impl<B> FromRequest<B> for AuthUser
where
    B: Send,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = AuthRedirect;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(store) = Extension::<MemoryStore>::from_request(req)
            .await
            .expect("`MemoryStore` extension is missing");

        let cookies = TypedHeader::<headers::Cookie>::from_request(req)
            .await
            .map_err(|e| match *e.name() {
                header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => AuthRedirect,
                    _ => panic!("unexpected error getting Cookie header(s): {}", e),
                },
                _ => panic!("unexpected error getting cookies: {}", e),
            })?;
        let session_cookie = cookies.get(COOKIE_NAME).ok_or(AuthRedirect)?;

        let session = store
            .load_session(session_cookie.to_string())
            .await
            .unwrap()
            .ok_or(AuthRedirect)?;

        let user = session.get::<AuthUser>("user").ok_or(AuthRedirect)?;

        Ok(user)
    }
}

#[derive(Template)]
#[template(path = "users_infos.html")]
pub struct InfosTemplate {
    auth_user: Option<AuthUser>,
}

/// GET handler for showing infos
pub async fn get_infos_page(user: AuthUser) -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok(InfosTemplate {
        auth_user: Some(user),
    })
}
