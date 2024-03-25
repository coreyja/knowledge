use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::operation::put_object::PutObject;
use aws_sdk_s3::presigning::PresigningConfig;
use axum::extract::State;
use axum::{Json, Router};
use miette::IntoDiagnostic;

use crate::AppState;

use aws_sdk_s3::Client;
use std::time::Duration;

async fn generate_presigned_url(
    State(state): State<AppState>,
) -> Result<Json<String>, Json<String>> {
    let credentials = Credentials::new(
        state.s3_config.access_key_id,
        state.s3_config.secret_access_key,
        None,
        None,
        "programmatic",
    );
    let config = aws_config::defaults(BehaviorVersion::v2023_11_09())
        .endpoint_url(state.s3_config.endpoint_url)
        .region(Region::new(state.s3_config.region))
        .credentials_provider(credentials)
        .load()
        .await;
    let client = Client::new(&config);

    let presigning_config = PresigningConfig::builder()
        .expires_in(Duration::from_secs(15 * 60)) // Presigned URL expires in 15 minutes
        .build()
        .unwrap();

    let request = client
        .put_object()
        .bucket("your-bucket-name")
        .key("your-object-key")
        .presigned(presigning_config)
        .await;

    match request {
        Ok(presigned_request) => {
            let url = presigned_request.uri().to_string();
            Ok(Json(url))
        }
        Err(e) => Err(Json(e.to_string())),
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
}
