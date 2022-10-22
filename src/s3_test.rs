use axum::Extension;

use crate::SharedState;

pub async fn test_s3(Extension(state): Extension<SharedState>) {
    let resp = state
        .s3_client
        .list_buckets()
        .send()
        .await
        .expect("failed to list buckets");
    for bucket in resp.buckets.unwrap() {
        println!("bucket: {}", bucket.name.unwrap());
    }
}
