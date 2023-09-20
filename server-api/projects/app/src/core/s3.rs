use std::{borrow::Cow, collections::HashMap};

use s3::{
    creds::{Credentials, Rfc3339OffsetDateTime},
    post_policy::PostPolicyExpiration,
    Bucket, BucketConfiguration, PostPolicy, PostPolicyField, PostPolicyValue, Region,
};
use serde::Serialize;
use time::{Duration, OffsetDateTime};

use super::{config::StorageConfig, errors::AppResult};

#[derive(Clone)]
pub struct S3Client {
    bucket: Bucket,
    bucket_name: String,
    credentials: Credentials,
}

/// Maximum file size for uploads using presigned post URLs, allow up to 500MB
const MAX_FILE_SIZE: u32 = 1024 * 1024 * 500;

#[derive(Debug, Serialize)]
pub enum UploadMode {
    /// Upload file using presigned POST URL
    SingleUpload,
    /// Upload file using presigned PUT URL
    MultipartUpload,
}

#[derive(Debug, Serialize)]
pub struct PresignedUrl {
    upload_mode: UploadMode,
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<HashMap<String, String>>,
}

#[tracing::instrument("initialize s3 client", skip(config))]
pub async fn init_client(config: &StorageConfig) -> AppResult<S3Client> {
    let region = Region::Custom {
        region: "eu-central-1".to_owned(),
        endpoint: config.endpoint.clone(),
    };

    let credentials = Credentials::new(
        Some(&config.access_key),
        Some(&config.secret_key),
        None,
        None,
        None,
    )?;

    let bucket = Bucket::new(&config.name, region, credentials.clone())?.with_path_style();

    Ok(S3Client {
        bucket,
        bucket_name: config.name.clone(),
        credentials,
    })
}

impl S3Client {
    /// Prepare the bucket to use for storing files
    /// This will create the bucket if it does not exist
    #[tracing::instrument("prepare bucket", skip(self))]
    pub async fn prepare_bucket(&mut self) -> AppResult<()> {
        // Check if bucket exists and create it if it does not
        if !self.bucket.exists().await? {
            // Create the bucket
            self.bucket = Bucket::create_with_path_style(
                &self.bucket.name,
                self.bucket.region.clone(),
                self.credentials.clone(),
                BucketConfiguration::default(),
            )
            .await?
            .bucket;
        }

        // Check if bucket versioning is enabled
        if !self.bucket.get_bucket_versioning().await? {
            // Enable bucket versioning
            self.bucket.enable_versioning(true).await?;
        }

        Ok(())
    }

    // Create PUT presigned URL
    #[tracing::instrument("create put presigned url", skip(self, key_prefix))]
    pub async fn create_presigned_url(
        &self,
        filename: &String,
        file_size: usize,
        key_prefix: &str,
    ) -> AppResult<PresignedUrl> {
        let duration = Duration::hours(4);
        let path = format!("{key_prefix}{filename}");

        // Check if file size is within limits
        if file_size < MAX_FILE_SIZE as usize {
            // If below, return a presigned POST URL for a single upload
            let expiration_at = OffsetDateTime::now_utc() + duration;
            let expriation_at = Rfc3339OffsetDateTime::from(expiration_at);

            let policy = PostPolicy::new(PostPolicyExpiration::ExpiresAt(expriation_at))
                .condition(
                    PostPolicyField::ContentLengthRange,
                    PostPolicyValue::Range(0, MAX_FILE_SIZE),
                )?
                .condition(
                    PostPolicyField::Key,
                    PostPolicyValue::Exact(Cow::from(path)),
                )?
                .condition(
                    // Allow user to upload any file type
                    PostPolicyField::Acl,
                    PostPolicyValue::Exact(Cow::from("write")),
                )?;

            let presigned_post = self.bucket.presign_post(policy)?;

            Ok(PresignedUrl {
                upload_mode: UploadMode::SingleUpload,
                url: presigned_post.url,
                fields: Some(presigned_post.fields),
            })
        } else {
            // Otherwise, return a presigned PUT URL for a multipart upload
            let expriation_at = duration.whole_seconds().abs() as u32;

            let presigned_put_url = self.bucket.presign_put(path, expriation_at, None)?;

            Ok(PresignedUrl {
                upload_mode: UploadMode::MultipartUpload,
                url: presigned_put_url,
                fields: None,
            })
        }
    }
}
