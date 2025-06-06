/**
 * 路由常量定义模块
 * 
 * 该模块定义了所有路由相关的常量，包括：
 * - API基础路径
 * - 服务名称常量
 * - 路由路径常量
 * - 模块路径常量
 */

/** API基础路径 */
pub const API_BASE_PATH: &str = "/api";

/** 认证服务名称 */
pub const SERVICE_NAME_AUTH: &str = "SysAuthenticationApi";
/** 访问密钥服务名称 */
pub const SERVICE_NAME_AUTH_KEY: &str = "SysAccessKeyApi";
/** 域名服务名称 */
pub const SERVICE_NAME_DOMAIN: &str = "SysDomainApi";
/** 接口服务名称 */
pub const SERVICE_NAME_ENDPOINT: &str = "SysEndpointApi";
/** 登录日志服务名称 */
pub const SERVICE_NAME_LOGIN_LOG: &str = "SysLoginLogApi";
/** 菜单服务名称 */
pub const SERVICE_NAME_MENU: &str = "SysMenuApi";
/** 操作日志服务名称 */
pub const SERVICE_NAME_OPERATION_LOG: &str = "SysOperationLogApi";
/** 角色服务名称 */
pub const SERVICE_NAME_ROLE: &str = "SysRoleApi";

/** 根路由路径 */
pub const ROUTE_ROOT: &str = "/";
/** ID路由路径 */
pub const ROUTE_ID: &str = "/{id}";
/** 树形结构路由路径 */
pub const ROUTE_TREE: &str = "/tree";
/** 认证路由路径 */
pub const ROUTE_AUTH_ROUTE: &str = "/auth-route";
/** 常量路由路径 */
pub const ROUTE_CONSTANT_ROUTES: &str = "/constant-routes";

/** 认证模块路径 */
pub const AUTH_PATH: &str = "/auth";
/** 访问密钥模块路径 */
pub const AUTH_KEY_PATH: &str = "/auth-key";
/** 域名模块路径 */
pub const DOMAIN_PATH: &str = "/domain";
/** 接口模块路径 */
pub const ENDPOINT_PATH: &str = "/endpoint";
/** 登录日志模块路径 */
pub const LOGIN_LOG_PATH: &str = "/login-log";
/** 菜单模块路径 */
pub const MENU_PATH: &str = "/menu";
/** 操作日志模块路径 */
pub const OPERATION_LOG_PATH: &str = "/operation-log";
/** 角色模块路径 */
pub const ROLE_PATH: &str = "/role";

/**
 * 构建完整路由路径
 * 
 * 将基础路径、模块路径和路由路径组合成完整的API路径。
 * 
 * # 参数
 * * `base_path` - 模块基础路径
 * * `route_path` - 具体路由路径
 * 
 * # 返回
 * * `String` - 完整的API路径
 */
pub fn build_route_path(base_path: &str, route_path: &str) -> String {
    format!("{}{}{}", API_BASE_PATH, base_path, route_path)
} 