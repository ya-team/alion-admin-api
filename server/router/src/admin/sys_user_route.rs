/**
 * 用户路由模块
 * 
 * 该模块提供了用户管理相关的路由功能，包括：
 * - 获取所有用户
 * - 获取用户列表
 * - 创建用户
 * - 获取用户详情
 * - 更新用户
 * - 删除用户
 * - 添加用户策略
 * - 删除用户策略
 */

use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use server_api::admin::SysUserApi;
use server_global::global::{add_route, RouteInfo};
use super::route_constants::{
    ROUTE_ROOT, ROUTE_ID, build_route_path,
};

/** 用户模块路径 */
const USER_PATH: &str = "/user";
/** 用户服务名称 */
const SERVICE_NAME_USER: &str = "SysUserApi";
/** 所有用户路由路径 */
const ROUTE_USERS: &str = "/users";
/** 添加策略路由路径 */
const ROUTE_ADD_POLICIES: &str = "/add_policies";
/** 删除策略路由路径 */
const ROUTE_REMOVE_POLICIES: &str = "/remove_policies";

/**
 * 用户路由结构体
 * 
 * 用于管理和注册用户相关的路由。
 */
#[derive(Debug)]
pub struct SysUserRouter;

impl SysUserRouter {
    /**
     * 初始化用户路由
     * 
     * 注册并返回用户相关的所有路由。
     * 
     * # 返回
     * * `Router` - 配置好的路由实例
     */
    pub async fn init_user_router() -> Router {
        // 注册路由信息到全局路由表
        Self::register_user_routes().await;

        // 构建路由
        let router = Router::new()
            .route(ROUTE_USERS, get(SysUserApi::get_all_users))
            .route(ROUTE_ROOT, get(SysUserApi::get_paginated_users))
            .route(ROUTE_ROOT, post(SysUserApi::create_user))
            .route(ROUTE_ID, get(SysUserApi::get_user))
            .route(ROUTE_ROOT, put(SysUserApi::update_user))
            .route(ROUTE_ID, delete(SysUserApi::delete_user))
            .route(ROUTE_ADD_POLICIES, get(SysUserApi::add_policies))
            .route(ROUTE_REMOVE_POLICIES, get(SysUserApi::remove_policies));

        Router::new().nest(&build_route_path(USER_PATH, ""), router)
    }

    /**
     * 注册用户相关的路由信息
     * 
     * 将用户相关的路由信息注册到全局路由表中。
     */
    async fn register_user_routes() {
        let routes = [
            (ROUTE_USERS, Method::GET, "获取所有用户"),
            (ROUTE_ROOT, Method::GET, "获取用户列表"),
            (ROUTE_ROOT, Method::POST, "创建用户"),
            (ROUTE_ID, Method::GET, "获取用户详情"),
            (ROUTE_ROOT, Method::PUT, "更新用户"),
            (ROUTE_ID, Method::DELETE, "删除用户"),
            (ROUTE_ADD_POLICIES, Method::GET, "添加用户策略"),
            (ROUTE_REMOVE_POLICIES, Method::GET, "删除用户策略"),
        ];

        for (path, method, description) in routes {
            let route_info = RouteInfo::new(
                &build_route_path(USER_PATH, path),
                method,
                SERVICE_NAME_USER,
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
