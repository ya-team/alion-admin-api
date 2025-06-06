/// 管理后台事件定义模块
/// 
/// 该模块定义了管理后台服务中的事件类型，用于：
/// - 事件驱动架构
/// - 服务间通信
/// - 异步操作处理
/// 
/// # 主要组件
/// 
/// ## 认证事件
/// * `AccessTokenEvent`: 访问令牌相关事件，如创建、刷新、撤销等
/// 
/// ## 日志事件
/// * `LoginLogEvent`: 登录日志事件，记录用户登录、登出等操作
/// 
/// # 使用示例
/// 
/// use server_service::admin::events::*;
/// 
/// // 创建访问令牌事件
/// let event = AccessTokenEvent::Created {
///     user_id: 1,
///     token: "token123".to_string(),
/// };
/// 
/// // 创建登录日志事件
/// let event = LoginLogEvent::Login {
///     user_id: 1,
///     ip: "127.0.0.1".to_string(),
///     user_agent: "Mozilla/5.0".to_string(),
/// };
/// 

pub mod access_token_event;
pub mod login_log_event;
