use neo4rs::{query, Node};

use super::models::Offer;

use crate::users::models::Company;

use crate::SharedState; // TODO : don't rly understand the purpose of this import

/// Create a new offer made by a company and put it in neo4j
pub async fn create_offer(
    offer: Offer,
    company: Company,
    state: SharedState,
) -> Result<Offer, neo4rs::Error> {
    Ok(offer)
}

// Return all the offers of the site
pub async fn get_offers(state: SharedState) -> Result<Option<Offer>, neo4rs::Error> {
    Ok(None)
}

// Return the list of offer made by a company
pub async fn get_offer_by_company(
    id: String,
    state: SharedState,
) -> Result<Option<Offer>, neo4rs::Error> {
    Ok(None)
}

// Return a single offer in an other page
pub async fn get_offer(id: String, state: SharedState) -> Result<Option<Offer>, neo4rs::Error> {
    Ok(None)
}
