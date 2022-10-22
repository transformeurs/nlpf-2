use std::collections::HashMap;

use axum::{extract, Extension};
use neo4rs::*;

use crate::SharedState;

pub async fn neo_create_user(
    extract::Query(params): extract::Query<HashMap<String, String>>,
    Extension(state): Extension<SharedState>,
) {
    let username = params.get("name").unwrap().to_string();
    tracing::info!("Creating user: {}", &username);
    let mut result = state
        .graph
        .execute(query("CREATE (u:User {name: $name}) RETURN u").param("name", username))
        .await
        .unwrap();

    while let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("u").unwrap();
        let name: String = node.get("name").unwrap();
        tracing::info!("Created user: {name}");
    }
}
