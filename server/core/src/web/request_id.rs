/**
 * 请求ID模块
 * 
 * 该模块提供了请求ID的生成和管理功能，用于跟踪和关联请求。
 * 主要功能包括：
 * - 请求ID生成
 * - 请求ID传递
 * - 请求追踪
 * - 日志关联
 * 
 * # 主要组件
 * 
 * ## RequestId
 * 请求ID类型，用于标识和追踪请求：
 * - 生成唯一ID
 * - 在请求间传递
 * - 关联日志和追踪信息
 * 
 * ## RequestIdLayer
 * 请求ID中间件层，用于处理请求ID：
 * - 生成请求ID
 * - 注入请求上下文
 * - 处理请求ID传递
 */

use axum::{
    extract::Request,
    http::{HeaderName, HeaderValue},
    response::Response,
};
use std::task::{Context, Poll};
use tower::{Layer, Service};
use uuid::Uuid;

/**
 * 请求ID类型
 * 
 * 用于标识和追踪请求的唯一标识符
 */
#[derive(Clone, Debug)]
pub struct RequestId(pub String);

/**
 * 请求ID中间件层
 * 
 * 用于处理请求ID的中间件层
 */
#[derive(Clone, Debug)]
pub struct RequestIdLayer;

impl RequestIdLayer {
    /**
     * 创建请求ID中间件层
     * 
     * # 返回值
     * 
     * 返回请求ID中间件层实例
     */
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for RequestIdLayer {
    type Service = RequestIdMiddleware<S>;

    /**
     * 创建请求ID中间件
     * 
     * # 参数
     * 
     * * `service` - 内部服务
     * 
     * # 返回值
     * 
     * 返回请求ID中间件实例
     */
    fn layer(&self, service: S) -> Self::Service {
        RequestIdMiddleware { service }
    }
}

/**
 * 请求ID中间件
 * 
 * 用于处理请求ID的中间件
 */
#[derive(Clone, Debug)]
pub struct RequestIdMiddleware<S> {
    service: S,
}

impl<S> Service<Request> for RequestIdMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + Clone + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    /**
     * 轮询服务
     * 
     * # 参数
     * 
     * * `cx` - 上下文
     * 
     * # 返回值
     * 
     * 返回轮询结果
     */
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    /**
     * 调用服务
     * 
     * # 参数
     * 
     * * `req` - 请求
     * 
     * # 返回值
     * 
     * 返回服务调用结果
     */
    fn call(&mut self, mut req: Request) -> Self::Future {
        let request_id = req
            .headers()
            .get("x-request-id")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        req.extensions_mut().insert(RequestId(request_id.clone()));

        let mut service = self.service.clone();
        Box::pin(async move {
            let mut response = service.call(req).await?;
            response
                .headers_mut()
                .insert(HeaderName::from_static("x-request-id"), HeaderValue::from_str(&request_id).unwrap());
            Ok(response)
        })
    }
}
