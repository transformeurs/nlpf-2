use askama::Template;
use axum::{http::StatusCode, response::IntoResponse};

use crate::users::models::AuthUser;

#[derive(Template)]
#[template(path = "questionnaires.html")]
pub struct QuestionnairesTemplate {
    auth_user: Option<AuthUser>,
}

/// GET handler for showing infos
pub async fn get_questionnaires_page(
    user: AuthUser,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok(QuestionnairesTemplate {
        auth_user: Some(user),
    })
}

#[derive(Template)]
#[template(path = "create_questionnaire.html")]
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
