/// 错误处理模块
/// 
/// 该模块提供了统一的错误处理机制，包括：
/// - API错误接口定义：定义了统一的错误处理接口
/// - 应用错误类型：实现了通用的应用错误结构
/// - 数据库错误转换：支持SeaORM数据库错误转换
/// - Redis错误转换：支持Redis错误转换
/// - MongoDB错误转换：支持MongoDB错误转换
/// - JWT错误转换：支持JWT相关错误转换
/// 
/// # 错误处理流程
/// 
/// 1. 错误捕获：捕获各种来源的错误（数据库、Redis、MongoDB等）
/// 2. 错误转换：将错误转换为统一的AppError格式
/// 3. 错误响应：将AppError转换为HTTP响应
/// 
/// # 错误码说明
/// 
/// - 400: 请求参数错误
/// - 401: 认证失败
/// - 404: 资源未找到
/// - 500: 服务器内部错误
/// - 503: 服务不可用

use axum::response::{IntoResponse, Response};
use mongodb::error::{Error as MongoError, ErrorKind};
use redis::RedisError;
use sea_orm::DbErr;

use crate::web::{jwt::JwtError, res::Res};

/// API错误接口
/// 
/// 定义了API错误的基本行为，包括错误码和错误消息。
/// 所有需要转换为HTTP响应的错误类型都应该实现这个trait。
/// 
/// # 实现要求
/// 
/// - code(): 返回HTTP状态码
/// - message(): 返回错误描述信息
pub trait ApiError {
    /// 获取错误码
    /// 
    /// 返回HTTP状态码，用于HTTP响应。
    /// 
    /// # 返回
    /// * `u16` - HTTP状态码
    fn code(&self) -> u16;

    /// 获取错误消息
    /// 
    /// 返回人类可读的错误描述信息。
    /// 
    /// # 返回
    /// * `String` - 错误描述信息
    fn message(&self) -> String;
}

/// 应用错误结构体
/// 
/// 用于表示应用程序中的错误，包含错误码和错误消息。
/// 实现了IntoResponse trait，可以直接转换为HTTP响应。
/// 
/// # 字段
/// 
/// * `code`: HTTP状态码
/// * `message`: 错误描述信息
#[derive(Debug)]
pub struct AppError {
    /// HTTP状态码
    pub code: u16,
    /// 错误描述信息
    pub message: String,
}

impl IntoResponse for AppError {
    /// 将错误转换为HTTP响应
    /// 
    /// 使用Res结构体包装错误信息，并转换为HTTP响应。
    /// 
    /// # 返回
    /// * `Response` - HTTP响应
    fn into_response(self) -> Response {
        Res::<()>::new_error(self.code, self.message.as_str()).into_response()
    }
}

impl ApiError for AppError {
    /// 获取错误码
    /// 
    /// # 返回
    /// * `u16` - HTTP状态码
    fn code(&self) -> u16 {
        self.code
    }

    /// 获取错误消息
    /// 
    /// # 返回
    /// * `String` - 错误描述信息
    fn message(&self) -> String {
        self.message.to_string()
    }
}

impl ApiError for DbErr {
    /// 获取数据库错误对应的HTTP状态码
    /// 
    /// 根据不同的数据库错误类型返回对应的HTTP状态码。
    /// 
    /// # 返回
    /// * `u16` - HTTP状态码
    fn code(&self) -> u16 {
        match self {
            DbErr::ConnectionAcquire(_) => 503, // 服务不可用
            DbErr::TryIntoErr { .. } => 400,    // 请求参数错误
            DbErr::Conn(_) => 500,              // 服务器内部错误
            DbErr::Exec(_) => 500,              // 执行错误
            DbErr::Query(_) => 500,             // 查询错误
            DbErr::ConvertFromU64(_) => 400,    // 请求参数错误
            DbErr::UnpackInsertId => 500,       // 服务器内部错误
            DbErr::UpdateGetPrimaryKey => 500,  // 更新错误
            DbErr::RecordNotFound(_) => 404,    // 未找到记录
            DbErr::AttrNotSet(_) => 400,        // 请求参数错误
            DbErr::Custom(_) => 500,            // 自定义错误
            DbErr::Type(_) => 400,              // 类型错误
            DbErr::Json(_) => 400,              // JSON解析错误
            DbErr::Migration(_) => 500,         // 迁移错误
            DbErr::RecordNotInserted => 400,    // 记录未插入
            DbErr::RecordNotUpdated => 404,     // 记录未更新
        }
    }

    /// 获取数据库错误消息
    /// 
    /// # 返回
    /// * `String` - 错误描述信息
    fn message(&self) -> String {
        format!("{}", self)
    }
}

impl From<DbErr> for AppError {
    /// 从数据库错误转换为应用错误
    /// 
    /// 将SeaORM数据库错误转换为统一的AppError格式。
    /// 
    /// # 参数
    /// * `err` - 数据库错误
    /// 
    /// # 返回
    /// * `Self` - 应用错误
    fn from(err: DbErr) -> Self {
        AppError {
            code: err.code(),
            message: err.message(),
        }
    }
}

impl From<JwtError> for AppError {
    /// 从JWT错误转换为应用错误
    /// 
    /// 将JWT相关错误转换为统一的AppError格式。
    /// 
    /// # 参数
    /// * `err` - JWT错误
    /// 
    /// # 返回
    /// * `Self` - 应用错误
    fn from(err: JwtError) -> Self {
        AppError {
            code: 400,
            message: err.to_string(),
        }
    }
}

impl From<RedisError> for AppError {
    /// 从Redis错误转换为应用错误
    /// 
    /// 将Redis错误转换为统一的AppError格式。
    /// 根据Redis错误类型返回对应的HTTP状态码。
    /// 
    /// # 参数
    /// * `err` - Redis错误
    /// 
    /// # 返回
    /// * `Self` - 应用错误
    fn from(err: RedisError) -> Self {
        use redis::ErrorKind;
        let code = match err.kind() {
            ErrorKind::ResponseError => 500,        // Redis响应错误
            ErrorKind::AuthenticationFailed => 401, // 认证失败
            ErrorKind::TypeError => 400,            // 类型错误
            ErrorKind::ExecAbortError => 500,       // 执行中止
            ErrorKind::BusyLoadingError => 503,     // Redis正在加载
            ErrorKind::InvalidClientConfig => 400,  // 客户端配置错误
            ErrorKind::IoError => 503,              // IO错误，可能是网络问题
            ErrorKind::ExtensionError => 500,       // 扩展错误
            _ => 500,                               // 其他错误
        };

        let message = if let Some(redis_code) = err.code() {
            format!("[{}] {}", redis_code, err)
        } else {
            format!("{}", err)
        };

        AppError { code, message }
    }
}

impl From<MongoError> for AppError {
    /// 从MongoDB错误转换为应用错误
    /// 
    /// 将MongoDB错误转换为统一的AppError格式。
    /// 根据MongoDB错误类型返回对应的HTTP状态码。
    /// 
    /// # 参数
    /// * `err` - MongoDB错误
    /// 
    /// # 返回
    /// * `Self` - 应用错误
    fn from(err: MongoError) -> Self {
        let code = match *err.kind {
            ErrorKind::Authentication { .. } => 401,        // 认证错误
            ErrorKind::InvalidArgument { .. } => 400,       // 参数错误
            ErrorKind::DnsResolve { .. } => 503,            // DNS解析错误
            ErrorKind::ConnectionPoolCleared { .. } => 503, // 连接池错误
            ErrorKind::Io(_) => 503,                        // IO错误
            ErrorKind::Command(_) => 400,                   // 命令执行错误
            ErrorKind::Write(_) => 500,                     // 写入错误
            ErrorKind::ServerSelection { .. } => 503,       // 服务器选择错误
            ErrorKind::Transaction { .. } => 500,           // 事务错误
            ErrorKind::Internal { .. } => 500,              // 内部错误
            ErrorKind::BsonDeserialization(_) => 400,       // BSON反序列化错误
            ErrorKind::BsonSerialization(_) => 400,         // BSON序列化错误
            ErrorKind::InvalidResponse { .. } => 500,       // 无效响应
            ErrorKind::IncompatibleServer { .. } => 503,    // 服务器不兼容
            ErrorKind::SessionsNotSupported => 503,         // 不支持会话
            ErrorKind::InvalidTlsConfig { .. } => 500,      // TLS配置错误
            ErrorKind::MissingResumeToken => 500,           // 缺少恢复令牌
            ErrorKind::GridFs(_) => 500,                    // GridFS错误
            ErrorKind::Custom(_) => 500,                    // 自定义错误
            ErrorKind::Shutdown => 503,                     // 关闭错误
            _ => 500,                                       // 其他未知错误
        };

        AppError {
            code,
            message: err.to_string(),
        }
    }
}
