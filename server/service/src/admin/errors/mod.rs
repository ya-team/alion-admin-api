/*! 管理后台错误处理模块
 * 
 * 该模块定义了管理后台服务中使用的错误类型和错误处理机制，包括：
 * - 基础错误类型和宏
 * - 各服务模块的特定错误类型
 * - 错误转换和传播机制
 * 
 * 主要组件
 * --------
 * 
 * 基础错误
 * --------
 * * `CommonError`: 通用错误类型
 * * `ServiceError`: 服务错误类型
 * 
 * 服务特定错误
 * --------
 * * `AuthError`: 认证服务错误
 * * `UserError`: 用户服务错误
 * * `RoleError`: 角色服务错误
 * * `DomainError`: 域名服务错误
 * * `AccessKeyError`: 访问密钥服务错误
 * * `AuthorizationError`: 授权服务错误
 * 
 * 错误处理宏
 * --------
 * * `impl_from_common_error`: 实现从通用错误转换
 * * `impl_from_db_error`: 实现从数据库错误转换
 * 
 * 使用示例
 * --------
 * /* 创建服务错误
 *  * let error = ServiceError::new(500, "Internal server error");
 *  */
 * 
 * /* 创建特定错误
 *  * let error = AuthError::InvalidCredentials;
 *  */
 * 
 * /* 错误转换
 *  * let service_error: ServiceError = error.into();
 *  */
 */

pub mod base_error;
pub mod sys_auth_error;
pub mod sys_user_error;
pub mod sys_role_error;
pub mod sys_menu_error;
pub mod sys_domain_error;
pub mod sys_endpoint_error;
pub mod sys_operation_log_error;
pub mod sys_login_log_error;
pub mod sys_access_key_error;
pub mod sys_authorization_error;

// Re-export base types and macros
pub use base_error::{CommonError, ServiceError};
pub use crate::{impl_from_common_error, impl_from_db_error};

// Re-export all error types
pub use sys_auth_error::AuthError;
pub use sys_user_error::UserError;
pub use sys_role_error::RoleError;
pub use sys_domain_error::DomainError;
pub use sys_access_key_error::AccessKeyError;
pub use sys_authorization_error::AuthorizationError;
