use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("{0} Not Found")]
    NotFound(String),

    #[error("Unable to connect to the cache. ")]
    RedisError(#[from] redis::RedisError),
    #[error("Database Error")]
    SqlxError(#[from] sqlx::Error),
    #[error("Validation Error: {0}")]
    ValidationError(String),
}

impl From<AppError> for Status {
    fn from(err: AppError) -> Self {
        match err {
            AppError::SqlxError(_) => Status::unavailable("database error"),
            AppError::RedisError(_) => Status::unavailable("redis error"),
            AppError::Internal(msg) => Status::internal(msg),
            AppError::NotFound(msg) => Status::not_found(msg),
            AppError::ValidationError(msg) => Status::invalid_argument(msg),
        }
    }
}
