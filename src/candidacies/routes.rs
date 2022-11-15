use std::collections::HashMap;

use askama::Template;

use axum::{
    extract::{Multipart, Path},
    http::StatusCode,
    Extension,
};
use uuid::Uuid;

use super::{crud::create_candidacy, models::Candidacy};
use crate::{users::models::AuthUser, SharedState};

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
        let name = field.name().unwrap().to_string();
        let content = field.text().await.unwrap().clone();
        form_fields.insert(name, content);
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
