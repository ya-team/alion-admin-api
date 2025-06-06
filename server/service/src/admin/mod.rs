/// 管理后台服务模块
/// 
/// 该模块实现了管理后台的核心业务逻辑服务，包括：
/// - 用户管理：用户CRUD、认证、授权等
/// - 角色管理：角色CRUD、权限分配等
/// - 菜单管理：菜单CRUD、权限控制等
/// - 组织管理：组织CRUD、层级关系等
/// - 域名管理：域名CRUD、配置等
/// - 访问密钥管理：API密钥CRUD、验证等
/// - 日志管理：登录日志、操作日志等
/// 
/// 每个服务都实现了相应的trait接口，提供了统一的错误处理和事件通知机制。
/// 
/// # 主要组件
/// 
/// ## 核心服务
/// * `SysUserService`: 用户管理服务
/// * `SysRoleService`: 角色管理服务
/// * `SysMenuService`: 菜单管理服务
/// * `SysAuthService`: 认证服务
/// * `SysAuthorizationService`: 授权服务
/// 
/// ## 辅助服务
/// * `SysDomainService`: 域名管理服务
/// * `SysAccessKeyService`: 访问密钥服务
/// * `SysLoginLogService`: 登录日志服务
/// * `SysOperationLogService`: 操作日志服务
/// * `SysOrganizationService`: 组织管理服务
/// 
/// ## 事件处理
/// * `event_handlers`: 事件处理器
/// * `events`: 事件定义
/// 
/// # 使用示例
/// 
/// use server_service::admin::*;
/// 
/// // 创建用户服务实例
/// let user_service = SysUserService::new();
/// 
/// // 创建角色服务实例
/// let role_service = SysRoleService::new();
/// 
/// // 创建认证服务实例
/// let auth_service = SysAuthService::new();
/// 

pub use errors::*;
pub use server_model::admin::{
    entities::{
        prelude::{SysDomain, SysEndpoint, SysMenu, SysRole, SysUser},
        sys_access_key::Model as SysAccessKeyModel,
        sys_domain::Model as SysDomainModel,
        sys_endpoint::Model as SysEndpointModel,
        sys_login_log::Model as SysLoginLogModel,
        sys_menu::Model as SysMenuModel,
        sys_operation_log::Model as SysOperationLogModel,
        sys_organization::Model as SysOrganizationModel,
        sys_role::Model as SysRoleModel,
    },
    input::*,
    output::*,
};
pub use sys_access_key_service::{
    api_key_validate_listener, SysAccessKeyService, TAccessKeyService,
};
pub use sys_auth_service::{
    auth_login_listener, jwt_created_listener, SysAuthService, TAuthService,
};
pub use sys_authorization_service::{SysAuthorizationService, TAuthorizationService};
pub use sys_domain_service::{SysDomainService, TDomainService};
pub use sys_endpoint_service::{SysEndpointService, TEndpointService};
pub use sys_login_log_service::{SysLoginLogService, TLoginLogService};
pub use sys_menu_service::{SysMenuService, TMenuService};
pub use sys_operation_log_service::{
    sys_operation_log_listener, SysOperationLogService, TOperationLogService,
};
pub use sys_organization_service::{SysOrganizationService, TOrganizationService};
pub use sys_role_service::{SysRoleService, TRoleService};
pub use sys_user_service::{SysUserService, TUserService};
pub mod dto;
pub mod errors;
pub mod helper;
mod sys_access_key_service;
mod sys_auth_service;
mod sys_authorization_service;
mod sys_domain_service;
mod sys_endpoint_service;
mod sys_login_log_service;
mod sys_menu_service;
mod sys_operation_log_service;
mod sys_organization_service;
mod sys_role_service;
mod sys_user_service;

mod event_handlers;
mod events;
