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

const USER_PATH: &str = "/user";
const SERVICE_NAME_USER: &str = "SysUserApi";
const ROUTE_USERS: &str = "/users";
const ROUTE_ADD_POLICIES: &str = "/add_policies";
const ROUTE_REMOVE_POLICIES: &str = "/remove_policies";

#[derive(Debug)]
pub struct SysUserRouter;

impl SysUserRouter {
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

    /// 注册用户相关的路由信息
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
