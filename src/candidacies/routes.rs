use std::collections::HashMap;

use askama::Template;
use axum::{
    extract::{Multipart, Path},
    http::StatusCode,
    Extension,
};
use uuid::Uuid;

use super::{
    crud::{
        accept_candidacy, candidacy, candidacy_by_candidate, candidacy_by_offer, create_candidacy,
        refuse_candidacy,
    },
    models::Candidacy,
};
use crate::{users::models::AuthUser, utils::s3::upload_bytes_to_s3, SharedState};

#[derive(Template)]
#[template(path = "candidacies/create_candidacy.html")]
pub struct CreateCandidacyTemplate {
    auth_user: Option<AuthUser>,
    uuid_offer: String,
}

pub async fn get_create_candidacy(
    Path(str_uuid): Path<String>,
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
        uuid_offer: str_uuid,
    })
}

#[derive(Template, Debug)]
#[template(path = "candidacies/post_create_candidacy.html")]
pub struct CreateCandidacySuccessTemplate {
    auth_user: Option<AuthUser>,
}

pub async fn post_create_candidacy(
    Path(str_uuid): Path<String>,
    mut multipart: Multipart,
    user: AuthUser,
    Extension(state): Extension<SharedState>,
) -> Result<CreateCandidacySuccessTemplate, (StatusCode, String)> {
    let uuid_offer = Uuid::try_parse(&str_uuid);

    if user.user_role != "candidate" {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid role : you must be a candidate to create a candidacy".to_string(),
        ));
    }
    let mut form_fields = HashMap::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
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
    // Hardcode to status value to "pending" (every created offer should have pending status)
    let name = "status";
    let content = "pending";
    form_fields.insert(name.to_string(), content.to_string());

    let uuid = Uuid::new_v4();
    println!("uuid_offer = {}", str_uuid);
    let candidacy = Candidacy::from_hash_map(form_fields, uuid);
    create_candidacy(uuid_offer.unwrap(), user.email.clone(), candidacy, state)
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

#[derive(Template)]
#[template(path = "candidacies/candidacies.html")]
pub struct CandidacyTemplate {
    auth_user: Option<AuthUser>,
    candidacies: Option<Vec<Candidacy>>,
}

pub async fn get_candidacy(
    user: AuthUser,
    Extension(state): Extension<SharedState>,
) -> CandidacyTemplate {
    let mut l_candidacies: Option<Vec<Candidacy>> = Some(Vec::new());
    //if user.user_role == "candidate" {
    //    l_candidacies = candidacies(state).await.unwrap();
    if user.user_role == "company" {
        l_candidacies = candidacy_by_candidate(user.email.clone(), state)
            .await
            .unwrap();
    }

    CandidacyTemplate {
        auth_user: Some(user),
        candidacies: l_candidacies,
    }
}

#[derive(Template)]
#[template(path = "candidacies/candidacies.html")]
pub struct CandidacyCandidateTemplate {
    auth_user: Option<AuthUser>,
    candidacies: Option<Vec<Candidacy>>,
}

pub async fn get_candidacy_candidate(
    user: AuthUser,
    Extension(state): Extension<SharedState>,
) -> CandidacyCandidateTemplate {
    let list_candidacies = candidacy_by_candidate(user.email.clone(), state)
        .await
        .unwrap();

    CandidacyCandidateTemplate {
        auth_user: Some(user),
        candidacies: list_candidacies,
    }
}

#[derive(Template)]
#[template(path = "candidacies/view_candidacy.html")]
pub struct ViewCandidacyTemplate {
    auth_user: Option<AuthUser>,
    candidacy: Option<Candidacy>,
}

pub async fn get_view_candidacy(
    Path(str_uuid): Path<String>,
    user: AuthUser,
    Extension(state): Extension<SharedState>,
) -> ViewCandidacyTemplate {
    println!("uuid = {}", str_uuid);
    let uuid = Uuid::try_parse(&str_uuid);
    if uuid.is_err() {
        return ViewCandidacyTemplate {
            auth_user: Some(user),
            candidacy: None,
        };
    }
    let candidacy_res = candidacy(uuid.unwrap(), state).await.unwrap();

    ViewCandidacyTemplate {
        auth_user: Some(user),
        candidacy: candidacy_res,
    }
}

#[derive(Template)]
#[template(path = "candidacies/candidacies_by_offer.html")]
pub struct ViewCandidacyByOfferTemplate {
    auth_user: Option<AuthUser>,
    candidacies: Option<Vec<Candidacy>>,
}

pub async fn get_view_candidacy_by_offer(
    Path(str_uuid): Path<String>,
    user: AuthUser,
    Extension(state): Extension<SharedState>,
) -> ViewCandidacyByOfferTemplate {
    println!("uuid = {}", str_uuid);
    let uuid = Uuid::try_parse(&str_uuid);

    if uuid.is_err() {
        return ViewCandidacyByOfferTemplate {
            auth_user: Some(user),
            candidacies: None,
        };
    }
    let l_candidacies = candidacy_by_offer(user.email.clone(), uuid.unwrap(), state)
        .await
        .unwrap();

    ViewCandidacyByOfferTemplate {
        auth_user: Some(user),
        candidacies: l_candidacies,
    }
}

#[derive(Template)]
#[template(path = "candidacies/post_accept_candidacy.html")]
pub struct AcceptCandidacyTemplate {
    auth_user: Option<AuthUser>,
}

pub async fn post_accept_candidacy(
    Path(str_uuid): Path<String>,
    user: AuthUser,
    Extension(state): Extension<SharedState>,
) -> Result<AcceptCandidacyTemplate, (StatusCode, String)> {
    println!("uuid = {}", str_uuid);
    let uuid = Uuid::try_parse(&str_uuid);

    accept_candidacy(user.email.clone(), uuid.unwrap(), state)
        .await
        .map_err(|err| {
            tracing::error!("Error accepting candidacy: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error accepting candidacy".to_string(),
            )
        })?;

    Ok(AcceptCandidacyTemplate {
        auth_user: Some(user),
    })
}

#[derive(Template)]
#[template(path = "candidacies/post_refuse_candidacy.html")]
pub struct RefuseCandidacyTemplate {
    auth_user: Option<AuthUser>,
}

pub async fn post_refuse_candidacy(
    Path(str_uuid): Path<String>,
    user: AuthUser,
    Extension(state): Extension<SharedState>,
) -> Result<RefuseCandidacyTemplate, (StatusCode, String)> {
    println!("uuid = {}", str_uuid);
    let uuid = Uuid::try_parse(&str_uuid);

    refuse_candidacy(user.email.clone(), uuid.unwrap(), state)
        .await
        .map_err(|err| {
            tracing::error!("Error refusing candidacy: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error refusing candidacy".to_string(),
            )
        })?;

    Ok(RefuseCandidacyTemplate {
        auth_user: Some(user),
    })
}
