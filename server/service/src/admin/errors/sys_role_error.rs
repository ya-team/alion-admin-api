#![allow(unused_imports, unused_variables)]

use server_core::web::error::{ApiError, AppError};
use thiserror::Error;
use crate::admin::errors::{CommonError, impl_from_common_error};
use sea_orm::DbErr;

#[derive(Debug, Error)]
pub enum RoleError {
    #[error("Role not found")]
    RoleNotFound,

    #[error("Duplicate role code")]
    DuplicateRoleCode,

    #[error("Role has children, cannot delete")]
    HasChildren,

    #[error("Role is in use by users, cannot delete")]
    InUse,

    #[error("Database operation failed: {0}")]
    DatabaseOperationFailed(String),
}

impl ApiError for RoleError {
    fn code(&self) -> u16 {
        match self {
            RoleError::RoleNotFound => 4001,
            RoleError::DuplicateRoleCode => 4002,
            RoleError::HasChildren => 4003,
            RoleError::InUse => 4004,
            RoleError::DatabaseOperationFailed(_) => 4005,
        }
    }

    fn message(&self) -> String {
        self.to_string()
    }
}

impl From<RoleError> for AppError {
    fn from(err: RoleError) -> Self {
        AppError {
            code: err.code(),
            message: err.message(),
        }
    }
}

impl From<DbErr> for RoleError {
    fn from(err: DbErr) -> Self {
        RoleError::DatabaseOperationFailed(err.to_string())
    }
}

impl RoleError {
    pub fn database_error(msg: String) -> Self {
        RoleError::DatabaseOperationFailed(msg)
    }

    pub fn authentication_error(msg: String) -> Self {
        RoleError::DatabaseOperationFailed(msg)
    }

    pub fn authorization_error(msg: String) -> Self {
        RoleError::DatabaseOperationFailed(msg)
    }

    pub fn not_found_error(_msg: String) -> Self {
        RoleError::RoleNotFound
    }

    pub fn validation_error(_msg: String) -> Self {
        RoleError::DuplicateRoleCode
    }

    pub fn internal_error(msg: String) -> Self {
        RoleError::DatabaseOperationFailed(msg)
    }
}

// Implement From<CommonError> for RoleError
impl_from_common_error!(RoleError);
