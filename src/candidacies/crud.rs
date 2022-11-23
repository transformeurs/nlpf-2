use neo4rs::{query, Relation};

use super::models::Candidacy;
use crate::SharedState;

/// Create a new candidacy in the database and put it in neo4j
pub async fn create_candidacy(
    uuid_str: uuid::Uuid,
    candidate_email: String,
    candidacy: Candidacy,
    state: SharedState,
) -> Result<Candidacy, neo4rs::Error> {
    let uuid_offer = uuid_str.to_string();
    tracing::info!("Creating candidacy: {}", &candidate_email);

    let mut result = state
        .graph
        .execute(
            query(
                r#"
                MATCH (c:Candidate)
                WITH c
                MATCH (o:Offer)
                WHERE c.email = $email AND o.uuid = $uuid_offer
                CREATE (c)-[r:CANDIDATE_TO {
                    uuid : $uuid,
                    status: $status,
                    cover_letter_url: $cover_letter_url,
                    resume_url: $resume_url,
                    custom_field: $custom_field
                }]->(o)
                RETURN r
        "#,
            )
            .param("email", candidate_email.clone())
            .param("uuid", candidacy.uuid.to_string())
            .param("status", candidacy.status.clone())
            .param("cover_letter_url", candidacy.cover_letter_url.clone())
            .param("resume_url", candidacy.resume_url.clone())
            .param("custom_field", candidacy.custom_field.clone())
            .param("uuid_offer", uuid_offer),
        )
        .await?;

    // Check if created, and log the name
    while let Ok(Some(row)) = result.next().await {
        let relation: Relation = row.get("r").unwrap();
        let field: String = relation.get("custom_field").unwrap();
        tracing::info!("Created candidacy: {field}");
    }

    Ok(candidacy)
}

pub async fn candidacy_by_candidate(
    email: String,
    state: SharedState,
) -> Result<Option<Vec<Candidacy>>, neo4rs::Error> {
    tracing::info!("Getting candidacy by candidate email: {}", email);

    let mut result = state
        .graph
        .execute(
            query(
                r#"
            MATCH (c:Candidate {email: $email})-[r:CANDIDATE_TO]->(o:Offer)
            RETURN r
        "#,
            )
            .param("email", email.to_string()),
        )
        .await?;

    let mut candidacies: Vec<Candidacy> = Vec::new();

    while let Ok(Some(row)) = result.next().await {
        let relation: Relation = row.get("r").unwrap();
        let custom_field: String = relation.get("custom_field").unwrap();
        tracing::info!("Found candidacy: {custom_field}");
        candidacies.push(Candidacy::from_relation(relation));
    }

    if !candidacies.is_empty() {
        return Ok(Some(candidacies));
    }

    Ok(None)
}

// Return a single offer in an other page
pub async fn candidacy(
    uuid_str: uuid::Uuid,
    state: SharedState,
) -> Result<Option<Candidacy>, neo4rs::Error> {
    let uuid = uuid_str.to_string();
    tracing::info!("Getting candidacy by uuid: {}", uuid);

    let mut result = state
        .graph
        .execute(
            query(
                r#"
            MATCH (c:Candidate)-[r:CANDIDATE_TO]->(o:Offer)
            WHERE r.uuid = $uuid
            RETURN r
        "#,
            )
            .param("uuid", uuid),
        )
        .await?;

    if let Ok(Some(row)) = result.next().await {
        let relation: Relation = row.get("r").unwrap();
        let custom_field: String = relation.get("custom_field").unwrap();
        tracing::info!("Found offer: {custom_field}");
        return Ok(Some(Candidacy::from_relation(relation)));
    }

    Ok(None)
}

pub async fn candidacy_by_offer(
    company_email: String,
    uuid_str: uuid::Uuid,
    state: SharedState,
) -> Result<Option<Vec<Candidacy>>, neo4rs::Error> {
    tracing::info!("Getting candidacies by offer: {}", company_email);

    let uuid_offer = uuid_str.to_string();
    let mut result = state
        .graph
        .execute(
            query(
                r#"
            MATCH (company:Company {email: $company_email})-[posted:POSTED]->(offer:Offer {uuid: $uuid_offer})<-[relation:CANDIDATE_TO]-(candidate:Candidate)
            RETURN relation
        "#,
            )
            .param("company_email", company_email.to_string())
            .param("uuid_offer", uuid_offer.to_string()),
        )
        .await?;

    let mut candidacies: Vec<Candidacy> = Vec::new();

    while let Ok(Some(row)) = result.next().await {
        let relation: Relation = row.get("relation").unwrap();
        let custom_field: String = relation.get("custom_field").unwrap();
        tracing::info!("Found candidacy: {custom_field}");
        candidacies.push(Candidacy::from_relation(relation));
    }

    if !candidacies.is_empty() {
        return Ok(Some(candidacies));
    }

    Ok(None)
}

pub async fn accept_candidacy(
    email: String,
    uuid_str: uuid::Uuid,
    state: SharedState,
) -> Result<SharedState, neo4rs::Error> {
    tracing::info!("Accepting candidacy: {}", uuid_str);

    let uuid_candidacy = uuid_str.to_string();
    let mut result = state
        .graph
        .execute(
            query(
                r#"
            MATCH (company:Company {email: $email})-[posted:POSTED]->(offer:Offer)<-[relation:CANDIDATE_TO {uuid: $uuid}]-(candidate:Candidate)
            SET relation.status="accepted"
            RETURN relation
        "#,
            )
            .param("email", email.to_string())
            .param("uuid", uuid_candidacy.to_string()),
        )
        .await?;

    while let Ok(Some(row)) = result.next().await {
        let relation: Relation = row.get("relation").unwrap();
        let custom_field: String = relation.get("custom_field").unwrap();
        tracing::info!("candidacy accepted: {custom_field}");
    }

    Ok(state)
}

pub async fn refuse_candidacy(
    email: String,
    uuid_str: uuid::Uuid,
    state: SharedState,
) -> Result<SharedState, neo4rs::Error> {
    tracing::info!("Refusing candidacy: {}", uuid_str);

    let uuid_candidacy = uuid_str.to_string();
    let mut result = state
        .graph
        .execute(
            query(
                r#"
            MATCH (company:Company {email: $email})-[posted:POSTED]->(offer:Offer)<-[relation:CANDIDATE_TO {uuid: $uuid}]-(candidate:Candidate)
            SET relation.status="refused"
            RETURN relation
        "#,
            )
            .param("email", email.to_string())
            .param("uuid", uuid_candidacy.to_string()),
        )
        .await?;

    while let Ok(Some(row)) = result.next().await {
        let relation: Relation = row.get("relation").unwrap();
        let custom_field: String = relation.get("custom_field").unwrap();
        tracing::info!("candidacy refused: {custom_field}");
    }

    Ok(state)
}
