// 路由相关的公共常量定义

// API 基础路径
pub const API_BASE_PATH: &str = "/api";

// 服务名称常量
pub const SERVICE_NAME_AUTH: &str = "SysAuthenticationApi";
pub const SERVICE_NAME_AUTH_KEY: &str = "SysAccessKeyApi";
pub const SERVICE_NAME_DOMAIN: &str = "SysDomainApi";
pub const SERVICE_NAME_ENDPOINT: &str = "SysEndpointApi";
pub const SERVICE_NAME_LOGIN_LOG: &str = "SysLoginLogApi";
pub const SERVICE_NAME_MENU: &str = "SysMenuApi";
pub const SERVICE_NAME_OPERATION_LOG: &str = "SysOperationLogApi";
pub const SERVICE_NAME_ROLE: &str = "SysRoleApi";

// 路由路径常量
pub const ROUTE_ROOT: &str = "/";
pub const ROUTE_ID: &str = "/{id}";
pub const ROUTE_TREE: &str = "/tree";
pub const ROUTE_AUTH_ROUTE: &str = "/auth-route";
pub const ROUTE_CONSTANT_ROUTES: &str = "/constant-routes";

// 模块路径常量
pub const AUTH_PATH: &str = "/auth";
pub const AUTH_KEY_PATH: &str = "/auth-key";
pub const DOMAIN_PATH: &str = "/domain";
pub const ENDPOINT_PATH: &str = "/endpoint";
pub const LOGIN_LOG_PATH: &str = "/login-log";
pub const MENU_PATH: &str = "/menu";
pub const OPERATION_LOG_PATH: &str = "/operation-log";
pub const ROLE_PATH: &str = "/role";

// 辅助函数：构建完整路由路径
pub fn build_route_path(base_path: &str, route_path: &str) -> String {
    format!("{}{}{}", API_BASE_PATH, base_path, route_path)
} 