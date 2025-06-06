/** 基础错误类型和宏定义
 * 
 * 该模块定义了服务错误处理的基础类型和辅助宏，包括：
 * - 服务错误trait
 * - 通用错误枚举
 * - 错误转换宏
 * 
 * 提供了统一的错误处理机制，支持错误代码、消息和类型转换。
 * 
 * 使用示例
 * --------
 * /* 创建通用错误
 *  * let error = CommonError::DatabaseError("Connection failed".into());
 *  * assert_eq!(error.code(), 5001);
 *  */
 * 
 * /* 使用错误转换宏
 *  * #[derive(Debug)]
 *  * struct MyError(String);
 *  * 
 *  * impl_from_common_error!(MyError);
 *  * impl_from_db_error!(MyError);
 *  */
 */

#[allow(unused_imports)]
use std::fmt;
use thiserror::Error;

/** 服务错误trait
 * 
 * 所有服务错误类型必须实现的trait，提供了错误代码和消息的访问方法。
 * 
 * 要求
 * --------
 * * 实现 `Display` 和 `Debug` trait
 * * 实现 `Send` 和 `Sync` trait
 * 
 * 使用示例
 * --------
 * /* 实现服务错误trait
 *  * #[derive(Debug)]
 *  * struct MyError {
 *  *     code: i32,
 *  *     message: String,
 *  * }
 *  * 
 *  * impl ServiceError for MyError {
 *  *     fn code(&self) -> i32 {
 *  *         self.code
 *  *     }
 *  *     
 *  *     fn message(&self) -> String {
 *  *         self.message.clone()
 *  *     }
 *  * }
 *  */
 */
pub trait ServiceError: fmt::Display + fmt::Debug + Send + Sync {
    /** 获取错误代码
     * 
     * 返回
     * --------
     * * `i32` - 错误代码
     */
    fn code(&self) -> i32;
    
    /** 获取错误消息
     * 
     * 返回
     * --------
     * * `String` - 错误消息
     */
    fn message(&self) -> String {
        self.to_string()
    }
}

/** 通用错误枚举
 * 
 * 定义了服务中常见的错误类型，每个变体都包含详细的错误消息。
 * 
 * 变体
 * --------
 * * `DatabaseError` - 数据库错误 (5001)
 * * `AuthenticationError` - 认证错误 (4001)
 * * `AuthorizationError` - 授权错误 (4003)
 * * `NotFoundError` - 资源未找到错误 (4004)
 * * `ValidationError` - 验证错误 (4000)
 * * `InternalError` - 内部服务器错误 (5000)
 * 
 * 使用示例
 * --------
 * /* 创建通用错误
 *  * let error = CommonError::DatabaseError("Connection failed".into());
 *  * assert_eq!(error.code(), 5001);
 *  * assert_eq!(error.message(), "Database error: Connection failed");
 *  */
 */
#[derive(Error, Debug)]
pub enum CommonError {
    #[error("Database error: {0}")]
    DatabaseError(String),

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

impl ServiceError for CommonError {
    fn code(&self) -> i32 {
        match self {
            CommonError::DatabaseError(_) => 5001,
            CommonError::AuthenticationError(_) => 4001,
            CommonError::AuthorizationError(_) => 4003,
            CommonError::NotFoundError(_) => 4004,
            CommonError::ValidationError(_) => 4000,
            CommonError::InternalError(_) => 5000,
        }
    }
}

/** 通用错误转换宏
 * 
 * 为服务特定错误类型实现从 `CommonError` 的转换。
 * 
 * 参数
 * --------
 * * `$error_type` - 要为其实现转换的错误类型
 * 
 * 要求
 * --------
 * 目标类型必须实现以下方法：
 * * `database_error(msg: String) -> Self`
 * * `authentication_error(msg: String) -> Self`
 * * `authorization_error(msg: String) -> Self`
 * * `not_found_error(msg: String) -> Self`
 * * `validation_error(msg: String) -> Self`
 * * `internal_error(msg: String) -> Self`
 * 
 * 使用示例
 * --------
 * /* 实现错误转换
 *  * #[derive(Debug)]
 *  * struct MyError(String);
 *  * 
 *  * impl MyError {
 *  *     fn database_error(msg: String) -> Self { Self(msg) }
 *  *     fn authentication_error(msg: String) -> Self { Self(msg) }
 *  *     fn authorization_error(msg: String) -> Self { Self(msg) }
 *  *     fn not_found_error(msg: String) -> Self { Self(msg) }
 *  *     fn validation_error(msg: String) -> Self { Self(msg) }
 *  *     fn internal_error(msg: String) -> Self { Self(msg) }
 *  * }
 *  * 
 *  * impl_from_common_error!(MyError);
 *  */
 */
#[macro_export]
macro_rules! impl_from_common_error {
    ($error_type:ty) => {
        impl From<crate::admin::errors::CommonError> for $error_type {
            fn from(error: crate::admin::errors::CommonError) -> Self {
                match error {
                    crate::admin::errors::CommonError::DatabaseError(msg) => Self::database_error(msg),
                    crate::admin::errors::CommonError::AuthenticationError(msg) => Self::authentication_error(msg),
                    crate::admin::errors::CommonError::AuthorizationError(msg) => Self::authorization_error(msg),
                    crate::admin::errors::CommonError::NotFoundError(msg) => Self::not_found_error(msg),
                    crate::admin::errors::CommonError::ValidationError(msg) => Self::validation_error(msg),
                    crate::admin::errors::CommonError::InternalError(msg) => Self::internal_error(msg),
                }
            }
        }
    };
}

/** 数据库错误转换宏
 * 
 * 为服务特定错误类型实现从 `sea_orm::DbErr` 的转换。
 * 
 * 参数
 * --------
 * * `$error_type` - 要为其实现转换的错误类型
 * 
 * 要求
 * --------
 * 目标类型必须实现从 `CommonError` 的转换
 * 
 * 使用示例
 * --------
 * /* 实现数据库错误转换
 *  * #[derive(Debug)]
 *  * struct MyError(String);
 *  * 
 *  * impl From<CommonError> for MyError {
 *  *     fn from(error: CommonError) -> Self {
 *  *         Self(error.to_string())
 *  *     }
 *  * }
 *  * 
 *  * impl_from_db_error!(MyError);
 *  */
 */
#[macro_export]
macro_rules! impl_from_db_error {
    ($error_type:ty) => {
        impl From<sea_orm::DbErr> for $error_type {
            fn from(error: sea_orm::DbErr) -> Self {
                Self::from(crate::admin::errors::CommonError::DatabaseError(error.to_string()))
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_error_codes() {
        assert_eq!(CommonError::DatabaseError("test".into()).code(), 5001);
        assert_eq!(CommonError::AuthenticationError("test".into()).code(), 4001);
        assert_eq!(CommonError::AuthorizationError("test".into()).code(), 4003);
        assert_eq!(CommonError::NotFoundError("test".into()).code(), 4004);
        assert_eq!(CommonError::ValidationError("test".into()).code(), 4000);
        assert_eq!(CommonError::InternalError("test".into()).code(), 5000);
    }
} 