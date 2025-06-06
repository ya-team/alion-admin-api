/*! 端点错误模块
 * 
 * 该模块定义了与API端点（Endpoint）相关的错误类型。
 * 包括端点的创建、修改、删除等操作相关的错误。
 * 
 * 错误类型
 * --------
 * EndpointError 定义了端点相关的所有错误情况，包括：
 * - 端点不存在
 * - 多个端点不存在
 * - 数据库错误
 * - 认证错误
 * - 授权错误
 * - 资源不存在
 * - 验证错误
 * - 内部错误
 * 
 * 错误代码
 * --------
 * - 404: 端点不存在
 * - 404: 多个端点不存在
 * - 500: 数据库错误
 * - 401: 认证错误
 * - 403: 授权错误
 * - 404: 资源不存在
 * - 400: 验证错误
 * - 500: 内部错误
 * 
 * 使用示例
 * --------
 * /* 创建端点不存在错误
 *  * let error = EndpointError::EndpointNotFound("/api/users".to_string());
 *  */
 * 
 * /* 处理多个端点不存在错误
 *  * let error = EndpointError::EndpointsNotFound(vec![1, 2, 3]);
 *  */
 * 
 * /* 处理数据库错误
 *  * let db_error = EndpointError::DatabaseError(Box::new(
 *  *     std::io::Error::new(std::io::ErrorKind::Other, "Failed to save endpoint")
 *  * ));
 *  */
 */

use server_core::web::error::AppError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EndpointError {
    #[error("Endpoint not found: {0}")]
    EndpointNotFound(String),
    #[error("One or more endpoints not found: {0:?}")]
    EndpointsNotFound(Vec<i32>),
    #[error("Database error: {0}")]
    DatabaseError(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    #[error("Authorization error: {0}")]
    AuthorizationError(String),
    #[error("Not found error: {0}")]
    NotFoundError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<EndpointError> for AppError {
    fn from(error: EndpointError) -> Self {
        match error {
            EndpointError::EndpointNotFound(_) => AppError { code: 404, message: error.to_string() },
            EndpointError::EndpointsNotFound(_) => AppError { code: 404, message: error.to_string() },
            EndpointError::DatabaseError(e) => AppError { code: 500, message: e.to_string() },
            EndpointError::AuthenticationError(msg) => AppError { code: 401, message: msg },
            EndpointError::AuthorizationError(msg) => AppError { code: 403, message: msg },
            EndpointError::NotFoundError(msg) => AppError { code: 404, message: msg },
            EndpointError::ValidationError(msg) => AppError { code: 400, message: msg },
            EndpointError::InternalError(msg) => AppError { code: 500, message: msg },
        }
    }
}

impl From<sea_orm::DbErr> for EndpointError {
    fn from(error: sea_orm::DbErr) -> Self {
        EndpointError::DatabaseError(Box::new(error))
    }
} 