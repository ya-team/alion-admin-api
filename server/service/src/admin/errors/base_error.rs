#![allow(unused_imports)]

use std::fmt;
use thiserror::Error;
use server_core::web::error::ApiError;

/// Base trait for all service errors
pub trait ServiceError: fmt::Display + fmt::Debug + Send + Sync {
    /// Get the error code for this error
    fn code(&self) -> i32;
    
    /// Get the error message
    fn message(&self) -> String {
        self.to_string()
    }
}

/// Common error variants that can be used across different services
#[derive(Error, Debug)]
pub enum CommonError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Authorization failed: {0}")]
    AuthorizationError(String),

    #[error("Resource not found: {0}")]
    NotFoundError(String),

    #[error("Invalid input: {0}")]
    ValidationError(String),

    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl ServiceError for CommonError {
    fn code(&self) -> i32 {
        match self {
            CommonError::DatabaseError(_) => 5001,
            CommonError::AuthenticationError(_) => 4001,
            CommonError::AuthorizationError(_) => 4003,
            CommonError::NotFoundError(_) => 4004,
            CommonError::ValidationError(_) => 4000,
            CommonError::InternalError(_) => 5000,
        }
    }
}

/// Helper macro to implement From<CommonError> for service-specific errors
#[macro_export]
macro_rules! impl_from_common_error {
    ($error_type:ty) => {
        impl From<crate::admin::errors::CommonError> for $error_type {
            fn from(error: crate::admin::errors::CommonError) -> Self {
                match error {
                    crate::admin::errors::CommonError::DatabaseError(msg) => Self::database_error(msg),
                    crate::admin::errors::CommonError::AuthenticationError(msg) => Self::authentication_error(msg),
                    crate::admin::errors::CommonError::AuthorizationError(msg) => Self::authorization_error(msg),
                    crate::admin::errors::CommonError::NotFoundError(msg) => Self::not_found_error(msg),
                    crate::admin::errors::CommonError::ValidationError(msg) => Self::validation_error(msg),
                    crate::admin::errors::CommonError::InternalError(msg) => Self::internal_error(msg),
                }
            }
        }
    };
}

/// Helper macro to implement From<sea_orm::DbErr> for service-specific errors
#[macro_export]
macro_rules! impl_from_db_error {
    ($error_type:ty) => {
        impl From<sea_orm::DbErr> for $error_type {
            fn from(error: sea_orm::DbErr) -> Self {
                Self::from(crate::admin::errors::CommonError::DatabaseError(error.to_string()))
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_error_codes() {
        assert_eq!(CommonError::DatabaseError("test".into()).code(), 5001);
        assert_eq!(CommonError::AuthenticationError("test".into()).code(), 4001);
        assert_eq!(CommonError::AuthorizationError("test".into()).code(), 4003);
        assert_eq!(CommonError::NotFoundError("test".into()).code(), 4004);
        assert_eq!(CommonError::ValidationError("test".into()).code(), 4000);
        assert_eq!(CommonError::InternalError("test".into()).code(), 5000);
    }
} 