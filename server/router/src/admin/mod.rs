/**
 * 管理后台路由模块
 * 
 * 该模块包含了所有管理后台相关的路由定义，包括：
 * - 认证相关路由（登录、用户信息等）
 * - 访问密钥管理路由
 * - 域名管理路由
 * - 接口管理路由
 * - 登录日志路由
 * - 菜单管理路由
 * - 操作日志路由
 * - 组织管理路由
 * - 角色管理路由
 * - 沙箱测试路由
 * - 用户管理路由
 */

pub use sys_access_key_route::SysAccessKeyRouter;
pub use sys_authentication_route::SysAuthenticationRouter;
pub use sys_domain_route::SysDomainRouter;
pub use sys_endpoint_route::SysEndpointRouter;
pub use sys_login_log_route::SysLoginLogRouter;
pub use sys_menu_route::SysMenuRouter;
pub use sys_operation_log_route::SysOperationLogRouter;
pub use sys_organization_route::SysOrganizationRouter;
pub use sys_role_route::SysRoleRouter;
pub use sys_sandbox_route::SysSandboxRouter;
pub use sys_user_route::SysUserRouter;

mod sys_access_key_route;
mod sys_authentication_route;
mod sys_domain_route;
mod sys_endpoint_route;
mod sys_login_log_route;
mod sys_menu_route;
mod sys_operation_log_route;
mod sys_organization_route;
mod sys_role_route;
mod sys_sandbox_route;
mod sys_user_route;

pub mod route_constants;
