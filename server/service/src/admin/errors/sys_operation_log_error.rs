use server_core::web::error::{ApiError, AppError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OperationLogError {
    #[error("Operation log not found")]
    LogNotFound,

    #[error("Failed to create operation log")]
    CreateFailed,

    #[error("Failed to handle operation log event")]
    EventHandleFailed,

    #[error("Invalid operation log data")]
    InvalidData,
}

impl ApiError for OperationLogError {
    fn code(&self) -> u16 {
        match self {
            OperationLogError::LogNotFound => 7001,
            OperationLogError::CreateFailed => 7002,
            OperationLogError::EventHandleFailed => 7003,
            OperationLogError::InvalidData => 7004,
        }
    }

    fn message(&self) -> String {
        format!("{}", self)
    }
}

impl From<OperationLogError> for AppError {
    fn from(err: OperationLogError) -> Self {
        AppError {
            code: err.code(),
            message: err.message(),
        }
    }
} 