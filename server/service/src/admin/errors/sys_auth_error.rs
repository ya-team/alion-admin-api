/*! 认证错误模块
 * 
 * 该模块定义了与用户认证相关的错误类型。
 * 包括登录认证、JWT令牌生成和验证、事件处理等相关的错误。
 * 
 * 错误类型
 * --------
 * AuthError 定义了认证相关的所有错误情况，包括：
 * - 用户不存在
 * - 无效的凭证
 * - 认证失败
 * - 事件发送失败
 * - 登录事件处理失败
 * - JWT令牌生成失败
 * - JWT令牌验证失败
 * - JWT令牌刷新失败
 * - 数据库操作失败
 * 
 * 错误代码
 * --------
 * - 9001: 用户不存在
 * - 9002: 无效的凭证
 * - 9003: 认证失败
 * - 9004: 事件发送失败
 * - 9005: 登录事件处理失败
 * - 9006: JWT令牌生成失败
 * - 9007: JWT令牌验证失败
 * - 9008: JWT令牌刷新失败
 * - 9009: 数据库操作失败
 * 
 * 使用示例
 * --------
 * /* 创建认证错误
 *  * let error = AuthError::UserNotFound;
 *  */
 * 
 * /* 处理JWT错误
 *  * let jwt_error = AuthError::JwtGenerationFailed("Invalid token format".to_string());
 *  */
 * 
 * /* 处理数据库错误
 *  * let db_error = AuthError::database_error("Failed to save user".to_string());
 *  */
 */

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