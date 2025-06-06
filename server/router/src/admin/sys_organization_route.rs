use axum::{http::Method, routing::get, Router};
use server_api::admin::SysOrganizationApi;
use server_global::global::{add_route, RouteInfo};
use super::route_constants::{
    ROUTE_ROOT, build_route_path,
};

const ORG_PATH: &str = "/org";
const SERVICE_NAME_ORG: &str = "SysOrganizationApi";

#[derive(Debug)]
pub struct SysOrganizationRouter;

impl SysOrganizationRouter {
    pub async fn init_organization_router() -> Router {
        // 注册路由信息到全局路由表
        Self::register_organization_routes().await;

        // 构建路由
        let router = Router::new()
            .route(ROUTE_ROOT, get(SysOrganizationApi::get_paginated_organizations));

        Router::new().nest(&build_route_path(ORG_PATH, ""), router)
    }

    /// 注册组织相关的路由信息
    async fn register_organization_routes() {
        let routes = [
            (ROUTE_ROOT, Method::GET, "获取组织列表"),
        ];

        for (path, method, description) in routes {
            let route_info = RouteInfo::new(
                &build_route_path(ORG_PATH, path),
                method,
                SERVICE_NAME_ORG,
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
