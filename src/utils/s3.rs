use aws_sdk_s3::{presigning::config::PresigningConfig, types::ByteStream};
use axum::body::Bytes;
use uuid::Uuid;

use crate::SharedState;

pub async fn upload_bytes_to_s3(
    bytes: Bytes,
    content_type: String,
    bucket: String,
    key: String,
    state: SharedState,
) -> Result<String, String> {
    let key = format!("{}-{}", Uuid::new_v4(), key);

    // Put the object
    state
        .s3_client
        .put_object()
        .body(ByteStream::from(bytes))
        .bucket(bucket.clone())
        .key(key.clone())
        .content_type(content_type)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    // Generate pre-signing config
    let presigning_config = PresigningConfig::builder()
        .expires_in(std::time::Duration::from_secs(60 * 60 * 24 * 7))
        .build()
        .unwrap();

    // Generate pre-signed URL
    let presigned_url = state
        .s3_client
        .get_object()
        .bucket(bucket)
        .key(key)
        .presigned(presigning_config)
        .await
        .map_err(|err| err.to_string())?;

    let uri_without_signing = format!(
        "{}://{}{}",
        presigned_url.uri().scheme().unwrap(),
        presigned_url.uri().authority().unwrap(),
        presigned_url.uri().path()
    );

    Ok(uri_without_signing)
}
