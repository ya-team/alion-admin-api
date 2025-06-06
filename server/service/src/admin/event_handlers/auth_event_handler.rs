/// 认证事件处理器
/// 
/// 该模块定义了认证相关事件的处理逻辑，用于：
/// - 处理用户登录事件
/// - 记录登录日志
/// - 管理访问令牌
/// 
/// # 主要组件
/// 
/// ## 事件结构
/// * `AuthEvent`: 认证事件，包含用户认证信息和上下文数据
/// 
/// ## 事件处理器
/// * `AuthEventHandler`: 处理认证事件，包括登录日志和访问令牌的处理
/// 
/// # 使用示例
/// 
/// use server_service::admin::event_handlers::auth_event_handler::*;
/// 
/// // 创建认证事件
/// let event = AuthEvent {
///     user_id: "user1".to_string(),
///     username: "admin".to_string(),
///     domain: "example.com".to_string(),
///     access_token: "token123".to_string(),
///     refresh_token: "refresh456".to_string(),
///     client_ip: "127.0.0.1".to_string(),
///     client_port: Some(8080),
///     address: "localhost".to_string(),
///     user_agent: "Mozilla/5.0".to_string(),
///     request_id: "req-123".to_string(),
///     login_type: "password".to_string(),
/// };
/// 
/// // 处理登录事件
/// AuthEventHandler::handle_login(event).await?;
/// 

use server_core::web::error::AppError;

use crate::{
    admin::events::{access_token_event::AccessTokenEvent, login_log_event::LoginLogEvent},
    helper::db_helper,
};

/// 认证事件
/// 
/// 表示一个用户认证事件，包含：
/// - 用户信息（用户ID、用户名）
/// - 令牌信息（访问令牌、刷新令牌）
/// - 上下文信息（域名、IP、端口等）
/// 
/// # 字段
/// * `user_id`: 用户ID
/// * `username`: 用户名
/// * `domain`: 域名
/// * `access_token`: 访问令牌
/// * `refresh_token`: 刷新令牌
/// * `client_ip`: 客户端IP地址
/// * `client_port`: 客户端端口号（可选）
/// * `address`: 访问地址
/// * `user_agent`: 用户代理信息
/// * `request_id`: 请求ID
/// * `login_type`: 登录类型
/// 
/// # 使用示例
/// 
/// let event = AuthEvent {
///     user_id: "user1".to_string(),
///     username: "admin".to_string(),
///     domain: "example.com".to_string(),
///     access_token: "token123".to_string(),
///     refresh_token: "refresh456".to_string(),
///     client_ip: "127.0.0.1".to_string(),
///     client_port: Some(8080),
///     address: "localhost".to_string(),
///     user_agent: "Mozilla/5.0".to_string(),
///     request_id: "req-123".to_string(),
///     login_type: "password".to_string(),
/// };
/// 
pub struct AuthEvent {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub username: String,
    /// 域名
    pub domain: String,
    /// 访问令牌
    pub access_token: String,
    /// 刷新令牌
    pub refresh_token: String,
    /// 客户端IP地址
    pub client_ip: String,
    /// 客户端端口号（可选）
    pub client_port: Option<i32>,
    /// 访问地址
    pub address: String,
    /// 用户代理信息
    pub user_agent: String,
    /// 请求ID
    pub request_id: String,
    /// 登录类型
    pub login_type: String,
}

/// 认证事件处理器
/// 
/// 处理认证相关事件，包括：
/// - 登录日志记录
/// - 访问令牌管理
/// 
/// # 使用示例
/// 
/// let event = AuthEvent { /* ... */ };
/// AuthEventHandler::handle_login(event).await?;
/// 
pub struct AuthEventHandler;

impl AuthEventHandler {
    /// 处理登录事件
    /// 
    /// 处理用户登录事件，包括：
    /// - 记录登录日志
    /// - 创建访问令牌
    /// 
    /// # 参数
    /// * `event` - 认证事件
    /// 
    /// # 返回
    /// * `Result<(), AppError>` - 成功返回 `()`，失败返回错误
    /// 
    /// # 使用示例
    /// 
    /// let event = AuthEvent { /* ... */ };
    /// AuthEventHandler::handle_login(event).await?;
    /// 
    pub async fn handle_login(event: AuthEvent) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;

        // 处理登录日志
        let login_log_event = LoginLogEvent {
            user_id: event.user_id.clone(),
            username: event.username.clone(),
            domain: event.domain.clone(),
            ip: event.client_ip.clone(),
            port: event.client_port,
            address: event.address.clone(),
            user_agent: event.user_agent.clone(),
            request_id: event.request_id.clone(),
            login_type: event.login_type.clone(),
        };

        login_log_event.handle(&db).await?;

        // 处理访问令牌
        let access_token_event = AccessTokenEvent {
            access_token: event.access_token,
            refresh_token: event.refresh_token,
            user_id: event.user_id,
            username: event.username,
            domain: event.domain,
            ip: event.client_ip,
            port: event.client_port,
            address: event.address.clone(),
            user_agent: event.user_agent,
            request_id: event.request_id,
            login_type: event.login_type,
        };

        access_token_event.handle(&db).await?;

        Ok(())
    }
}
