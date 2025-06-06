/**
 * 域名路由模块
 * 
 * 该模块提供了域名管理相关的路由功能，包括：
 * - 获取域名列表
 * - 创建域名
 * - 获取域名详情
 * - 更新域名
 * - 删除域名
 */

use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use server_api::admin::SysDomainApi;
use server_global::global::{add_route, RouteInfo};
use super::route_constants::{
    DOMAIN_PATH, SERVICE_NAME_DOMAIN, ROUTE_ROOT, ROUTE_ID, build_route_path,
};

/**
 * 域名路由结构体
 * 
 * 用于管理和注册域名相关的路由。
 */
#[derive(Debug)]
pub struct SysDomainRouter;

impl SysDomainRouter {
    /**
     * 初始化域名路由
     * 
     * 注册并返回域名相关的所有路由。
     * 
     * # 返回
     * * `Router` - 配置好的路由实例
     */
    pub async fn init_domain_router() -> Router {
        // 注册路由信息到全局路由表
        Self::register_domain_routes().await;

        // 构建路由
        let router = Router::new()
            .route(ROUTE_ROOT, get(SysDomainApi::get_paginated_domains))
            .route(ROUTE_ROOT, post(SysDomainApi::create_domain))
            .route(ROUTE_ID, get(SysDomainApi::get_domain))
            .route(ROUTE_ROOT, put(SysDomainApi::update_domain))
            .route(ROUTE_ID, delete(SysDomainApi::delete_domain));

        Router::new().nest(&build_route_path(DOMAIN_PATH, ""), router)
    }

    /**
     * 注册域名相关的路由信息
     * 
     * 将域名相关的路由信息注册到全局路由表中。
     */
    async fn register_domain_routes() {
        let routes = [
            (ROUTE_ROOT, Method::GET, "获取域名列表"),
            (ROUTE_ROOT, Method::POST, "创建域名"),
            (ROUTE_ID, Method::GET, "获取域名详情"),
            (ROUTE_ROOT, Method::PUT, "更新域名"),
            (ROUTE_ID, Method::DELETE, "删除域名"),
        ];

        for (path, method, description) in routes {
            let route_info = RouteInfo::new(
                &build_route_path(DOMAIN_PATH, path),
                method,
                SERVICE_NAME_DOMAIN,
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
