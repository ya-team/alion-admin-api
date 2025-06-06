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
