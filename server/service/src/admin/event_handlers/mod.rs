/**
 * 管理后台事件处理器模块
 * 
 * 该模块定义了管理后台服务中的事件处理器，用于：
 * - 处理认证相关事件
 * - 处理日志相关事件
 * - 实现事件驱动的业务逻辑
 * 
 * # 主要组件
 * 
 * ## 认证事件处理器
 * * `AuthEventHandler`: 处理认证相关事件，如令牌创建、刷新等
 * 
 * # 使用示例
 * 
 * use server_service::admin::event_handlers::*;
 * 
 * // 创建认证事件处理器
 * let handler = AuthEventHandler::new();
 * 
 * // 处理认证事件
 * handler.handle_auth_event(event).await?;
 */

pub mod auth_event_handler;
