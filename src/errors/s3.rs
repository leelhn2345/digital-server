use aws_sdk_s3::presigning::PresigningConfigError;

#[derive(thiserror::Error, Debug)]
pub enum S3Error {
    #[error(transparent)]
    Presign(#[from] PresigningConfigError),

    #[error("{0}")]
    SdkError(String),
}
