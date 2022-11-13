use std::collections::HashMap;

use askama::Template;

use axum::{
    extract::{Multipart, Path},
    http::StatusCode,
    Extension,
};
use uuid::Uuid;

use super::{
    crud::{create_offer, offer, offer_by_company, offers},
    models::Offer,
};
use crate::{users::models::AuthUser, SharedState};

#[derive(Template)]
#[template(path = "offers/offers.html")]
pub struct OfferTemplate {
    auth_user: Option<AuthUser>,
    list_offers: Vec<String>,
}

pub async fn get_offer(user: AuthUser) -> OfferTemplate {
    let mut offers = Vec::new();
    if user.user_role == "candidate" {
        offers.push("cand".to_string());
    } else if user.user_role == "company" {
        offers.push("comp".to_string());
    }

    OfferTemplate {
        auth_user: Some(user),
        list_offers: offers,
    }
}

#[derive(Template)]
#[template(path = "offers/offers.html")]
pub struct OfferCompanyTemplate {
    auth_user: Option<AuthUser>,
    list_offers: Vec<String>,
}

pub async fn get_offer_company(
    Path(company_name): Path<String>,
    user: AuthUser,
) -> OfferCompanyTemplate {
    let mut offers = Vec::new();
    if user.user_role == "candidate" {
        offers.push("cand".to_string());
    } else if user.user_role == "company" {
        offers.push("comp".to_string());
    }
    print!("company_name: {}", company_name);
    OfferCompanyTemplate {
        auth_user: Some(user),
        list_offers: offers,
    }
}

#[derive(Template)]
#[template(path = "offers/create_offer.html")]
pub struct CreateOfferTemplate {
    auth_user: Option<AuthUser>,
}

pub async fn get_create_offer(user: AuthUser) -> CreateOfferTemplate {
    CreateOfferTemplate {
        auth_user: Some(user),
    }
}

#[derive(Template, Debug)]
#[template(path = "offers/post_create_offer.html")]
pub struct CreateOfferSuccessTemplate {
    auth_user: Option<AuthUser>,
}

pub async fn post_create_offer(
    mut multipart: Multipart,
    user: AuthUser,
    Extension(state): Extension<SharedState>, // Ca sert a quoi ?
) -> Result<CreateOfferSuccessTemplate, (StatusCode, String)> {
    if user.user_role != "company" {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid role : you must be a company".to_string(),
        ));
    }

    let mut form_fields = HashMap::new();
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let content = field.text().await.unwrap().clone();
        form_fields.insert(name, content);
    }

    let uuid = Uuid::new_v4();
    let skills_str: Vec<&str> = form_fields.get("skills").unwrap().split(",").collect();
    // convert Vec<&str> to Vec<String>
    let skills: Vec<String> = skills_str.iter().map(|&s| s.to_string()).collect();

    print!("offer: {:?}", form_fields);
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
}

pub async fn get_view_offer(user: AuthUser) -> ViewOfferTemplate {
    ViewOfferTemplate {
        auth_user: Some(user),
    }
}
