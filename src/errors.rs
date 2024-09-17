use axum::{http::StatusCode, response::IntoResponse, Json};
use s3::S3Error;
use serde::Serialize;

pub mod s3;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    S3(#[from] S3Error),

    #[error(transparent)]
    Postgres(#[from] sqlx::Error),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status_code, msg) = match self {
            Self::Postgres(e) => {
                tracing::error!("{e:#?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
            ),
        };
        (status_code, Json(ErrorResponse { message: msg })).into_response()
    }
}
