use neo4rs::{query, Relation};

use super::models::Candidacy;

use crate::SharedState;

/// Create a new candidacy in the database and put it in neo4j
pub async fn create_candidacy(
    //candidate: Candidate,
    candidate_email: String,
    candidacy: Candidacy,
    state: SharedState,
) -> Result<Candidacy, neo4rs::Error> {
    tracing::info!("Creating candidacy: {}", &candidate_email);

    let mut result = state
        .graph
        .execute(
            query(
                r#"
                MATCH (c:Candidate)
                WITH c
                MATCH (o:Offer)
                WHERE c.email = $email AND o.title = "Stage"
                CREATE (c)-[r:CANDIDATE_TO {
                    status: $status,
                    cover_letter_url: $cover_letter_url,
                    resume_url: $resume_url,
                    custom_field: $custom_field
                }]->(o)
                RETURN r
        "#,
            )
            .param("email", candidate_email.clone())
            .param("status", candidacy.status.clone())
            .param("cover_letter_url", candidacy.cover_letter_url.clone())
            .param("resume_url", candidacy.resume_url.clone())
            .param("custom_field", candidacy.custom_field.clone()),
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
) -> Result<Option<Candidacy>, neo4rs::Error> {
    tracing::info!("Getting candidacy by candidate email: {}", email);

    let mut result = state
        .graph
        .execute(
            query(
                r#"
            MATCH (r:Candidacy {email: $email})
            RETURN r
        "#,
            )
            .param("email", email.to_string()),
        )
        .await?;

    if let Ok(Some(row)) = result.next().await {
        let relation: Relation = row.get("r").unwrap();
        let email: String = relation.get("email").unwrap();
        tracing::info!("Found candidacy for : {email}");
        return Ok(Some(Candidacy::from_relation(relation)));
    }

    Ok(None)
}
