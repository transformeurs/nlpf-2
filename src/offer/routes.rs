use askama::Template;

use crate::users::models::AuthUser;

#[derive(Template)]
#[template(path = "offers/offers.html")]
pub struct OfferTemplate {
    auth_user: Option<AuthUser>,
    list_offers: Vec<String>
}

pub async fn get_offer(user: AuthUser) -> OfferTemplate {
    let mut offers = Vec::new();
    if user.user_role == "candidate" {
        offers.push("cand".to_string());
    }
    else if user.user_role == "company" {
        offers.push("comp".to_string());
    }

    OfferTemplate {
        auth_user: Some(user),
        list_offers: offers
    }
}

#[derive(Template)]
#[template(path = "offers/create_offer.html")]
pub struct CreateOfferTemplate {
    auth_user: Option<AuthUser>,
}

pub async fn create_offer(user: AuthUser) -> CreateOfferTemplate {
    CreateOfferTemplate {
        auth_user: Some(user)
    }
}

#[derive(Template)]
#[template(path = "offers/view_offer.html")]
pub struct ViewOfferTemplate {
    auth_user: Option<AuthUser>
}

pub async fn view_offer(user: AuthUser) -> ViewOfferTemplate {
    ViewOfferTemplate {
        auth_user: Some(user)
    }
}