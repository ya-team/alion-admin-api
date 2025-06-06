#![allow(unused_imports)]

use server_core::web::error::{ApiError, AppError};
use thiserror::Error;
use crate::admin::errors::{CommonError, impl_from_common_error, impl_from_db_error};
use sea_orm::DbErr;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    UserNotFound,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Username already exists")]
    UsernameAlreadyExists,

    #[error("Invalid user status")]
    InvalidUserStatus,

    #[error("Database operation failed: {0}")]
    DatabaseOperationFailed(String),
}

impl ApiError for UserError {
    fn code(&self) -> u16 {
        match self {
            UserError::UserNotFound => 1001,
            UserError::InvalidCredentials => 1002,
            UserError::AuthenticationFailed(_) => 1003,
            UserError::UsernameAlreadyExists => 1004,
            UserError::InvalidUserStatus => 1005,
            UserError::DatabaseOperationFailed(_) => 1006,
        }
    }

    fn message(&self) -> String {
        format!("{}", self)
    }
}

impl From<UserError> for AppError {
    fn from(err: UserError) -> Self {
        AppError {
            code: err.code() as u16,
            message: err.message(),
        }
    }
}

// Helper methods for creating specific error types
impl UserError {
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

// Implement From<CommonError> for UserError
impl_from_common_error!(UserError);

// Implement From<DbErr> for UserError
impl_from_db_error!(UserError);
