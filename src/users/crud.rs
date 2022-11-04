use neo4rs::{query, Node};

use super::models::Candidate;
use crate::SharedState;

/// Create a new candidate in the database and return it
pub async fn create_candidate(
    candidate: Candidate,
    state: SharedState,
) -> Result<Candidate, neo4rs::Error> {
    tracing::info!("Creating candidate: {}", &candidate.name);

    let mut result = state
        .graph
        .execute(
            query(
                r#"
            CREATE (c:Candidate {
                name: $name,
                email: $email,
                password: $password,
                age: $age,
                photo_url: $photo_url,
                description: $description
            })
            RETURN c
        "#,
            )
            .param("name", candidate.name.clone())
            .param("email", candidate.email.clone())
            .param("password", candidate.password.clone())
            .param("age", candidate.age)
            .param("photo_url", candidate.photo_url.clone())
            .param("description", candidate.description.clone()),
        )
        .await?;

    // Check if created, and log the name
    while let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("c").unwrap();
        let name: String = node.get("name").unwrap();
        tracing::info!("Created candidate: {name}");
    }

    Ok(candidate)
}

/// Get a candidate by email
pub async fn get_candidate_by_email(
    email: String,
    state: SharedState,
) -> Result<Option<Candidate>, neo4rs::Error> {
    tracing::info!("Getting candidate by email: {}", &email);

    let mut result = state
        .graph
        .execute(
            query(
                r#"
            MATCH (c:Candidate {email: $email})
            RETURN c
        "#,
            )
            .param("email", email),
        )
        .await?;

    while let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("c").unwrap();
        let name: String = node.get("name").unwrap();
        tracing::info!("Found candidate: {name}");
        return Ok(Some(Candidate::from_node(node)));
    }

    Ok(None)
}
