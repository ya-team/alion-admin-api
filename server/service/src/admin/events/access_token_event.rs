/** 访问令牌事件定义
 * 
 * 该模块定义了访问令牌相关的事件类型和处理逻辑，用于：
 * - 令牌创建和存储
 * - 令牌状态管理
 * - 令牌使用记录
 * 
 * 主要组件
 * --------
 * 
 * 事件结构
 * --------
 * * `AccessTokenEvent`: 访问令牌事件，包含令牌信息和上下文数据
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
 * /* 处理事件
 *  * event.handle(&db).await?;
 *  */
 */

use chrono::Local;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use server_constant::definition::consts::TokenStatus;
use server_core::web::error::AppError;
use server_model::admin::entities::sys_tokens::ActiveModel as SysTokensActiveModel;
use ulid::Ulid;

/** 访问令牌事件
 * 
 * 表示一个访问令牌的创建事件，包含：
 * - 令牌信息（访问令牌、刷新令牌）
 * - 用户信息（用户ID、用户名）
 * - 上下文信息（域名、IP、端口等）
 * 
 * 字段
 * --------
 * * `access_token`: 访问令牌
 * * `refresh_token`: 刷新令牌
 * * `user_id`: 用户ID
 * * `username`: 用户名
 * * `domain`: 域名
 * * `ip`: 客户端IP地址
 * * `port`: 客户端端口号（可选）
 * * `address`: 访问地址
 * * `user_agent`: 用户代理信息
 * * `request_id`: 请求ID
 * * `login_type`: 登录类型
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
 */
#[derive(Clone, Debug)]
pub struct AccessTokenEvent {
    /** 访问令牌 */
    pub access_token: String,
    /** 刷新令牌 */
    pub refresh_token: String,
    /** 用户ID */
    pub user_id: String,
    /** 用户名 */
    pub username: String,
    /** 域名 */
    pub domain: String,
    /** 客户端IP地址 */
    pub ip: String,
    /** 客户端端口号（可选） */
    pub port: Option<i32>,
    /** 访问地址 */
    pub address: String,
    /** 用户代理信息 */
    pub user_agent: String,
    /** 请求ID */
    pub request_id: String,
    /** 登录类型 */
    pub login_type: String,
}

impl AccessTokenEvent {
    /** 处理访问令牌事件
     * 
     * 将访问令牌事件信息保存到数据库，包括：
     * - 生成唯一ID
     * - 设置令牌状态
     * - 记录创建时间和创建者
     * 
     * 参数
     * --------
     * * `db` - 数据库连接
     * 
     * 返回
     * --------
     * * `Result<(), AppError>` - 成功返回 `()`，失败返回错误
     * 
     * 使用示例
     * --------
     * /* 处理访问令牌事件
     *  * let event = AccessTokenEvent { /* ... */ };
     *  * event.handle(&db).await?;
     *  */
     */
    pub async fn handle(self, db: &DatabaseConnection) -> Result<(), AppError> {
        let now = Local::now().naive_local();

        SysTokensActiveModel {
            id: Set(Ulid::new().to_string()),
            access_token: Set(self.access_token),
            refresh_token: Set(self.refresh_token),
            status: Set(TokenStatus::Active.to_string()),
            user_id: Set(self.user_id),
            username: Set(self.username.clone()),
            domain: Set(self.domain),
            login_time: Set(now),
            ip: Set(self.ip),
            port: Set(self.port),
            address: Set(self.address),
            user_agent: Set(self.user_agent),
            request_id: Set(self.request_id),
            r#type: Set(self.login_type),
            created_at: Set(now),
            created_by: Set(self.username),
        }
        .insert(db)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }
}
