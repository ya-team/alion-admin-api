/*! 访问密钥错误模块
 * 
 * 该模块定义了与访问密钥（Access Key）相关的错误类型。
 * 访问密钥用于API访问认证，支持密钥的创建、验证、过期和撤销等功能。
 * 
 * 错误类型
 * --------
 * AccessKeyError 定义了访问密钥相关的所有错误情况，包括：
 * - 密钥不存在
 * - 密钥无效
 * - 密钥过期
 * - 密钥被撤销
 * - 密钥使用限制
 * - 数据库操作错误
 * 
 * 错误代码
 * --------
 * - 2001: 访问密钥不存在
 * - 2002: 无效的访问密钥
 * - 2003: 访问密钥已过期
 * - 2004: 访问密钥已撤销
 * - 2005: 访问密钥使用限制
 * - 2006: 数据库操作失败
 * 
 * 使用示例
 * --------
 * /* 创建访问密钥错误
 *  * let error = AccessKeyError::AccessKeyNotFound;
 *  */
 * 
 * /* 处理数据库错误
 *  * let db_error = AccessKeyError::database_error("Failed to save access key".to_string());
 *  */
 * 
 * /* 验证错误
 *  * let validation_error = AccessKeyError::validation_error("Invalid key format".to_string());
 *  */
 */

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
