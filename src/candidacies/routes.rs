use std::collections::HashMap;

use askama::Template;

use axum::{
    extract::{Multipart, Path},
    http::StatusCode,
    Extension,
};
use uuid::Uuid;

use super::{crud::create_candidacy, models::Candidacy};
use crate::{users::models::AuthUser, utils::s3::upload_bytes_to_s3, SharedState};

#[derive(Template)]
#[template(path = "candidacies/create_candidacy.html")]
pub struct CreateCandidacyTemplate {
    auth_user: Option<AuthUser>,
}

pub async fn get_create_candidacy(
    user: AuthUser,
) -> Result<CreateCandidacyTemplate, (StatusCode, String)> {
    if user.user_role != "candidate" {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid role : you must be a candidate to create an candidacy".to_string(),
        ));
    }
    Ok(CreateCandidacyTemplate {
        auth_user: Some(user),
    })
}

#[derive(Template, Debug)]
#[template(path = "candidacies/post_create_candidacy.html")]
pub struct CreateCandidacySuccessTemplate {
    auth_user: Option<AuthUser>,
}

pub async fn post_create_candidacy(
    mut multipart: Multipart,
    user: AuthUser,
    Extension(state): Extension<SharedState>,
) -> Result<CreateCandidacySuccessTemplate, (StatusCode, String)> {
    if user.user_role != "candidate" {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid role : you must be a candidate to create a candidacy".to_string(),
        ));
    }
    let mut form_fields = HashMap::new();
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        if let Some(name) = field.name() {
            // Handling the file upload to generate an URL from S3
            if name == "cover_letter_url" {
                let content_type = field.content_type().unwrap().to_string();
                let key = field.file_name().unwrap().to_string();
                let bytes = field.bytes().await.unwrap();
                let uri = upload_bytes_to_s3(
                    bytes,
                    content_type,
                    "cover-letter".to_string(),
                    key,
                    state.clone(),
                )
                .await
                .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err))?;

                let name = String::from("cover_letter_url");
                form_fields.insert(name, uri);
            } else if name == "resume_url" {
                let content_type = field.content_type().unwrap().to_string();
                let key = field.file_name().unwrap().to_string();
                let bytes = field.bytes().await.unwrap();
                let uri = upload_bytes_to_s3(
                    bytes,
                    content_type,
                    "resume".to_string(),
                    key,
                    state.clone(),
                )
                .await
                .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err))?;

                let name = String::from("resume_url");
                form_fields.insert(name, uri);
            }
            // Other fields
            else {
                let name = field.name().unwrap().to_string();
                let content = field.text().await.unwrap().clone();
                form_fields.insert(name, content);
            }
        }
    }

    print!("candidacy: {:?}", form_fields);
    let candidacy = Candidacy::from_hash_map(form_fields);
    create_candidacy(user.email.clone(), candidacy, state)
        .await
        .map_err(|err| {
            tracing::error!("Error creating candidacy: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error creating candidacy".to_string(),
            )
        })?;

    Ok(CreateCandidacySuccessTemplate {
        auth_user: Some(user),
    })
}