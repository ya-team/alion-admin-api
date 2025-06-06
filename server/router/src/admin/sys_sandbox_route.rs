/**
 * 沙箱路由模块
 * 
 * 该模块提供了沙箱测试相关的路由功能，包括：
 * - 简单API密钥测试
 * - 复杂API密钥测试
 */

use axum::{routing::get, Router};
use server_api::admin::SysSandboxApi;
use super::route_constants::build_route_path;

/** 沙箱模块路径 */
const SANDBOX_PATH: &str = "/sandbox";
/** 简单API密钥测试路由路径 */
const ROUTE_SIMPLE_API_KEY: &str = "/simple-api-key";
/** 复杂API密钥测试路由路径 */
const ROUTE_COMPLEX_API_KEY: &str = "/complex-api-key";

/**
 * 沙箱路由结构体
 * 
 * 用于管理和注册沙箱测试相关的路由。
 */
#[derive(Debug)]
pub struct SysSandboxRouter;

impl SysSandboxRouter {
    /**
     * 初始化简单沙箱路由
     * 
     * 注册并返回简单API密钥测试相关的路由。
     * 
     * # 返回
     * * `Router` - 配置好的路由实例
     */
    pub async fn init_simple_sandbox_router() -> Router {
        let router = Router::new()
            .route(ROUTE_SIMPLE_API_KEY, get(SysSandboxApi::test_simple_api_key));

        Router::new().nest(&build_route_path(SANDBOX_PATH, ""), router)
    }

    /**
     * 初始化复杂沙箱路由
     * 
     * 注册并返回复杂API密钥测试相关的路由。
     * 
     * # 返回
     * * `Router` - 配置好的路由实例
     */
    pub async fn init_complex_sandbox_router() -> Router {
        let router = Router::new()
            .route(ROUTE_COMPLEX_API_KEY, get(SysSandboxApi::test_complex_api_key));

        Router::new().nest(&build_route_path(SANDBOX_PATH, ""), router)
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
