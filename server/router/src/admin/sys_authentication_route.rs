use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use server_api::admin::SysAuthenticationApi;
use server_global::global::{add_route, RouteInfo};
use super::route_constants::{
    AUTH_PATH, SERVICE_NAME_AUTH, build_route_path,
};

// 路由路径常量
const ROUTE_LOGIN: &str = "/login";
const ROUTE_USER_INFO: &str = "/user-info";
const ROUTE_USER_ROUTES: &str = "/user-routes";
const ROUTE_ASSIGN_PERMISSION: &str = "/assign-permission";
const ROUTE_ASSIGN_ROUTES: &str = "/assign-routes";

#[derive(Debug)]
pub struct SysAuthenticationRouter;

impl SysAuthenticationRouter {
    /// 初始化公开路由（无需认证）
    pub async fn init_authentication_router() -> Router {
        let auth_router = Router::new()
            .route(ROUTE_LOGIN, post(SysAuthenticationApi::login_handler));

        Router::new().nest(&build_route_path(AUTH_PATH, ""), auth_router)
    }

    /// 初始化需要认证的路由
    pub async fn init_protected_router() -> Router {
        let auth_router = Router::new()
            .route(ROUTE_USER_INFO, get(SysAuthenticationApi::get_user_info))
            .route(ROUTE_USER_ROUTES, get(SysAuthenticationApi::get_user_routes));

        Router::new().nest(&build_route_path(AUTH_PATH, ""), auth_router)
    }

    /// 初始化需要授权的路由
    pub async fn init_authorization_router() -> Router {
        // 注册路由信息到全局路由表
        Self::register_authorization_routes().await;

        // 构建授权路由
        let auth_router = Router::new()
            .route(ROUTE_ASSIGN_PERMISSION, post(SysAuthenticationApi::assign_permissions))
            .route(ROUTE_ASSIGN_ROUTES, post(SysAuthenticationApi::assign_routes));

        Router::new().nest(&build_route_path(AUTH_PATH, ""), auth_router)
    }

    /// 注册授权相关的路由信息
    async fn register_authorization_routes() {
        let routes = [
            (ROUTE_ASSIGN_PERMISSION, "分配权限"),
            (ROUTE_ASSIGN_ROUTES, "分配路由"),
        ];

        for (path, description) in routes {
            let route_info = RouteInfo::new(
                &build_route_path(AUTH_PATH, path),
                Method::POST,
                SERVICE_NAME_AUTH,
                description,
            );
            add_route(route_info).await;
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_variables)]
    #[test]
    fn test_route_paths() {
        // 这里可以添加路由测试逻辑
        // 例如验证路由是否正确注册，路径是否正确等
    }
}
