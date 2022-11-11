use crate::SharedState;

/// Create a new candidate in the database and return it
pub async fn create_candidate(
    questionnaire: Questionnaire,
    state: SharedState,
) -> Result<Questionnaire, neo4rs::Error> {
    tracing::info!("Creating candidate: {}", &questionnaire.name);

    // let mut result = state
    //     .graph
    //     .execute(
    //         query(
    //             r#"
    //         CREATE (c:Candidate {
    //             name: $name,
    //             email: $email,
    //             password: $password,
    //             age: $age,
    //             photo_url: $photo_url,
    //             description: $description
    //         })
    //         RETURN c
    //     "#,
    //         )
    //         .param("name", candidate.name.clone())
    //         .param("email", candidate.email.clone())
    //         .param("password", candidate.password.clone())
    //         .param("age", candidate.age)
    //         .param("photo_url", candidate.photo_url.clone())
    //         .param("description", candidate.description.clone()),
    //     )
    //     .await?;

    // // Check if created, and log the name
    // while let Ok(Some(row)) = result.next().await {
    //     let node: Node = row.get("c").unwrap();
    //     let name: String = node.get("name").unwrap();
    //     tracing::info!("Created candidate: {name}");
    // }

    // Ok(candidate)
}
