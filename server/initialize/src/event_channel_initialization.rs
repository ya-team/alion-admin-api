/**
 * 事件通道初始化模块
 * 
 * 本模块负责初始化系统的事件通道，注册各种事件监听器，
 * 包括认证、审计、API密钥验证等事件的处理。
 */

use server_constant::definition::consts::SystemEvent;
use server_global::global;

/**
 * 初始化事件通道
 * 
 * 注册系统所需的事件监听器，包括：
 * - JWT创建事件监听器
 * - 认证登录事件监听器
 * - 审计操作日志事件监听器
 * - API密钥验证事件监听器
 */
pub async fn initialize_event_channel() {
    use server_service::admin::{
        api_key_validate_listener, auth_login_listener, jwt_created_listener,
        sys_operation_log_listener,
    };

    global::register_event_listeners(
        Box::new(|rx| Box::pin(jwt_created_listener(rx))),
        &[
            (
                SystemEvent::AuthLoggedInEvent.to_string(),
                Box::new(|rx| Box::pin(auth_login_listener(rx))),
            ),
            (
                SystemEvent::AuditOperationLoggedEvent.to_string(),
                Box::new(|rx| Box::pin(sys_operation_log_listener(rx))),
            ),
            (
                SystemEvent::AuthApiKeyValidatedEvent.to_string(),
                Box::new(|rx| Box::pin(api_key_validate_listener(rx))),
            ),
        ],
    )
    .await;
}
