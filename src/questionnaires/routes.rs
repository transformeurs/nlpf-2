use std::collections::HashMap;

use askama::Template;
use axum::{
    extract::{ContentLengthLimit, Multipart, Path},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Extension,
};

use super::crud::{delete_questionnaire_by_id, get_questionnaires_by_company_email};
use crate::{
    questionnaires::{crud::create_questionnaire, models::Questionnaire},
    users::models::AuthUser,
    SharedState,
};

// ***************************************
// GET /questionnaires
// ***************************************

#[derive(Template)]
#[template(path = "questionnaires/questionnaires.html")]
pub struct QuestionnairesTemplate {
    auth_user: Option<AuthUser>,
    questionnaires: Vec<Questionnaire>,
}

/// GET handler for showing infos
pub async fn get_questionnaires_page(
    user: AuthUser,
    Extension(state): Extension<SharedState>,
) -> QuestionnairesTemplate {
    let l_questionnaires = get_questionnaires_by_company_email(user.email.clone(), state)
        .await
        .unwrap();

    QuestionnairesTemplate {
        auth_user: Some(user),
        questionnaires: l_questionnaires,
    }
}

// ***************************************
// POST /questionnaires
// ***************************************

/// POST handler for creating questionnaire
/// First, we extract the multipart form data from the request and create a
/// hash for each field. Then, create a new questionnaire in the database.
pub async fn post_questionnaire_page(
    user: AuthUser,
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            10 * 1024 * 1024 /* 10 MB */
        },
    >,
    Extension(state): Extension<SharedState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut form_fields = HashMap::new();

    // Fill the hashmap wigth the form fields
    while let Some(field) = multipart
        .next_field()
        .await
        // Return HTTP 400 Bad Request in case of any error
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
    {
        if let Some(name) = field.name() {
            let name = name.to_string();
            let content = field.text().await.unwrap().clone();
            form_fields.insert(name, content);
        }
    }

    println!("Form fields: {:?}", form_fields);

    // Create a new candidate or a new company by reading the userRole field
    let questionnaire = Questionnaire::from_hash_map(form_fields);

    create_questionnaire(questionnaire, user.email, state)
        .await
        .map_err(|err| {
            tracing::error!("Error creating questionnaire: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error creating questionnaire".to_string(),
            )
        })?;

    Ok(Redirect::to("/questionnaires"))
}

// ***************************************
// GET /questionnaires/create
// ***************************************

#[derive(Template)]
#[template(path = "questionnaires/create_questionnaire.html")]
pub struct CreateQuestionnaireTemplate {
    auth_user: Option<AuthUser>,
}

// Get handler for creating a questionnaire
pub async fn get_create_questionnaire_page(
    user: AuthUser,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok(CreateQuestionnaireTemplate {
        auth_user: Some(user),
    })
}

// ***************************************
// DELETE /questionnaires/{id}
// ***************************************

/// DELETE handler for deleting a questionnaire
pub async fn delete_questionnaire_page(
    Path(questionnaire_id): Path<String>,
    Extension(state): Extension<SharedState>,
) -> Result<(), (StatusCode, String)> {
    delete_questionnaire_by_id(questionnaire_id, state)
        .await
        .map_err(|err| {
            tracing::error!("Error deleting questionnaire: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error deleting questionnaire".to_string(),
            )
        })?;

    Ok(())
}
