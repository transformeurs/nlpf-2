use neo4rs::{query, Node};

use super::models::{Candidate, Company};
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

/// Create a new company in the database and return it
pub async fn create_company(
    company: Company,
    state: SharedState,
) -> Result<Company, neo4rs::Error> {
    tracing::info!("Creating company: {}", &company.name);

    let mut result = state
        .graph
        .execute(
            query(
                r#"
            CREATE (c:Company {
                name: $name,
                email: $email,
                password: $password,
                photo_url: $photo_url,
                description: $description
            })
            RETURN c
        "#,
            )
            .param("name", company.name.clone())
            .param("email", company.email.clone())
            .param("password", company.password.clone())
            .param("photo_url", company.photo_url.clone())
            .param("description", company.description.clone()),
        )
        .await?;

    // Check if created, and log the name
    while let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("c").unwrap();
        let name: String = node.get("name").unwrap();
        tracing::info!("Created company: {name}");
    }

    Ok(company)
}

/// Get a company by email
pub async fn get_company_by_email(
    email: String,
    state: SharedState,
) -> Result<Option<Company>, neo4rs::Error> {
    tracing::info!("Getting company by email: {}", &email);

    let mut result = state
        .graph
        .execute(
            query(
                r#"
            MATCH (c:Company {email: $email})
            RETURN c
        "#,
            )
            .param("email", email),
        )
        .await?;

    while let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("c").unwrap();
        let name: String = node.get("name").unwrap();
        tracing::info!("Found company: {name}");
        return Ok(Some(Company::from_node(node)));
    }

    Ok(None)
}
