/**
 * 管理后台API模块
 * 
 * 该模块包含所有管理后台相关的API实现，包括：
 * - 访问密钥管理 (SysAccessKeyApi)
 * - 认证管理 (SysAuthenticationApi)
 * - 域管理 (SysDomainApi)
 * - 端点管理 (SysEndpointApi)
 * - 登录日志管理 (SysLoginLogApi)
 * - 菜单管理 (SysMenuApi)
 * - 操作日志管理 (SysOperationLogApi)
 * - 组织管理 (SysOrganizationApi)
 * - 角色管理 (SysRoleApi)
 * - 沙箱管理 (SysSandboxApi)
 * - 用户管理 (SysUserApi)
 * 
 * 每个API模块都实现了相应的业务逻辑，并通过统一的错误处理和响应格式
 * 提供RESTful接口服务。
 */

pub mod sys_access_key_api;
pub mod sys_authentication_api;
pub mod sys_domain_api;
pub mod sys_endpoint_api;
pub mod sys_login_log_api;
pub mod sys_menu_api;
pub mod sys_operation_log_api;
pub mod sys_organization_api;
pub mod sys_role_api;
pub mod sys_sandbox_api;
pub mod sys_user_api;

pub use sys_access_key_api::SysAccessKeyApi;
pub use sys_authentication_api::SysAuthenticationApi;
pub use sys_domain_api::SysDomainApi;
pub use sys_endpoint_api::SysEndpointApi;
pub use sys_login_log_api::SysLoginLogApi;
pub use sys_menu_api::SysMenuApi;
pub use sys_operation_log_api::SysOperationLogApi;
pub use sys_organization_api::SysOrganizationApi;
pub use sys_role_api::SysRoleApi;
pub use sys_sandbox_api::SysSandboxApi;
pub use sys_user_api::SysUserApi;
