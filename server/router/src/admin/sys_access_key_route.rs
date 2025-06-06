use axum::{
    http::Method,
    routing::{delete, get, post},
    Router,
};
use server_api::admin::SysAccessKeyApi;
use server_global::global::{add_route, RouteInfo};
use super::route_constants::{
    AUTH_KEY_PATH, SERVICE_NAME_AUTH_KEY, ROUTE_ROOT, ROUTE_ID, build_route_path,
};

#[derive(Debug)]
pub struct SysAccessKeyRouter;

impl SysAccessKeyRouter {
    pub async fn init_access_key_router() -> Router {
        // 注册路由信息到全局路由表
        Self::register_access_key_routes().await;

        // 构建路由
        let router = Router::new()
            .route(ROUTE_ROOT, get(SysAccessKeyApi::get_paginated_access_keys))
            .route(ROUTE_ROOT, post(SysAccessKeyApi::create_access_key))
            .route(ROUTE_ID, delete(SysAccessKeyApi::delete_access_key));

        Router::new().nest(&build_route_path(AUTH_KEY_PATH, ""), router)
    }

    /// 注册访问密钥相关的路由信息
    async fn register_access_key_routes() {
        let routes = [
            (ROUTE_ROOT, Method::GET, "获取访问密钥列表"),
            (ROUTE_ROOT, Method::POST, "创建访问密钥"),
            (ROUTE_ID, Method::DELETE, "删除访问密钥"),
        ];

        for (path, method, description) in routes {
            let route_info = RouteInfo::new(
                &build_route_path(AUTH_KEY_PATH, path),
                method,
                SERVICE_NAME_AUTH_KEY,
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
