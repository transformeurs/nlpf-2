mod config;
mod greet;
mod neo_test;
mod s3_test;

use std::sync::Arc;

use aws_sdk_s3::Client;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use neo4rs::*;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{config::Settings, greet::greet_template, neo_test::neo_create_user, s3_test::test_s3};

pub struct State {
    graph: Graph,
    s3_client: Client,
}

type SharedState = Arc<State>;

/// Build the application router and embed connections to DB and S3.
async fn build_app(config: Settings) -> Router {
    let graph = config
        .neo4j
        .get_connection()
        .await
        .expect("Failed to connect to Neo4j.");
    let s3_client = config.s3.get_s3_client().await;

    let shared_state = Arc::new(State { graph, s3_client });

    // Build our application with some routes
    Router::new()
        .route("/greet/:name", get(greet_template))
        .route("/neotest", post(neo_create_user))
        .route("/tests3", get(test_s3))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(shared_state))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "nlpf_2=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::get_config().expect("Failed to read configuration.");
    let uri = config.uri;

    let app = build_app(config).await;

    // Run it
    tracing::info!("Listening on {}", uri);
    axum::Server::bind(&uri)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
