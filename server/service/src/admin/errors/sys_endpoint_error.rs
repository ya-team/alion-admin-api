use server_core::web::error::AppError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EndpointError {
    #[error("Endpoint not found: {0}")]
    EndpointNotFound(String),
    #[error("One or more endpoints not found: {0:?}")]
    EndpointsNotFound(Vec<i32>),
    #[error("Database error: {0}")]
    DatabaseError(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    #[error("Authorization error: {0}")]
    AuthorizationError(String),
    #[error("Not found error: {0}")]
    NotFoundError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<EndpointError> for AppError {
    fn from(error: EndpointError) -> Self {
        match error {
            EndpointError::EndpointNotFound(_) => AppError { code: 404, message: error.to_string() },
            EndpointError::EndpointsNotFound(_) => AppError { code: 404, message: error.to_string() },
            EndpointError::DatabaseError(e) => AppError { code: 500, message: e.to_string() },
            EndpointError::AuthenticationError(msg) => AppError { code: 401, message: msg },
            EndpointError::AuthorizationError(msg) => AppError { code: 403, message: msg },
            EndpointError::NotFoundError(msg) => AppError { code: 404, message: msg },
            EndpointError::ValidationError(msg) => AppError { code: 400, message: msg },
            EndpointError::InternalError(msg) => AppError { code: 500, message: msg },
        }
    }
}

impl From<sea_orm::DbErr> for EndpointError {
    fn from(error: sea_orm::DbErr) -> Self {
        EndpointError::DatabaseError(Box::new(error))
    }
} 