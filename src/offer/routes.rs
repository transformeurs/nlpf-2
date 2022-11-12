use std::collections::HashMap;

use askama::Template;

use async_session::log::kv::Error;
use axum::{
    extract::{ContentLengthLimit, Multipart, Path},
    http::StatusCode,
};

use super::crud::{create_offer, offer, offer_by_company, offers};
use crate::users::models::AuthUser;

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

#[derive(Template)]
#[template(path = "offers/post_create_offer.html")]
pub struct CreateOfferSuccessTemplate {
    auth_user: Option<AuthUser>,
}

pub async fn post_create_offer(
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            10 * 1024 * 1024 /* 10 MB */
        },
    >,
    user: AuthUser,
) -> Result<CreateOfferSuccessTemplate, (StatusCode, String)> {
    let mut form_fields: HashMap<String, String> = HashMap::new();
    while let Some(field) = multipart
        .next_field()
        .await
        // Return HTTP 400 Bad Request in case of any error
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
    {
        print!("field: {}", field.name().unwrap());
    }
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
