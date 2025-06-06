/**
 * 应用程序错误类型定义
 * 
 * 本模块定义了应用程序中使用的所有错误类型，包括：
 * - 业务错误（400-499）：如参数验证、未授权、资源不存在等
 * - 系统错误（500-599）：如数据库错误、内部服务器错误等
 * - 网络错误：如网络连接问题、请求超时等
 * - 其他错误：未知错误类型
 */

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use tracing::{error, warn};

/**
 * 应用程序错误枚举
 * 
 * 定义了应用程序中可能出现的所有错误类型，每个错误类型都包含：
 * - 错误消息
 * - HTTP状态码
 * - 错误代码
 * - 日志级别
 */
#[derive(Error, Debug)]
pub enum AppError {
    // 业务错误 (400-499)
    /** 业务逻辑错误 */
    #[error("业务错误: {0}")]
    Business(String),
    
    /** 参数验证错误 */
    #[error("参数验证错误: {0}")]
    Validation(String),
    
    /** 未授权访问错误 */
    #[error("未授权访问: {0}")]
    Unauthorized(String),
    
    /** 资源不存在错误 */
    #[error("资源不存在: {0}")]
    NotFound(String),
    
    /** 资源冲突错误 */
    #[error("资源已存在: {0}")]
    Conflict(String),
    
    // 系统错误 (500-599)
    /** 数据库操作错误 */
    #[error("数据库错误: {0}")]
    Database(#[from] sea_orm::DbErr),
    
    /** 内部服务器错误 */
    #[error("内部服务器错误: {0}")]
    Internal(String),
    
    /** 外部服务调用错误 */
    #[error("外部服务错误: {0}")]
    External(String),
    
    // 网络错误
    /** 网络连接错误 */
    #[error("网络错误: {0}")]
    Network(String),
    
    /** 请求超时错误 */
    #[error("请求超时: {0}")]
    Timeout(String),
    
    // 其他错误
    /** 未知错误类型 */
    #[error("未知错误: {0}")]
    Unknown(String),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            // 业务错误
            AppError::Business(_) => StatusCode::BAD_REQUEST,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            
            // 系统错误
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::External(_) => StatusCode::BAD_GATEWAY,
            
            // 网络错误
            AppError::Network(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::Timeout(_) => StatusCode::GATEWAY_TIMEOUT,
            
            // 其他错误
            AppError::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    
    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::Business(_) => "BUSINESS_ERROR",
            AppError::Validation(_) => "VALIDATION_ERROR",
            AppError::Unauthorized(_) => "UNAUTHORIZED",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::Conflict(_) => "CONFLICT",
            AppError::Database(_) => "DATABASE_ERROR",
            AppError::Internal(_) => "INTERNAL_ERROR",
            AppError::External(_) => "EXTERNAL_ERROR",
            AppError::Network(_) => "NETWORK_ERROR",
            AppError::Timeout(_) => "TIMEOUT",
            AppError::Unknown(_) => "UNKNOWN_ERROR",
        }
    }
    
    pub fn log_error(&self) {
        match self {
            // 业务错误使用 warn 级别
            AppError::Business(msg) => warn!(error = %msg, error_code = self.error_code(), "业务错误"),
            AppError::Validation(msg) => warn!(error = %msg, error_code = self.error_code(), "参数验证错误"),
            AppError::Unauthorized(msg) => warn!(error = %msg, error_code = self.error_code(), "未授权访问"),
            AppError::NotFound(msg) => warn!(error = %msg, error_code = self.error_code(), "资源不存在"),
            AppError::Conflict(msg) => warn!(error = %msg, error_code = self.error_code(), "资源冲突"),
            
            // 系统错误使用 error 级别
            AppError::Database(err) => error!(error = ?err, error_code = self.error_code(), "数据库错误"),
            AppError::Internal(msg) => error!(error = %msg, error_code = self.error_code(), "内部服务器错误"),
            AppError::External(msg) => error!(error = %msg, error_code = self.error_code(), "外部服务错误"),
            AppError::Network(msg) => error!(error = %msg, error_code = self.error_code(), "网络错误"),
            AppError::Timeout(msg) => error!(error = %msg, error_code = self.error_code(), "请求超时"),
            AppError::Unknown(msg) => error!(error = %msg, error_code = self.error_code(), "未知错误"),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 记录错误日志
        self.log_error();
        
        // 构建错误响应
        let status = self.status_code();
        let error_response = json!({
            "code": self.error_code(),
            "message": self.to_string(),
            "status": status.as_u16(),
        });
        
        (status, Json(error_response)).into_response()
    }
}

// 为常见错误类型实现 From trait
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Internal(format!("IO错误: {}", err))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Validation(format!("JSON解析错误: {}", err))
    }
}

impl From<tokio::time::error::Elapsed> for AppError {
    fn from(_: tokio::time::error::Elapsed) -> Self {
        AppError::Timeout("操作超时".to_string())
    }
}

// 辅助函数，用于创建各种类型的错误
impl AppError {
    pub fn business(msg: impl Into<String>) -> Self {
        AppError::Business(msg.into())
    }
    
    pub fn validation(msg: impl Into<String>) -> Self {
        AppError::Validation(msg.into())
    }
    
    pub fn unauthorized(msg: impl Into<String>) -> Self {
        AppError::Unauthorized(msg.into())
    }
    
    pub fn not_found(msg: impl Into<String>) -> Self {
        AppError::NotFound(msg.into())
    }
    
    pub fn conflict(msg: impl Into<String>) -> Self {
        AppError::Conflict(msg.into())
    }
    
    pub fn internal(msg: impl Into<String>) -> Self {
        AppError::Internal(msg.into())
    }
    
    pub fn external(msg: impl Into<String>) -> Self {
        AppError::External(msg.into())
    }
    
    pub fn network(msg: impl Into<String>) -> Self {
        AppError::Network(msg.into())
    }
    
    pub fn timeout(msg: impl Into<String>) -> Self {
        AppError::Timeout(msg.into())
    }
} 