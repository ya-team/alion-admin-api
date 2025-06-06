#![allow(unused_imports)]

use server_core::web::error::{ApiError, AppError};
use thiserror::Error;
use crate::admin::errors::{CommonError, impl_from_common_error, impl_from_db_error};
use sea_orm::DbErr;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Domain not found")]
    DomainNotFound,

    #[error("Invalid domain code")]
    InvalidDomainCode,

    #[error("Domain already exists")]
    DomainAlreadyExists,

    #[error("Domain is disabled")]
    DomainDisabled,

    #[error("Cannot modify or delete built-in domain")]
    BuiltInDomain,

    #[error("Duplicate domain code")]
    DuplicateCode,

    #[error("Duplicate domain name")]
    DuplicateName,

    #[error("Domain operation failed: {0}")]
    DomainOperationFailed(String),

    #[error("Database operation failed: {0}")]
    DatabaseOperationFailed(String),
}

impl ApiError for DomainError {
    fn code(&self) -> u16 {
        match self {
            DomainError::DomainNotFound => 3001,
            DomainError::InvalidDomainCode => 3002,
            DomainError::DomainAlreadyExists => 3003,
            DomainError::DomainDisabled => 3004,
            DomainError::BuiltInDomain => 3005,
            DomainError::DuplicateCode => 3006,
            DomainError::DuplicateName => 3007,
            DomainError::DomainOperationFailed(_) => 3008,
            DomainError::DatabaseOperationFailed(_) => 3009,
        }
    }

    fn message(&self) -> String {
        format!("{}", self)
    }
}

impl From<DomainError> for AppError {
    fn from(err: DomainError) -> Self {
        AppError {
            code: err.code() as u16,
            message: err.message(),
        }
    }
}

// Helper methods for creating specific error types
impl DomainError {
    pub fn database_error(msg: String) -> Self {
        Self::DatabaseOperationFailed(msg)
    }

    pub fn authentication_error(_msg: String) -> Self {
        Self::DomainOperationFailed("Authentication failed".to_string())
    }

    pub fn authorization_error(_msg: String) -> Self {
        Self::DomainOperationFailed("Authorization failed".to_string())
    }

    pub fn not_found_error(_msg: String) -> Self {
        Self::DomainNotFound
    }

    pub fn validation_error(_msg: String) -> Self {
        Self::InvalidDomainCode
    }

    pub fn internal_error(msg: String) -> Self {
        Self::DatabaseOperationFailed(msg)
    }
}

// Implement From<CommonError> for DomainError
impl_from_common_error!(DomainError);

// Implement From<DbErr> for DomainError
impl_from_db_error!(DomainError);
