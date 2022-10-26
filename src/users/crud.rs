use neo4rs::{query, Graph, Node};

use super::models::Candidate;

pub async fn create_candidate(
    candidate: Candidate,
    graph: Graph,
) -> Result<Candidate, neo4rs::Error> {
    tracing::info!("Creating candidate: {}", &candidate.name);

    let mut result = graph
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

    while let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("u").unwrap();
        let name: String = node.get("name").unwrap();
        tracing::info!("Created candidate: {name}");
    }

    Ok(candidate)
}
