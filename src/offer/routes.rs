use std::collections::HashMap;

use askama::Template;
use axum::{
    extract::{Multipart, Path},
    http::StatusCode,
    Extension,
};
use uuid::Uuid;

use super::{
    crud::{create_offer, offer_by_company, offer_with_company, offers},
    models::Offer,
};
use crate::{
    questionnaires::{
        crud::{
            delete_questionnaire_by_id, get_questionnaire_by_id,
            get_questionnaires_by_company_email,
        },
        models::Questionnaire,
    },
    users::models::{AuthUser, Company},
    SharedState,
};

#[derive(Template)]
#[template(path = "offers/offers.html")]
pub struct OfferTemplate {
    auth_user: Option<AuthUser>,
    offers: Option<Vec<Offer>>,
}

pub async fn get_offer(user: AuthUser, Extension(state): Extension<SharedState>) -> OfferTemplate {
    let mut l_offers: Option<Vec<Offer>> = Some(Vec::new());
    if user.user_role == "candidate" {
        l_offers = offers(state).await.unwrap();
    } else if user.user_role == "company" {
        l_offers = offer_by_company(user.email.clone(), state).await.unwrap();
    }

    OfferTemplate {
        auth_user: Some(user),
        offers: l_offers,
    }
}

#[derive(Template)]
#[template(path = "offers/offers.html")]
pub struct OfferCompanyTemplate {
    auth_user: Option<AuthUser>,
    offers: Option<Vec<Offer>>,
}

pub async fn get_offer_company(
    Path(company_email): Path<String>,
    user: AuthUser,
    Extension(state): Extension<SharedState>,
) -> OfferCompanyTemplate {
    let l_offers = offer_by_company(company_email, state).await.unwrap();

    OfferCompanyTemplate {
        auth_user: Some(user),
        offers: l_offers,
    }
}

#[derive(Template)]
#[template(path = "offers/create_offer.html")]
pub struct CreateOfferTemplate {
    auth_user: Option<AuthUser>,
    questionnaires: Option<Vec<Questionnaire>>,
}

pub async fn get_create_offer(
    user: AuthUser,
    Extension(state): Extension<SharedState>,
) -> Result<CreateOfferTemplate, (StatusCode, String)> {
    if user.user_role != "company" {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid role : you must be a company to create an offer".to_string(),
        ));
    }

    let questionnaires = get_questionnaires_by_company_email(user.email.clone(), state)
        .await
        .unwrap();

    Ok(CreateOfferTemplate {
        auth_user: Some(user),
        questionnaires: Some(questionnaires),
    })
}

#[derive(Template, Debug)]
#[template(path = "offers/post_create_offer.html")]
pub struct CreateOfferSuccessTemplate {
    auth_user: Option<AuthUser>,
}

pub async fn post_create_offer(
    mut multipart: Multipart,
    user: AuthUser,
    Extension(state): Extension<SharedState>,
) -> Result<CreateOfferSuccessTemplate, (StatusCode, String)> {
    if user.user_role != "company" {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid role : you must be a company to create an offer".to_string(),
        ));
    }
    let mut form_fields = HashMap::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let content = field.text().await.unwrap().clone();
        form_fields.insert(name, content);
    }

    let uuid = Uuid::new_v4();
    let skills_str: Vec<&str> = form_fields.get("skills").unwrap().split(',').collect();
    let skills: Vec<String> = skills_str.iter().map(|&s| s.to_string()).collect();

    let offer = Offer::from_hash_map(form_fields, skills, uuid);
    create_offer(offer, user.email.clone(), state)
        .await
        .map_err(|err| {
            tracing::error!("Error creating offer: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error creating offer".to_string(),
            )
        })?;

    Ok(CreateOfferSuccessTemplate {
        auth_user: Some(user),
    })
}

#[derive(Template)]
#[template(path = "offers/view_offer.html")]
pub struct ViewOfferTemplate {
    auth_user: Option<AuthUser>,
    offer_with_company: Option<(Offer, Company)>,
    questionnaire: Option<Questionnaire>,
}

pub async fn get_view_offer(
    Path(str_uuid): Path<String>,
    user: AuthUser,
    Extension(state): Extension<SharedState>,
) -> ViewOfferTemplate {
    let uuid = Uuid::try_parse(&str_uuid);
    if uuid.is_err() {
        return ViewOfferTemplate {
            auth_user: Some(user),
            offer_with_company: None,
            questionnaire: None,
        };
    }

    let offer_id = uuid.unwrap();
    let crud_res = offer_with_company(offer_id, state.clone()).await.unwrap();
    if let Some(o) = crud_res {
        if let Some(q) = o.0.questionnaire_id {
            let questionnaire = get_questionnaire_by_id(q.to_string(), state.clone())
                .await
                .unwrap();
            return ViewOfferTemplate {
                auth_user: Some(user),
                offer_with_company: Some(o),
                questionnaire,
            };
        }

        return ViewOfferTemplate {
            auth_user: Some(user),
            offer_with_company: Some(o),
            questionnaire: None,
        };
    }

    ViewOfferTemplate {
        auth_user: Some(user),
        offer_with_company: None,
        questionnaire: None,
    }
}
