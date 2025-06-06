#![allow(unused_imports)]

use server_core::web::error::{ApiError, AppError};
use thiserror::Error;
use crate::admin::errors::{CommonError, impl_from_common_error, impl_from_db_error};
use sea_orm::DbErr;

#[derive(Error, Debug)]
pub enum AccessKeyError {
    #[error("Access key not found")]
    AccessKeyNotFound,

    #[error("Invalid access key")]
    InvalidAccessKey,

    #[error("Access key expired")]
    AccessKeyExpired,

    #[error("Access key revoked")]
    AccessKeyRevoked,

    #[error("Access key limit exceeded")]
    AccessKeyLimitExceeded,

    #[error("Database operation failed: {0}")]
    DatabaseOperationFailed(String),
}

impl ApiError for AccessKeyError {
    fn code(&self) -> u16 {
        match self {
            AccessKeyError::AccessKeyNotFound => 2001,
            AccessKeyError::InvalidAccessKey => 2002,
            AccessKeyError::AccessKeyExpired => 2003,
            AccessKeyError::AccessKeyRevoked => 2004,
            AccessKeyError::AccessKeyLimitExceeded => 2005,
            AccessKeyError::DatabaseOperationFailed(_) => 2006,
        }
    }

    fn message(&self) -> String {
        format!("{}", self)
    }
}

impl From<AccessKeyError> for AppError {
    fn from(err: AccessKeyError) -> Self {
        AppError {
            code: err.code() as u16,
            message: err.message(),
        }
    }
}

// Helper methods for creating specific error types
impl AccessKeyError {
    pub fn database_error(msg: String) -> Self {
        Self::DatabaseOperationFailed(msg)
    }

    pub fn authentication_error(_msg: String) -> Self {
        Self::InvalidAccessKey
    }

    pub fn authorization_error(_msg: String) -> Self {
        Self::AccessKeyRevoked
    }

    pub fn not_found_error(_msg: String) -> Self {
        Self::AccessKeyNotFound
    }

    pub fn validation_error(_msg: String) -> Self {
        Self::InvalidAccessKey
    }

    pub fn internal_error(msg: String) -> Self {
        Self::DatabaseOperationFailed(msg)
    }
}

// Implement From<CommonError> for AccessKeyError
impl_from_common_error!(AccessKeyError);

// Implement From<DbErr> for AccessKeyError
impl_from_db_error!(AccessKeyError);
