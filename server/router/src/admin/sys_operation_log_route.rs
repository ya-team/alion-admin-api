/**
 * 操作日志路由模块
 * 
 * 该模块提供了操作日志相关的路由功能，包括：
 * - 获取操作日志列表
 */

use axum::{http::Method, routing::get, Router};
use server_api::admin::SysOperationLogApi;
use server_global::global::{add_route, RouteInfo};
use super::route_constants::{
    OPERATION_LOG_PATH, SERVICE_NAME_OPERATION_LOG, ROUTE_ROOT, build_route_path,
};

/**
 * 操作日志路由结构体
 * 
 * 用于管理和注册操作日志相关的路由。
 */
#[derive(Debug)]
pub struct SysOperationLogRouter;

impl SysOperationLogRouter {
    /**
     * 初始化操作日志路由
     * 
     * 注册并返回操作日志相关的所有路由。
     * 
     * # 返回
     * * `Router` - 配置好的路由实例
     */
    pub async fn init_operation_log_router() -> Router {
        // 注册路由信息到全局路由表
        Self::register_operation_log_routes().await;

        // 构建路由
        let router = Router::new()
            .route(ROUTE_ROOT, get(SysOperationLogApi::get_paginated_operation_logs));

        Router::new().nest(&build_route_path(OPERATION_LOG_PATH, ""), router)
    }

    /**
     * 注册操作日志相关的路由信息
     * 
     * 将操作日志相关的路由信息注册到全局路由表中。
     */
    async fn register_operation_log_routes() {
        let routes = [
            (ROUTE_ROOT, Method::GET, "获取操作日志列表"),
        ];

        for (path, method, description) in routes {
            let route_info = RouteInfo::new(
                &build_route_path(OPERATION_LOG_PATH, path),
                method,
                SERVICE_NAME_OPERATION_LOG,
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
