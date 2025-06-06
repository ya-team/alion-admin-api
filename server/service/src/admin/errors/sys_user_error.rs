/*! 用户错误模块
 * 
 * 该模块定义了与系统用户相关的错误类型。
 * 包括用户的创建、修改、删除等操作相关的错误。
 * 
 * 错误类型
 * --------
 * UserError 定义了用户相关的所有错误情况，包括：
 * - 用户不存在
 * - 用户已存在
 * - 用户被禁用
 * - 内置用户不可修改
 * - 用户名重复
 * - 邮箱重复
 * - 手机号重复
 * - 用户操作失败
 * - 数据库操作失败
 * 
 * 错误代码
 * --------
 * - 7001: 用户不存在
 * - 7002: 用户已存在
 * - 7003: 用户被禁用
 * - 7004: 内置用户不可修改
 * - 7005: 用户名重复
 * - 7006: 邮箱重复
 * - 7007: 手机号重复
 * - 7008: 用户操作失败
 * - 7009: 数据库操作失败
 * 
 * 使用示例
 * --------
 * /* 创建用户不存在错误
 *  * let error = UserError::UserNotFound;
 *  */
 * 
 * /* 处理重复用户名错误
 *  * let error = UserError::DuplicateUsername;
 *  */
 * 
 * /* 处理数据库错误
 *  * let db_error = UserError::database_error("Failed to save user".to_string());
 *  */
 */

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
