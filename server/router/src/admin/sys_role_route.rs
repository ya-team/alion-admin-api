/**
 * 角色路由模块
 * 
 * 该模块提供了角色管理相关的路由功能，包括：
 * - 获取角色列表
 * - 创建角色
 * - 获取角色详情
 * - 更新角色
 * - 删除角色
 */

use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use server_api::admin::SysRoleApi;
use server_global::global::{add_route, RouteInfo};
use super::route_constants::{
    ROLE_PATH, SERVICE_NAME_ROLE, ROUTE_ROOT, ROUTE_ID, build_route_path,
};

/**
 * 角色路由结构体
 * 
 * 用于管理和注册角色相关的路由。
 */
#[derive(Debug)]
pub struct SysRoleRouter;

impl SysRoleRouter {
    /**
     * 初始化角色路由
     * 
     * 注册并返回角色相关的所有路由。
     * 
     * # 返回
     * * `Router` - 配置好的路由实例
     */
    pub async fn init_role_router() -> Router {
        // 注册路由信息到全局路由表
        Self::register_role_routes().await;

        // 构建路由
        let router = Router::new()
            .route(ROUTE_ROOT, get(SysRoleApi::find_paginated_roles))
            .route(ROUTE_ROOT, post(SysRoleApi::create_role))
            .route(ROUTE_ID, get(SysRoleApi::get_role))
            .route(ROUTE_ROOT, put(SysRoleApi::update_role))
            .route(ROUTE_ID, delete(SysRoleApi::delete_role));

        Router::new().nest(&build_route_path(ROLE_PATH, ""), router)
    }

    /**
     * 注册角色相关的路由信息
     * 
     * 将角色相关的路由信息注册到全局路由表中。
     */
    async fn register_role_routes() {
        let routes = [
            (ROUTE_ROOT, Method::GET, "获取角色列表"),
            (ROUTE_ROOT, Method::POST, "创建角色"),
            (ROUTE_ID, Method::GET, "获取角色详情"),
            (ROUTE_ROOT, Method::PUT, "更新角色"),
            (ROUTE_ID, Method::DELETE, "删除角色"),
        ];

        for (path, method, description) in routes {
            let route_info = RouteInfo::new(
                &build_route_path(ROLE_PATH, path),
                method,
                SERVICE_NAME_ROLE,
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
