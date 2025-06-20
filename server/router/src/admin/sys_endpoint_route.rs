/**
 * 接口路由模块
 * 
 * 该模块提供了接口管理相关的路由功能，包括：
 * - 获取接口列表
 * - 获取角色API权限
 * - 获取接口树
 */

use axum::{http::Method, routing::get, Router};
use server_api::admin::SysEndpointApi;
use server_global::global::{add_route, RouteInfo};
use super::route_constants::{
    ENDPOINT_PATH, SERVICE_NAME_ENDPOINT, ROUTE_ROOT, ROUTE_TREE, build_route_path,
};

/** 角色API权限路由路径 */
const ROUTE_AUTH_ENDPOINT: &str = "/auth-endpoint/{roleCode}";

/**
 * 接口路由结构体
 * 
 * 用于管理和注册接口相关的路由。
 */
#[derive(Debug)]
pub struct SysEndpointRouter;

impl SysEndpointRouter {
    /**
     * 初始化接口路由
     * 
     * 注册并返回接口相关的所有路由。
     * 
     * # 返回
     * * `Router` - 配置好的路由实例
     */
    pub async fn init_endpoint_router() -> Router {
        // 注册路由信息到全局路由表
        Self::register_endpoint_routes().await;

        // 构建路由
        let router = Router::new()
            .route(ROUTE_ROOT, get(SysEndpointApi::get_paginated_endpoints))
            .route(ROUTE_AUTH_ENDPOINT, get(SysEndpointApi::get_auth_endpoints))
            .route(ROUTE_TREE, get(SysEndpointApi::tree_endpoint));

        Router::new().nest(&build_route_path(ENDPOINT_PATH, ""), router)
    }

    /**
     * 注册接口相关的路由信息
     * 
     * 将接口相关的路由信息注册到全局路由表中。
     */
    async fn register_endpoint_routes() {
        let routes = [
            (ROUTE_ROOT, Method::GET, "获取接口列表"),
            (ROUTE_AUTH_ENDPOINT, Method::GET, "获取角色API权限"),
            (ROUTE_TREE, Method::GET, "获取接口树"),
        ];

        for (path, method, description) in routes {
            let route_info = RouteInfo::new(
                &build_route_path(ENDPOINT_PATH, path),
                method,
                SERVICE_NAME_ENDPOINT,
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
