use server_core::web::error::{ApiError, AppError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoginLogError {
    #[error("Login log not found")]
    LogNotFound,

    #[error("Failed to create login log")]
    CreateFailed,

    #[error("Failed to handle login log event")]
    EventHandleFailed,

    #[error("Invalid login log data")]
    InvalidData,
}

impl ApiError for LoginLogError {
    fn code(&self) -> u16 {
        match self {
            LoginLogError::LogNotFound => 8001,
            LoginLogError::CreateFailed => 8002,
            LoginLogError::EventHandleFailed => 8003,
            LoginLogError::InvalidData => 8004,
        }
    }

    fn message(&self) -> String {
        format!("{}", self)
    }
}

impl From<LoginLogError> for AppError {
    fn from(err: LoginLogError) -> Self {
        AppError {
            code: err.code(),
            message: err.message(),
        }
    }
} 