/*! 授权错误模块
 * 
 * 该模块定义了与用户授权相关的错误类型。
 * 包括域、角色、权限、路由等资源的访问控制相关的错误。
 * 
 * 错误类型
 * --------
 * AuthorizationError 定义了授权相关的所有错误情况，包括：
 * - 域不存在
 * - 角色不存在
 * - 权限不存在
 * - 路由不存在
 * - 用户不存在
 * - 权限被拒绝
 * - 数据库错误
 * - 认证错误
 * - 授权错误
 * - 资源不存在
 * - 验证错误
 * - 内部错误
 * 
 * 错误代码
 * --------
 * - 3001: 域不存在
 * - 3002: 角色不存在
 * - 3003: 权限不存在
 * - 3004: 路由不存在
 * - 3005: 用户不存在
 * - 3006: 权限被拒绝
 * - 3007: 数据库操作失败
 * - 3008: 认证失败
 * - 3009: 授权失败
 * - 3010: 资源不存在
 * - 3011: 验证失败
 * - 3012: 内部错误
 * 
 * 使用示例
 * --------
 * /* 创建域不存在错误
 *  * let error = AuthorizationError::domain_not_found(
 *  *     "example.com".to_string(),
 *  *     "DOMAIN_001".to_string()
 *  * );
 *  */
 * 
 * /* 创建权限被拒绝错误
 *  * let error = AuthorizationError::permission_denied(
 *  *     "User does not have required permissions".to_string()
 *  * );
 *  */
 * 
 * /* 处理数据库错误
 *  * let db_error = AuthorizationError::database_error(
 *  *     "Failed to load permissions".to_string()
 *  * );
 *  */
 */

#![allow(unused_imports)]

use server_core::web::error::{ApiError, AppError};
use thiserror::Error;
use crate::admin::errors::{CommonError, impl_from_common_error, impl_from_db_error};
use sea_orm::DbErr;

// Error codes for authorization errors
pub const ERROR_DOMAIN_NOT_FOUND: u16 = 3001;
pub const ERROR_ROLE_NOT_FOUND: u16 = 3002;
pub const ERROR_PERMISSIONS_NOT_FOUND: u16 = 3003;
pub const ERROR_ROUTES_NOT_FOUND: u16 = 3004;
pub const ERROR_USERS_NOT_FOUND: u16 = 3005;
pub const ERROR_PERMISSION_DENIED: u16 = 3006;
pub const ERROR_DATABASE_OPERATION: u16 = 3007;
pub const ERROR_AUTHENTICATION: u16 = 3008;
pub const ERROR_AUTHORIZATION: u16 = 3009;
pub const ERROR_NOT_FOUND: u16 = 3010;
pub const ERROR_VALIDATION: u16 = 3011;
pub const ERROR_INTERNAL: u16 = 3012;

#[derive(Error, Debug)]
pub enum AuthorizationError {
    #[error("Domain not found: {domain} (code: {code})")]
    DomainNotFound { domain: String, code: String },

    #[error("Role not found: {role_id} (code: {code})")]
    RoleNotFound { role_id: String, code: String },

    #[error("Permissions not found: {missing_ids:?} (found: {found_ids:?})")]
    PermissionsNotFound { 
        missing_ids: Vec<String>,
        found_ids: Vec<String>,
    },

    #[error("Routes not found: {missing_ids:?} (found: {found_ids:?})")]
    RoutesNotFound { 
        missing_ids: Vec<i32>,
        found_ids: Vec<i32>,
    },

    #[error("Users not found: {missing_ids:?} (found: {found_ids:?})")]
    UsersNotFound { 
        missing_ids: Vec<String>,
        found_ids: Vec<String>,
    },

    #[error("Permission denied: {reason}")]
    PermissionDenied { reason: String },

    #[error("Database operation failed: {0}")]
    DatabaseError(#[from] Box<dyn std::error::Error + Send + Sync>),

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

impl ApiError for AuthorizationError {
    fn code(&self) -> u16 {
        match self {
            AuthorizationError::DomainNotFound { .. } => ERROR_DOMAIN_NOT_FOUND,
            AuthorizationError::RoleNotFound { .. } => ERROR_ROLE_NOT_FOUND,
            AuthorizationError::PermissionsNotFound { .. } => ERROR_PERMISSIONS_NOT_FOUND,
            AuthorizationError::RoutesNotFound { .. } => ERROR_ROUTES_NOT_FOUND,
            AuthorizationError::UsersNotFound { .. } => ERROR_USERS_NOT_FOUND,
            AuthorizationError::PermissionDenied { .. } => ERROR_PERMISSION_DENIED,
            AuthorizationError::DatabaseError(_) => ERROR_DATABASE_OPERATION,
            AuthorizationError::AuthenticationError(_) => ERROR_AUTHENTICATION,
            AuthorizationError::AuthorizationError(_) => ERROR_AUTHORIZATION,
            AuthorizationError::NotFoundError(_) => ERROR_NOT_FOUND,
            AuthorizationError::ValidationError(_) => ERROR_VALIDATION,
            AuthorizationError::InternalError(_) => ERROR_INTERNAL,
        }
    }

    fn message(&self) -> String {
        format!("{}", self)
    }
}

impl From<AuthorizationError> for AppError {
    fn from(err: AuthorizationError) -> Self {
        AppError {
            code: err.code(),
            message: err.message(),
        }
    }
}

// Helper methods for creating specific error types
impl AuthorizationError {
    pub fn domain_not_found(domain: String, code: String) -> Self {
        Self::DomainNotFound { domain, code }
    }

    pub fn role_not_found(role_id: String, code: String) -> Self {
        Self::RoleNotFound { role_id, code }
    }

    pub fn permissions_not_found(missing_ids: Vec<String>, found_ids: Vec<String>) -> Self {
        Self::PermissionsNotFound { missing_ids, found_ids }
    }

    pub fn routes_not_found(missing_ids: Vec<i32>, found_ids: Vec<i32>) -> Self {
        Self::RoutesNotFound { missing_ids, found_ids }
    }

    pub fn users_not_found(missing_ids: Vec<String>, found_ids: Vec<String>) -> Self {
        Self::UsersNotFound { missing_ids, found_ids }
    }

    pub fn permission_denied(reason: String) -> Self {
        Self::PermissionDenied { reason }
    }

    pub fn database_error(msg: String) -> Self {
        Self::DatabaseError(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            msg,
        )))
    }

    pub fn authentication_error(msg: String) -> Self {
        Self::AuthenticationError(msg)
    }

    pub fn authorization_error(msg: String) -> Self {
        Self::AuthorizationError(msg)
    }

    pub fn not_found_error(msg: String) -> Self {
        Self::NotFoundError(msg)
    }

    pub fn validation_error(msg: String) -> Self {
        Self::ValidationError(msg)
    }

    pub fn internal_error(msg: String) -> Self {
        Self::InternalError(msg)
    }
}

// Implement From<CommonError> for AuthorizationError
impl_from_common_error!(AuthorizationError);

// Implement From<DbErr> for AuthorizationError
impl_from_db_error!(AuthorizationError); 