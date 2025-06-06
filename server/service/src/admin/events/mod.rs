/** 事件定义模块
 * 
 * 该模块定义了系统中使用的各种事件类型，用于：
 * - 异步事件处理
 * - 事件驱动架构
 * - 解耦系统组件
 * 
 * 主要组件
 * --------
 * 
 * 事件类型
 * --------
 * * `AccessTokenEvent`: 访问令牌事件，用于处理令牌的创建和存储
 * * `LoginLogEvent`: 登录日志事件，用于记录用户登录信息
 * 
 * 使用示例
 * --------
 * /* 创建访问令牌事件
 *  * let event = AccessTokenEvent {
 *  *     access_token: "token123".to_string(),
 *  *     refresh_token: "refresh456".to_string(),
 *  *     user_id: "user1".to_string(),
 *  *     username: "admin".to_string(),
 *  *     domain: "example.com".to_string(),
 *  *     ip: "127.0.0.1".to_string(),
 *  *     port: Some(8080),
 *  *     address: "localhost".to_string(),
 *  *     user_agent: "Mozilla/5.0".to_string(),
 *  *     request_id: "req-123".to_string(),
 *  *     login_type: "password".to_string(),
 *  * };
 *  */
 * 
 * /* 创建登录日志事件
 *  * let event = LoginLogEvent {
 *  *     user_id: "user1".to_string(),
 *  *     username: "admin".to_string(),
 *  *     domain: "example.com".to_string(),
 *  *     ip: "127.0.0.1".to_string(),
 *  *     port: Some(8080),
 *  *     address: "localhost".to_string(),
 *  *     user_agent: "Mozilla/5.0".to_string(),
 *  *     request_id: "req-123".to_string(),
 *  *     login_type: "password".to_string(),
 *  * };
 *  */
 */

pub mod access_token_event;
pub mod login_log_event;
