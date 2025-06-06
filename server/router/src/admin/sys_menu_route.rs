/**
 * 菜单路由模块
 * 
 * 该模块提供了菜单管理相关的路由功能，包括：
 * - 获取常量路由
 * - 获取菜单树
 * - 获取菜单列表
 * - 创建菜单
 * - 获取菜单详情
 * - 更新菜单
 * - 删除菜单
 * - 获取角色菜单
 */

use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use server_api::admin::SysMenuApi;
use server_core::web::operation_log::OperationLogLayer;
use server_global::global::{add_route, RouteInfo};
use super::route_constants::{
    MENU_PATH, SERVICE_NAME_MENU, ROUTE_ROOT, ROUTE_ID, ROUTE_TREE,
    ROUTE_AUTH_ROUTE, ROUTE_CONSTANT_ROUTES, build_route_path,
};

/**
 * 菜单路由结构体
 * 
 * 用于管理和注册菜单相关的路由。
 */
#[derive(Debug)]
pub struct SysMenuRouter;

impl SysMenuRouter {
    /**
     * 初始化公开路由（无需认证）
     * 
     * 注册并返回无需认证即可访问的菜单路由。
     * 
     * # 返回
     * * `Router` - 配置好的路由实例
     */
    pub async fn init_menu_router() -> Router {
        let router = Router::new().route(
            ROUTE_CONSTANT_ROUTES,
            get(SysMenuApi::get_constant_routes).layer(OperationLogLayer::new(true)),
        );
        Router::new().nest(&build_route_path(MENU_PATH, ""), router)
    }

    /**
     * 初始化需要认证的路由
     * 
     * 注册并返回需要用户认证才能访问的菜单路由。
     * 
     * # 返回
     * * `Router` - 配置好的路由实例
     */
    pub async fn init_protected_menu_router() -> Router {
        // 注册路由信息到全局路由表
        Self::register_menu_routes().await;

        // 构建路由
        let router = Router::new()
            .route(ROUTE_TREE, get(SysMenuApi::tree_menu))
            .route(ROUTE_ROOT, get(SysMenuApi::get_menu_list))
            .route(ROUTE_ROOT, post(SysMenuApi::create_menu))
            .route(ROUTE_ID, get(SysMenuApi::get_menu))
            .route(ROUTE_ROOT, put(SysMenuApi::update_menu))
            .route(ROUTE_ID, delete(SysMenuApi::delete_menu))
            .route(ROUTE_AUTH_ROUTE, get(SysMenuApi::get_constant_routes));

        Router::new().nest(&build_route_path(MENU_PATH, ""), router)
    }

    /**
     * 注册菜单相关的路由信息
     * 
     * 将菜单相关的路由信息注册到全局路由表中。
     */
    async fn register_menu_routes() {
        let routes = [
            (ROUTE_TREE, Method::GET, "获取菜单树"),
            (ROUTE_ROOT, Method::GET, "获取菜单列表"),
            (ROUTE_ROOT, Method::POST, "创建菜单"),
            (ROUTE_ID, Method::GET, "获取菜单详情"),
            (ROUTE_ROOT, Method::PUT, "更新菜单"),
            (ROUTE_ID, Method::DELETE, "删除菜单"),
            (ROUTE_AUTH_ROUTE, Method::GET, "获取角色菜单"),
        ];

        for (path, method, description) in routes {
            let route_info = RouteInfo::new(
                &build_route_path(MENU_PATH, path),
                method,
                SERVICE_NAME_MENU,
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
