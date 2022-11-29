use neo4rs::{query, Node};

use super::models::Offer;
use crate::{users::models::Company, SharedState};

// Create a new offer made by a company and put it in neo4j
pub async fn create_offer(
    offer: Offer,
    company_email: String,
    state: SharedState,
) -> Result<Offer, neo4rs::Error> {
    tracing::info!("Creating offer: {}", &offer.title);

    let mut result = state
        .graph
        .execute(
            query(
                r#"
            MATCH (c:Company {email: $email})
            CREATE (o:Offer {
                title : $title,
                uuid : $uuid,
                description : $description,
                created_at : $created_at,
                skills : $skills,
                location : $location,
                salary : $salary,
                job_duration : $job_duration,
                job_start : $job_start,
                questionnaire_id : $questionnaire_id
            })
            CREATE (c)-[:POSTED]->(o)
            RETURN o
        "#,
            )
            .param("title", offer.title.clone())
            .param("uuid", offer.uuid.to_string())
            .param("description", offer.description.clone())
            .param("created_at", offer.created_at)
            .param("skills", offer.skills.clone())
            .param("location", offer.location.clone())
            .param("salary", offer.salary)
            .param("job_duration", offer.job_duration.clone())
            .param("job_start", offer.job_start.clone())
            .param("email", company_email.clone())
            .param(
                "questionnaire_id",
                offer
                    .questionnaire_id
                    .map_or("0".to_string(), |id| id.to_string()),
            ),
        )
        .await?;

    // Check if created, and log the name
    while let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("o").unwrap();
        let title: String = node.get("title").unwrap();
        tracing::info!("Created offer: {title}");
    }

    Ok(offer)
}

// Return all the offers of the site
pub async fn offers(state: SharedState) -> Result<Option<Vec<Offer>>, neo4rs::Error> {
    tracing::info!("Getting all offers");

    let mut result = state
        .graph
        .execute(query(
            r#"
        MATCH (o:Offer)
        RETURN o
    "#,
        ))
        .await?;

    let mut offers: Vec<Offer> = Vec::new();

    while let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("o").unwrap();
        let title: String = node.get("title").unwrap();
        tracing::info!("Found offer: {title}");
        offers.push(Offer::from_node(node));
    }

    if !offers.is_empty() {
        return Ok(Some(offers));
    }

    Ok(None)
}

// Return the list of offer made by a company
pub async fn offer_by_company(
    email: String,
    state: SharedState,
) -> Result<Option<Vec<Offer>>, neo4rs::Error> {
    tracing::info!("Getting offer by company email: {}", email);

    let mut result = state
        .graph
        .execute(
            query(
                r#"
            MATCH (c:Company {email:$email})
            MATCH (c)-[POSTED]-(o:Offer)
            RETURN o
        "#,
            )
            .param("email", email.to_string()),
        )
        .await?;

    let mut offers: Vec<Offer> = Vec::new();

    while let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("o").unwrap();
        let title: String = node.get("title").unwrap();
        tracing::info!("Found offer: {title}");
        offers.push(Offer::from_node(node));
    }

    if !offers.is_empty() {
        return Ok(Some(offers));
    }

    Ok(None)
}

// Return a single offer in an other page
pub async fn offer_with_company(
    uuid_str: uuid::Uuid,
    state: SharedState,
) -> Result<Option<(Offer, Company)>, neo4rs::Error> {
    let uuid = uuid_str.to_string();
    tracing::info!("Getting offer by uuid: {}", uuid);

    let mut result = state
        .graph
        .execute(
            query(
                r#"
            MATCH (o:Offer {uuid: $uuid})<-[:POSTED]-(c:Company)
            RETURN o, c
        "#,
            )
            .param("uuid", uuid),
        )
        .await?;

    if let Ok(Some(row)) = result.next().await {
        let offer_node: Node = row.get("o").unwrap();
        let company_node: Node = row.get("c").unwrap();
        return Ok(Some((
            Offer::from_node(offer_node),
            Company::from_node(company_node),
        )));
    }

    Ok(None)
}
