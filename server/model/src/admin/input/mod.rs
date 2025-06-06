/**
 * 管理后台输入参数模块
 * 
 * 本模块聚合并导出所有管理后台相关的输入DTO类型，
 * 用于接口请求参数的数据校验与结构化。
 * 
 * 主要包含：
 * - 访问密钥相关输入
 * - 认证/登录相关输入
 * - 授权相关输入
 * - 域名、接口、菜单、角色、用户等管理输入
 */

pub use sys_access_key::{AccessKeyPageRequest, CreateAccessKeyInput};
pub use sys_authentication::LoginInput;
pub use sys_authorization::{AssignPermissionDto, AssignRouteDto, AssignUserDto};
pub use sys_domain::{CreateDomainInput, DomainPageRequest, UpdateDomainInput};
pub use sys_endpoint::EndpointPageRequest;
pub use sys_login_log::LoginLogPageRequest;
pub use sys_menu::{MenuInput, MenuPageRequest, CreateMenuInput, UpdateMenuInput};
pub use sys_operation_log::OperationLogPageRequest;
pub use sys_organization::OrganizationPageRequest;
pub use sys_role::{CreateRoleInput, RolePageRequest, UpdateRoleInput};
pub use sys_user::{CreateUserInput, UpdateUserInput, UserPageRequest};

mod sys_access_key;
mod sys_authentication;
mod sys_authorization;
mod sys_domain;
mod sys_endpoint;
mod sys_login_log;
mod sys_menu;
mod sys_operation_log;
mod sys_organization;
mod sys_role;
mod sys_user;
