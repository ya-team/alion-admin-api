/// 认证服务数据传输对象
/// 
/// 该模块定义了认证服务中使用的数据传输对象，用于：
/// - 登录上下文信息传递
/// - 认证请求和响应数据封装
/// - 令牌和会话信息管理
/// 
/// # 主要组件
/// 
/// ## 登录上下文
/// * `LoginContext`: 用户登录时的上下文信息，包含客户端信息、请求信息等
/// 
/// # 使用示例
/// 
/// use server_service::admin::dto::sys_auth_dto::*;
/// use server_constant::definition::Audience;
/// 
/// // 创建登录上下文
/// let context = LoginContext {
///     client_ip: "127.0.0.1".to_string(),
///     client_port: Some(8080),
///     address: "localhost".to_string(),
///     user_agent: "Mozilla/5.0".to_string(),
///     request_id: "req-123".to_string(),
///     audience: Audience::Admin,
///     login_type: "password".to_string(),
///     domain: "example.com".to_string(),
/// };
/// 

use server_constant::definition::Audience;

/// 登录上下文信息
/// 
/// 记录用户登录时的上下文信息，用于：
/// - 登录日志记录
/// - 安全审计
/// - 会话管理
/// 
/// # 字段
/// * `client_ip`: 客户端IP地址
/// * `client_port`: 客户端端口号（可选）
/// * `address`: 访问地址
/// * `user_agent`: 用户代理信息
/// * `request_id`: 请求ID，用于请求追踪
/// * `audience`: 认证受众，如管理后台、API等
/// * `login_type`: 登录类型，如密码登录、OAuth等
/// * `domain`: 登录域名
/// 
/// # 使用示例
/// 
/// let context = LoginContext {
///     client_ip: "127.0.0.1".to_string(),
///     client_port: Some(8080),
///     address: "localhost".to_string(),
///     user_agent: "Mozilla/5.0".to_string(),
///     request_id: "req-123".to_string(),
///     audience: Audience::Admin,
///     login_type: "password".to_string(),
///     domain: "example.com".to_string(),
/// };
/// 
#[derive(Clone, Debug)]
pub struct LoginContext {
    /// 客户端IP地址
    pub client_ip: String,
    /// 客户端端口号（可选）
    pub client_port: Option<i32>,
    /// 访问地址
    pub address: String,
    /// 用户代理信息
    pub user_agent: String,
    /// 请求ID，用于请求追踪
    pub request_id: String,
    /// 认证受众，如管理后台、API等
    pub audience: Audience,
    /// 登录类型，如密码登录、OAuth等
    pub login_type: String,
    /// 登录域名
    pub domain: String,
}
