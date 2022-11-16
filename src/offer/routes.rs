use std::{collections::HashMap, iter::Enumerate};

use askama::Template;

use async_session::chrono::{self, NaiveDate};

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
    title: String,
    uuid: uuid::Uuid,
    description: String,
    created_at: String,
    skills: Vec<String>,
    location: String,
    salary: i64,
    job_duration: String,
    job_start: String,
}

pub async fn get_offer(user: AuthUser, Extension(state): Extension<SharedState>) -> OfferTemplate {
    let mut l_offers: Option<Vec<Offer>> = Some(Vec::new());
    if user.user_role == "candidate" {
        l_offers = offers(state).await.unwrap();
    } else if user.user_role == "company" {
        l_offers = offer_by_company(user.email.clone(), state).await.unwrap();
    }
    let unwrap_l_offer = l_offers.unwrap();

    /*for x in unwrap_l_offer.iter() {
        println!("Title Offer: {}\n", x.title);
    }*/

    OfferTemplate {
        auth_user: Some(user),
        title: unwrap_l_offer[0].title.clone(),
        uuid: unwrap_l_offer[0].uuid.clone(),
        description: unwrap_l_offer[0].description.clone(),
        created_at: unwrap_l_offer[0].created_at.clone().to_string(),
        skills: unwrap_l_offer[0].skills.clone(),
        location: unwrap_l_offer[0].location.clone(),
        salary: unwrap_l_offer[0].salary.clone(),
        job_duration: unwrap_l_offer[0].job_duration.clone(),
        job_start: unwrap_l_offer[0].job_start.clone(),
    }
}
/*
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
*/
#[derive(Template)]
#[template(path = "offers/create_offer.html")]
pub struct CreateOfferTemplate {
    auth_user: Option<AuthUser>,
}

pub async fn get_create_offer(user: AuthUser) -> Result<CreateOfferTemplate, (StatusCode, String)> {
    if user.user_role != "company" {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid role : you must be a company to create an offer".to_string(),
        ));
    }
    Ok(CreateOfferTemplate {
        auth_user: Some(user),
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
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let content = field.text().await.unwrap().clone();
        form_fields.insert(name, content);
    }

    let uuid = Uuid::new_v4();
    let skills_str: Vec<&str> = form_fields.get("skills").unwrap().split(",").collect();
    // convert Vec<&str> to Vec<String>
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
/*
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

*/
