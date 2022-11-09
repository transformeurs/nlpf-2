use neo4rs::{query, Node, Relation};

use super::models::Candidacy;

use crate::{users::models::Candidate, SharedState};

/// Create a new candidate in the database and return it
pub async fn create_candidacy(
    candidate: Candidate,
    //offer: Offer,
    candidacy: Candidacy,
    state: SharedState,
) -> Result<Candidacy, neo4rs::Error> {
    tracing::info!("Creating candidacy:");

    let mut result = state
        .graph
        .execute(
            query(
                r#"
                CREATE (o:Offer {id: 1})
                MATCH (c:Candidate)
                WITH c
                MATCH (o:Offer)
                WHERE c.name = $candidate.name AND o.id = $offer.id
                CREATE (c)-[:CANDIDATE_TO {
                    created_at: $created_at,
                    status: $status,
                    cover_letter_url: $cover_letter_url,
                    resume_url: $resume_url,
                    custom_field: $custom_field
                }]->(o)
                RETURN r
        "#,
            )
            .param("status", candidacy.status.clone())
            .param("cover_letter_url", candidacy.cover_letter_url.clone())
            .param("resume_url", candidacy.resume_url.clone())
            .param("custom_field", candidacy.custom_field.clone()),
        )
        .await?;

    // Check if created, and log the name
    while let Ok(Some(row)) = result.next().await {
        let relation: Relation = row.get("r").unwrap();
        let offer_id: String = relation.get("offer_id").unwrap();
        tracing::info!("Created candidacy: {offer_id}");
    }

    Ok(candidacy)
}
