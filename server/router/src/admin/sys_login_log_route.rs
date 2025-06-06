use axum::{http::Method, routing::get, Router};
use server_api::admin::SysLoginLogApi;
use server_global::global::{add_route, RouteInfo};
use super::route_constants::{
    LOGIN_LOG_PATH, SERVICE_NAME_LOGIN_LOG, ROUTE_ROOT, build_route_path,
};

#[derive(Debug)]
pub struct SysLoginLogRouter;

impl SysLoginLogRouter {
    pub async fn init_login_log_router() -> Router {
        // 注册路由信息到全局路由表
        Self::register_login_log_routes().await;

        // 构建路由
        let router = Router::new()
            .route(ROUTE_ROOT, get(SysLoginLogApi::get_paginated_login_logs));

        Router::new().nest(&build_route_path(LOGIN_LOG_PATH, ""), router)
    }

    /// 注册登录日志相关的路由信息
    async fn register_login_log_routes() {
        let routes = [
            (ROUTE_ROOT, Method::GET, "获取登录日志列表"),
        ];

        for (path, method, description) in routes {
            let route_info = RouteInfo::new(
                &build_route_path(LOGIN_LOG_PATH, path),
                method,
                SERVICE_NAME_LOGIN_LOG,
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
