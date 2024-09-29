use async_openai::error::OpenAIError;

/// Possible errors arising from chat module
#[derive(thiserror::Error, Debug)]
pub enum ChatError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    OpenAI(#[from] OpenAIError),

    #[error("{0}")]
    Unknown(String),
}
