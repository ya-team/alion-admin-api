/** 登录日志事件定义
 * 
 * 该模块定义了登录日志相关的事件类型和处理逻辑，用于：
 * - 记录用户登录信息
 * - 跟踪登录状态
 * - 分析登录行为
 * 
 * 主要组件
 * --------
 * 
 * 事件结构
 * --------
 * * `LoginLogEvent`: 登录日志事件，包含登录信息和上下文数据
 * 
 * 使用示例
 * --------
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
 * 
 * /* 处理事件
 *  * event.handle(&db).await?;
 *  */
 */

use chrono::Local;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use server_core::web::error::AppError;
use server_model::admin::entities::sys_login_log::ActiveModel as SysLoginLogActiveModel;
use ulid::Ulid;

/** 登录日志事件
 * 
 * 表示一个用户登录事件，包含：
 * - 用户信息（用户ID、用户名）
 * - 登录上下文（域名、IP、端口等）
 * - 请求信息（请求ID、用户代理等）
 * 
 * 字段
 * --------
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
#[derive(Clone, Debug)]
pub struct LoginLogEvent {
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

impl LoginLogEvent {
    /** 处理登录日志事件
     * 
     * 将登录日志事件信息保存到数据库，包括：
     * - 生成唯一ID
     * - 记录登录时间
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
     * /* 处理登录日志事件
     *  * let event = LoginLogEvent { /* ... */ };
     *  * event.handle(&db).await?;
     *  */
     */
    pub async fn handle(self, db: &DatabaseConnection) -> Result<(), AppError> {
        let now = Local::now().naive_local();

        SysLoginLogActiveModel {
            id: Set(Ulid::new().to_string()),
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
