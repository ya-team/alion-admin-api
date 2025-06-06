/*! 登录日志错误模块
 * 
 * 该模块定义了与系统登录日志相关的错误类型。
 * 包括日志的创建、查询、删除等操作相关的错误。
 * 
 * 错误类型
 * --------
 * LoginLogError 定义了登录日志相关的所有错误情况，包括：
 * - 日志不存在
 * - 日志创建失败
 * - 日志查询失败
 * - 日志删除失败
 * - 数据库操作失败
 * 
 * 错误代码
 * --------
 * - 8001: 日志不存在
 * - 8002: 日志创建失败
 * - 8003: 日志查询失败
 * - 8004: 日志删除失败
 * - 8005: 数据库操作失败
 * 
 * 使用示例
 * --------
 * /* 创建日志不存在错误
 *  * let error = LoginLogError::LogNotFound;
 *  */
 * 
 * /* 处理数据库错误
 *  * let db_error = LoginLogError::database_error("Failed to save log".to_string());
 *  */
 * 
 * /* 处理日志创建错误
 *  * let create_error = LoginLogError::LogCreationFailed("Invalid log data".to_string());
 *  */
 */

#![allow(unused_imports)]

use server_core::web::error::{ApiError, AppError};
use thiserror::Error;
use crate::admin::errors::{CommonError, impl_from_common_error, impl_from_db_error};
use sea_orm::DbErr;

#[derive(Debug, Error)]
pub enum LoginLogError {
    #[error("Login log not found")]
    LogNotFound,

    #[error("Failed to create login log")]
    CreateFailed,

    #[error("Failed to handle login log event")]
    EventHandleFailed,

    #[error("Invalid login log data")]
    InvalidData,
}

impl ApiError for LoginLogError {
    fn code(&self) -> u16 {
        match self {
            LoginLogError::LogNotFound => 8001,
            LoginLogError::CreateFailed => 8002,
            LoginLogError::EventHandleFailed => 8003,
            LoginLogError::InvalidData => 8004,
        }
    }

    fn message(&self) -> String {
        format!("{}", self)
    }
}

impl From<LoginLogError> for AppError {
    fn from(err: LoginLogError) -> Self {
        AppError {
            code: err.code(),
            message: err.message(),
        }
    }
} 