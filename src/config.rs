use std::net::SocketAddr;

use aws_sdk_s3::{Credentials, Endpoint, Region};
use neo4rs::Graph;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Deserialize)]
pub struct Settings {
    #[serde_as(as = "DisplayFromStr")]
    pub uri: SocketAddr,
    pub s3: S3Settings,
    pub neo4j: Neo4jSettings,
}

/// Retrieve the settings from the `./settings/config.yaml` file.
/// Override settings with environment variables. For example NLPF__S3__AWS_ACCESS_KEY_ID will
/// override s3.aws_access_key_id.
pub fn get_config() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory.");
    let config_dir = base_path.join("settings");

    let settings = config::Config::builder()
        .add_source(config::File::from(config_dir.join("config")))
        .add_source(config::Environment::with_prefix("NLPF").separator("__"))
        .build()?;

    settings.try_deserialize()
}

#[derive(Deserialize)]
pub struct S3Settings {
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_region: String,
    pub s3_endpoint: Option<String>,
}

impl S3Settings {
    /// Get a S3 client from the settings.
    pub async fn get_s3_client(self) -> aws_sdk_s3::Client {
        let creds = Credentials::new(
            self.aws_access_key_id,
            self.aws_secret_access_key,
            None,
            None,
            "credentials",
        );

        let sdk_config = aws_config::from_env()
            .credentials_provider(creds)
            .region(Region::new(self.aws_region))
            .load()
            .await;

        let s3_config = match self.s3_endpoint {
            Some(endpoint) => {
                let endpoint = Endpoint::immutable(endpoint.parse().unwrap());
                aws_sdk_s3::config::Builder::from(&sdk_config)
                    .endpoint_resolver(endpoint)
                    .build()
            },
            None => aws_sdk_s3::config::Builder::from(&sdk_config).build(),
        };

        aws_sdk_s3::Client::from_conf(s3_config)
    }
}

#[derive(Deserialize)]
pub struct Neo4jSettings {
    pub uri: String,
    pub username: String,
    pub password: String,
}

impl Neo4jSettings {
    /// Get a pooled connection to the Neo4j database.
    pub async fn get_connection(&self) -> Result<Graph, neo4rs::Error> {
        let config = neo4rs::config()
            .uri(&self.uri)
            .user(&self.username)
            .password(&self.password)
            .build()?;

        Graph::connect(config).await
    }
}
