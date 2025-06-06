/*! 菜单错误模块
 * 
 * 该模块定义了与系统菜单相关的错误类型。
 * 包括菜单的创建、修改、删除等操作相关的错误。
 * 
 * 错误类型
 * --------
 * MenuError 定义了菜单相关的所有错误情况，包括：
 * - 菜单不存在
 * - 菜单已存在
 * - 菜单被禁用
 * - 内置菜单不可修改
 * - 菜单代码重复
 * - 菜单名称重复
 * - 菜单操作失败
 * - 数据库操作失败
 * 
 * 错误代码
 * --------
 * - 6001: 菜单不存在
 * - 6002: 菜单已存在
 * - 6003: 菜单被禁用
 * - 6004: 内置菜单不可修改
 * - 6005: 菜单代码重复
 * - 6006: 菜单名称重复
 * - 6007: 菜单操作失败
 * - 6008: 数据库操作失败
 * 
 * 使用示例
 * --------
 * /* 创建菜单不存在错误
 *  * let error = MenuError::MenuNotFound;
 *  */
 * 
 * /* 处理重复菜单代码错误
 *  * let error = MenuError::DuplicateCode;
 *  */
 * 
 * /* 处理数据库错误
 *  * let db_error = MenuError::database_error("Failed to save menu".to_string());
 *  */
 */

#![allow(unused_imports)]

use server_core::web::error::{ApiError, AppError};
use thiserror::Error;
use crate::admin::errors::{CommonError, impl_from_common_error, impl_from_db_error};
use sea_orm::DbErr;

#[derive(Debug, Error)]
pub enum MenuError {
    #[error("Menu not found")]
    MenuNotFound,

    #[error("Duplicate route name")]
    DuplicateRouteName,

    #[error("Parent menu not found")]
    ParentMenuNotFound,

    #[error("Parent menu must be a directory")]
    ParentNotDirectory,

    #[error("Menu has children, cannot delete")]
    HasChildren,

    #[error("Menu is in use by roles, cannot delete")]
    InUse,

    #[error("Cannot move menu to its own submenu")]
    CircularReference,

    #[error("Database operation failed: {0}")]
    DatabaseOperationFailed(String),
}

impl ApiError for MenuError {
    fn code(&self) -> u16 {
        match self {
            MenuError::MenuNotFound => 5001,
            MenuError::DuplicateRouteName => 5002,
            MenuError::ParentMenuNotFound => 5003,
            MenuError::ParentNotDirectory => 5004,
            MenuError::HasChildren => 5005,
            MenuError::InUse => 5006,
            MenuError::CircularReference => 5007,
            MenuError::DatabaseOperationFailed(_) => 5008,
        }
    }

    fn message(&self) -> String {
        self.to_string()
    }
}

impl From<MenuError> for AppError {
    fn from(err: MenuError) -> Self {
        AppError {
            code: err.code(),
            message: err.message(),
        }
    }
}

impl From<DbErr> for MenuError {
    fn from(err: DbErr) -> Self {
        MenuError::DatabaseOperationFailed(err.to_string())
    }
}

impl MenuError {
    pub fn database_error(err: DbErr) -> Self {
        MenuError::DatabaseOperationFailed(err.to_string())
    }

    pub fn authentication_error(msg: String) -> Self {
        MenuError::DatabaseOperationFailed(msg)
    }

    pub fn authorization_error(msg: String) -> Self {
        MenuError::DatabaseOperationFailed(msg)
    }

    pub fn not_found_error(msg: String) -> Self {
        MenuError::DatabaseOperationFailed(msg)
    }

    pub fn validation_error(msg: String) -> Self {
        MenuError::DatabaseOperationFailed(msg)
    }

    pub fn internal_error(msg: String) -> Self {
        MenuError::DatabaseOperationFailed(msg)
    }
}
