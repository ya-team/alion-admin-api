/**
 * 管理后台输出参数模块
 * 
 * 本模块聚合并导出所有管理后台相关的输出DTO类型，
 * 用于接口响应数据的结构化。
 * 
 * 主要包含：
 * - 认证相关输出（登录信息、用户信息、路由信息）
 * - 域名相关输出
 * - 接口树形结构输出
 * - 菜单相关输出（路由、树形结构、元数据）
 * - 用户相关输出（带域和组织信息、无密码信息）
 */

pub use sys_authentication::{AuthOutput, UserInfoOutput, UserRoute};
pub use sys_domain::DomainOutput;
pub use sys_endpoint::EndpointTree;
pub use sys_menu::{MenuRoute, MenuTree, RouteMeta};
pub use sys_user::{UserWithDomainAndOrgOutput, UserWithoutPassword};

mod sys_authentication;
mod sys_domain;
mod sys_endpoint;
mod sys_menu;
mod sys_user;
