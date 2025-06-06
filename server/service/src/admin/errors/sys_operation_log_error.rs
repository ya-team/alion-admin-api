/*! 操作日志错误模块
 * 
 * 该模块定义了与系统操作日志相关的错误类型。
 * 包括日志的创建、查询、删除等操作相关的错误。
 * 
 * 错误类型
 * --------
 * OperationLogError 定义了操作日志相关的所有错误情况，包括：
 * - 日志不存在
 * - 日志创建失败
 * - 日志查询失败
 * - 日志删除失败
 * - 数据库操作失败
 * 
 * 错误代码
 * --------
 * - 4001: 日志不存在
 * - 4002: 日志创建失败
 * - 4003: 日志查询失败
 * - 4004: 日志删除失败
 * - 4005: 数据库操作失败
 * 
 * 使用示例
 * --------
 * /* 创建日志不存在错误
 *  * let error = OperationLogError::LogNotFound;
 *  */
 * 
 * /* 处理数据库错误
 *  * let db_error = OperationLogError::database_error("Failed to save log".to_string());
 *  */
 * 
 * /* 处理日志创建错误
 *  * let create_error = OperationLogError::LogCreationFailed("Invalid log data".to_string());
 *  */
 */

#![allow(unused_imports)]

use server_core::web::error::{ApiError, AppError};
use thiserror::Error;
use crate::admin::errors::{CommonError, impl_from_common_error, impl_from_db_error};
use sea_orm::DbErr;

#[derive(Debug, Error)]
pub enum OperationLogError {
    #[error("Operation log not found")]
    LogNotFound,

    #[error("Failed to create operation log")]
    CreateFailed,

    #[error("Failed to handle operation log event")]
    EventHandleFailed,

    #[error("Invalid operation log data")]
    InvalidData,
}

impl ApiError for OperationLogError {
    fn code(&self) -> u16 {
        match self {
            OperationLogError::LogNotFound => 7001,
            OperationLogError::CreateFailed => 7002,
            OperationLogError::EventHandleFailed => 7003,
            OperationLogError::InvalidData => 7004,
        }
    }

    fn message(&self) -> String {
        format!("{}", self)
    }
}

impl From<OperationLogError> for AppError {
    fn from(err: OperationLogError) -> Self {
        AppError {
            code: err.code(),
            message: err.message(),
        }
    }
} 