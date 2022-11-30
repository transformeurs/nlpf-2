mod candidacies;
mod config;
mod homepage;
mod offer;
mod questionnaires;
mod users;
mod utils;

use std::sync::Arc;

use async_redis_session::RedisSessionStore;
use aws_sdk_s3::Client;
use axum::{routing::get, Extension, Router};
use neo4rs::*;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Settings;

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

    let store =
        RedisSessionStore::new(config.redis.get_uri()).expect("Failed to connect to Redis.");

    // Build our application with some routes
    Router::new()
        .route("/", get(homepage::get_home_page))
        .nest("/", users::get_router())
        .nest("/", candidacies::get_router())
        .nest("/", offer::get_router())
        .nest("/questionnaires", questionnaires::get_router())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(shared_state))
        .layer(Extension(store))
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

    // Build the app
    let app = build_app(config).await;

    // Run the app
    tracing::info!("Listening on {}", uri);
    axum::Server::bind(&uri)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
