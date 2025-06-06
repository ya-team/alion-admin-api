use axum::{routing::get, Router};
use server_api::admin::SysSandboxApi;
use super::route_constants::build_route_path;

const SANDBOX_PATH: &str = "/sandbox";
const ROUTE_SIMPLE_API_KEY: &str = "/simple-api-key";
const ROUTE_COMPLEX_API_KEY: &str = "/complex-api-key";

#[derive(Debug)]
pub struct SysSandboxRouter;

impl SysSandboxRouter {
    pub async fn init_simple_sandbox_router() -> Router {
        let router = Router::new()
            .route(ROUTE_SIMPLE_API_KEY, get(SysSandboxApi::test_simple_api_key));

        Router::new().nest(&build_route_path(SANDBOX_PATH, ""), router)
    }

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
