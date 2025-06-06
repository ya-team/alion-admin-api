/*! 角色错误模块
 * 
 * 该模块定义了与系统角色相关的错误类型。
 * 包括角色的创建、修改、删除等操作相关的错误。
 * 
 * 错误类型
 * --------
 * RoleError 定义了角色相关的所有错误情况，包括：
 * - 角色不存在
 * - 角色已存在
 * - 角色被禁用
 * - 内置角色不可修改
 * - 角色代码重复
 * - 角色名称重复
 * - 角色操作失败
 * - 数据库操作失败
 * 
 * 错误代码
 * --------
 * - 5001: 角色不存在
 * - 5002: 角色已存在
 * - 5003: 角色被禁用
 * - 5004: 内置角色不可修改
 * - 5005: 角色代码重复
 * - 5006: 角色名称重复
 * - 5007: 角色操作失败
 * - 5008: 数据库操作失败
 * 
 * 使用示例
 * --------
 * /* 创建角色不存在错误
 *  * let error = RoleError::RoleNotFound;
 *  */
 * 
 * /* 处理重复角色代码错误
 *  * let error = RoleError::DuplicateCode;
 *  */
 * 
 * /* 处理数据库错误
 *  * let db_error = RoleError::database_error("Failed to save role".to_string());
 *  */
 */

#![allow(unused_imports, unused_variables)]

use server_core::web::error::{ApiError, AppError};
use thiserror::Error;
use crate::admin::errors::{CommonError, impl_from_common_error, impl_from_db_error};
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
