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
    tracing::info!("Creating offer: {}", &offer.title);

    let mut result = state
        .graph
        .execute(
            query(
                r#"
            MATCH (c:Company {email: $email})
            CREATE (o:Offer {
                title : $title,
                uid : $uid,
                description : $description,
                created_at : $created_at,
                skills : $skills,
                location : $location,
                salary : $salary,
                job_duration : $job_duration,
                job_start : $job_start
            }),
            (o)-[:POSTED]->(c) 
            RETURN o
        "#,
            )
            .param("title", offer.title.clone())
            .param("uuid", offer.uuid.clone().to_string())
            .param("description", offer.description.clone())
            .param("created_at", offer.created_at.clone())
            .param("skills", offer.skills.clone())
            .param("location", offer.location.clone())
            .param("salary", offer.salary.clone())
            .param("job_duration", offer.job_duration.clone())
            .param("job_start", offer.job_start.clone())
            .param("email", company.email.clone()),
        )
        .await?;

    // Check if created, and log the name
    while let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("c").unwrap();
        let title: String = node.get("title").unwrap();
        tracing::info!("Created offer: {title}");
    }

    Ok(offer)
}

// Return all the offers of the site
pub async fn offers(state: SharedState) -> Result<Option<Offer>, neo4rs::Error> {
    tracing::info!("Getting all offers");

    let mut result = state
        .graph
        .execute(
            query(
                r#"
            MATCH (c:Offer)
            RETURN c
        "#,
            ),
        )
        .await?;

    if let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("c").unwrap();
        let title: String = node.get("title").unwrap();
        tracing::info!("Found offer: {title}");
        return Ok(Some(Offer::from_node(node)));
    }

    Ok(None)
}

// Return the list of offer made by a company
// TODO : put all the offers linked to a company
pub async fn offer_by_company(
    name: String,
    state: SharedState,
) -> Result<Option<Offer>, neo4rs::Error> {
    tracing::info!("Getting offer by company name: {}", name);

    let mut result = state
        .graph
        .execute(
            query(
                r#"
            MATCH (c:Offer {uuid: $uuid})
            RETURN c
        "#,
            )
            .param("name", name.to_string()),
        )
        .await?;

    if let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("c").unwrap();
        let title: String = node.get("title").unwrap();
        tracing::info!("Found offer: {title}");
        return Ok(Some(Offer::from_node(node)));
    }

    Ok(None)
}

// Return a single offer in an other page
pub async fn offer(id: uuid::Uuid, state: SharedState) -> Result<Option<Offer>, neo4rs::Error> {
    tracing::info!("Getting offer by uuid: {}", id.to_string());

    let mut result = state
        .graph
        .execute(
            query(
                r#"
            MATCH (c:Offer {uuid: $uuid})
            RETURN c
        "#,
            )
            .param("uuid", id.to_string()),
        )
        .await?;

    if let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("c").unwrap();
        let title: String = node.get("title").unwrap();
        tracing::info!("Found offer: {title}");
        return Ok(Some(Offer::from_node(node)));
    }

    Ok(None)
}
