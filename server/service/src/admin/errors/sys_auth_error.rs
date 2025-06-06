#![allow(unused_imports)]

use server_core::web::error::{ApiError, AppError};
use thiserror::Error;
use crate::admin::errors::{CommonError, impl_from_common_error, impl_from_db_error};
use sea_orm::DbErr;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("User not found")]
    UserNotFound,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Failed to send event: {0}")]
    SendError(#[from] tokio::sync::mpsc::error::SendError<Box<dyn std::any::Any + Send>>),

    #[error("Failed to handle login event: {0}")]
    LoginHandlerError(String),

    #[error("Failed to generate JWT token: {0}")]
    JwtGenerationFailed(String),

    #[error("Failed to validate JWT token")]
    JwtValidationFailed,

    #[error("Failed to refresh JWT token")]
    JwtRefreshFailed,

    #[error("Database operation failed: {0}")]
    DatabaseOperationFailed(String),
}

impl ApiError for AuthError {
    fn code(&self) -> u16 {
        match self {
            AuthError::UserNotFound => 9001,
            AuthError::InvalidCredentials => 9002,
            AuthError::AuthenticationFailed(_) => 9003,
            AuthError::SendError(_) => 9004,
            AuthError::LoginHandlerError(_) => 9005,
            AuthError::JwtGenerationFailed(_) => 9006,
            AuthError::JwtValidationFailed => 9007,
            AuthError::JwtRefreshFailed => 9008,
            AuthError::DatabaseOperationFailed(_) => 9009,
        }
    }

    fn message(&self) -> String {
        format!("{}", self)
    }
}

impl From<AuthError> for AppError {
    fn from(err: AuthError) -> Self {
        AppError {
            code: err.code(),
            message: err.message(),
        }
    }
}

// Helper methods for creating specific error types
impl AuthError {
    pub fn database_error(msg: String) -> Self {
        Self::DatabaseOperationFailed(msg)
    }

    pub fn authentication_error(msg: String) -> Self {
        Self::AuthenticationFailed(msg)
    }

    pub fn authorization_error(msg: String) -> Self {
        Self::AuthenticationFailed(msg)
    }

    pub fn not_found_error(_msg: String) -> Self {
        Self::UserNotFound
    }

    pub fn validation_error(msg: String) -> Self {
        Self::AuthenticationFailed(msg)
    }

    pub fn internal_error(msg: String) -> Self {
        Self::DatabaseOperationFailed(msg)
    }
}

// Implement From<CommonError> for AuthError
impl_from_common_error!(AuthError);

// Implement From<DbErr> for AuthError
impl_from_db_error!(AuthError); 