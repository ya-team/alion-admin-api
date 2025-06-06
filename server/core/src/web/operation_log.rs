/**
 * 操作日志模块
 * 
 * 该模块提供了操作日志的记录和查询功能，用于跟踪系统中的重要操作。
 * 主要功能包括：
 * - 记录用户操作
 * - 查询操作历史
 * - 导出操作日志
 * - 日志分析
 * 
 * # 主要组件
 * 
 * ## OperationLog
 * 操作日志实体，包含以下字段：
 * - id: 日志ID
 * - user_id: 操作用户ID
 * - username: 操作用户名
 * - operation: 操作类型
 * - method: 请求方法
 * - path: 请求路径
 * - params: 请求参数
 * - ip: 操作IP
 * - status: 操作状态
 * - error_msg: 错误信息
 * - created_at: 创建时间
 * 
 * ## OperationLogService
 * 操作日志服务，提供以下功能：
 * - 记录操作日志
 * - 查询操作日志
 * - 导出操作日志
 * - 清理过期日志
 */

use std::{
    collections::HashMap,
    convert::Infallible,
    fmt,
    net::SocketAddr,
    task::{Context, Poll},
};

use axum::{
    body::{to_bytes, Body, Bytes},
    extract::{ConnectInfo, Request},
    response::Response,
};
use bytes::BytesMut;
use chrono::Local;
use futures::{future::BoxFuture, StreamExt};
use http::{Extensions, HeaderMap, Uri};
use serde_json::Value;
use server_constant::definition::consts::SystemEvent;
use server_global::global::{self, OperationLogContext};
use tower_layer::Layer;
use tower_service::Service;

use super::{auth::User, RequestId};

/**
 * 用户代理请求头名称
 */
const USER_AGENT_HEADER: &str = "user-agent";

/**
 * 未知请求ID的默认值
 */
const UNKNOWN_REQUEST_ID: &str = "unknown";

/**
 * 默认请求体缓冲区大小（16KB）
 */
const DEFAULT_BODY_CAPACITY: usize = 1024 * 16;

/**
 * 操作日志层，用于创建操作日志中间件
 * 
 * 控制操作日志的启用/禁用状态，并提供创建中间件的功能。
 * 通过Layer trait实现，可以方便地集成到tower服务栈中。
 */
#[derive(Clone)]
pub struct OperationLogLayer {
    /**
     * 是否启用操作日志
     */
    pub enabled: bool,
}

impl OperationLogLayer {
    /**
     * 创建新的操作日志层
     * 
     * # 参数
     * * `enabled` - 是否启用操作日志
     * 
     * # 返回
     * * `Self` - 新的操作日志层实例
     */
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl<S> Layer<S> for OperationLogLayer
where
    S: Service<Request<Body>, Response = Response<Body>, Error = Infallible>
        + Send
        + Clone
        + 'static,
{
    type Service = OperationLogMiddleware<S>;

    /**
     * 创建操作日志中间件
     * 
     * 包装内部服务，添加操作日志功能。
     * 
     * # 参数
     * * `service` - 内部服务
     * 
     * # 返回
     * * `Self::Service` - 新的操作日志中间件
     */
    fn layer(&self, service: S) -> Self::Service {
        OperationLogMiddleware {
            inner: service,
            enabled: self.enabled,
        }
    }
}

/**
 * 操作日志中间件，用于记录请求和响应信息
 * 
 * 实现tower::Service trait，在请求处理过程中记录操作日志。
 * 记录的信息包括：
 * - 请求和响应内容
 * - 用户信息
 * - 时间信息
 * - 性能指标
 * 
 * # 类型参数
 * 
 * * `S`: 内部服务的类型，必须实现Service trait
 */
#[derive(Clone)]
pub struct OperationLogMiddleware<S> {
    /**
     * 内部服务
     */
    inner: S,
    /**
     * 是否启用操作日志
     */
    enabled: bool,
}

impl<S> Service<Request<Body>> for OperationLogMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<Body>, Error = Infallible>
        + Send
        + Clone
        + 'static,
    S::Future: Send + 'static,
{
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = Response<Body>;

    /**
     * 检查服务是否准备好处理请求
     * 
     * 检查内部服务是否准备好处理请求。
     * 
     * # 参数
     * * `cx` - 任务上下文
     * 
     * # 返回
     * * `Poll<Result<(), Self::Error>>` - 服务就绪状态
     */
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    /**
     * 处理请求并记录操作日志
     * 
     * 处理请求并记录操作日志，包括：
     * - 记录请求开始时间
     * - 收集请求信息（方法、URI、头信息等）
     * - 处理请求
     * - 记录响应信息
     * - 计算处理时间
     * - 发送操作日志事件
     * 
     * # 参数
     * * `req` - HTTP请求
     * 
     * # 返回
     * * `Self::Future` - 异步处理结果
     */
    fn call(&mut self, req: Request<Body>) -> Self::Future {
        if !self.enabled {
            let mut inner = self.inner.clone();
            return Box::pin(async move { inner.call(req).await });
        }

        let mut inner = self.inner.clone();
        Box::pin(async move {
            let start_time = Local::now().naive_local();
            let (parts, body) = req.into_parts();
            let headers = &parts.headers;
            let extensions = &parts.extensions;

            let (user_id, username, domain) = get_user_info(extensions);

            let request_id = extensions
                .get::<RequestId>()
                .map(ToString::to_string)
                .unwrap_or_else(|| UNKNOWN_REQUEST_ID.to_string());

            if let Ok(bytes) = buffer_body(body).await {
                let method = parts.method.to_string();
                let uri = parts.uri.to_string();
                let ip = get_client_ip(extensions, headers);
                let user_agent = get_user_agent(headers);
                let params = parse_query_params(&parts.uri);

                let req = Request::from_parts(parts, Body::from(bytes.clone()));
                let response = inner.call(req).await?;

                let (response_parts, response_body) = response.into_parts();
                let response_bytes = to_bytes(response_body, usize::MAX)
                    .await
                    .unwrap_or_default();

                let end_time = Local::now().naive_local();
                let duration = (end_time - start_time).num_milliseconds() as i32;

                let context = OperationLogContext {
                    user_id,
                    username,
                    domain,
                    module_name: "TODO".to_string(),
                    description: "TODO".to_string(),
                    request_id,
                    method,
                    url: uri,
                    ip,
                    user_agent,
                    params,
                    body: (!bytes.is_empty())
                        .then(|| serde_json::from_slice(&bytes).ok())
                        .flatten(),
                    response: serde_json::from_slice(&response_bytes).ok(),
                    start_time,
                    end_time,
                    duration,
                    created_at: start_time,
                };

                global::send_dyn_event(
                    SystemEvent::AuditOperationLoggedEvent.as_ref(),
                    Box::new(context),
                );

                Ok(Response::from_parts(
                    response_parts,
                    Body::from(response_bytes),
                ))
            } else {
                let mut inner = inner;
                inner.call(Request::from_parts(parts, Body::empty())).await
            }
        })
    }
}

/**
 * 缓冲请求体，带容量限制
 *
 * 读取请求体内容并存储在内存中，同时限制最大大小。
 * 如果请求体超过最大限制，将返回错误。
 *
 * # 参数
 * * `body` - 请求体，类型为 axum 的 Body
 *
 * # 返回值
 * * `Result<Bytes, Box<dyn std::error::Error + Send + Sync>>` -
 *   成功返回缓冲的字节数据，失败返回错误
 *
 * # 错误
 * * 当请求体大小超过 MAX_SIZE (32KB) 时返回错误
 * * 当读取请求体流失败时返回错误
 */
#[inline]
async fn buffer_body(body: Body) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
    const MAX_SIZE: usize = DEFAULT_BODY_CAPACITY * 2;
    let mut bytes = BytesMut::with_capacity(DEFAULT_BODY_CAPACITY);

    let mut stream = body.into_data_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        if bytes.len() + chunk.len() > MAX_SIZE {
            return Err("Request body too large".into());
        }
        bytes.extend_from_slice(&chunk);
    }

    Ok(bytes.freeze())
}

/**
 * 从请求头获取用户代理
 *
 * 从请求头中提取User-Agent信息。
 *
 * # 参数
 * * `headers` - HTTP 请求头映射
 *
 * # 返回值
 * * `Option<String>` - 成功返回用户代理字符串，未找到或解析失败返回 None
 */
#[inline(always)]
fn get_user_agent(headers: &HeaderMap) -> Option<String> {
    headers
        .get(USER_AGENT_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned)
}

/**
 * 获取客户端 IP，优先从扩展中获取
 *
 * 按照以下优先级获取客户端IP：
 * 1. 从请求扩展中获取（如果存在）
 * 2. 从请求头中获取
 * 3. 从Socket地址中获取
 *
 * # 参数
 * * `extensions` - 请求扩展
 * * `headers` - HTTP 请求头映射
 *
 * # 返回值
 * * `String` - 客户端IP地址
 */
#[inline(always)]
fn get_client_ip(extensions: &Extensions, headers: &HeaderMap) -> String {
    // 首先尝试从扩展中获取
    if let Some(ConnectInfo(addr)) = extensions.get::<ConnectInfo<SocketAddr>>() {
        return addr.ip().to_string();
    }

    // 然后尝试从请求头中获取
    let ip_headers = [
        "X-Real-IP",
        "X-Forwarded-For",
        "CF-Connecting-IP",
        "True-Client-IP",
        "X-Client-IP",
        "Fastly-Client-IP",
        "X-Cluster-Client-IP",
        "X-Original-Forwarded-For",
    ];

    for header_name in ip_headers {
        if let Some(ip_header) = headers.get(header_name) {
            if let Ok(ip_str) = ip_header.to_str() {
                let real_ip = ip_str.split(',').next().unwrap_or("").trim();
                if !real_ip.is_empty() {
                    return real_ip.to_string();
                }
            }
        }
    }

    // 如果都没有找到，返回unknown
    "unknown".to_string()
}

/**
 * 从请求扩展中获取用户信息
 * 
 * # 参数
 * * `extensions` - 请求扩展
 * 
 * # 返回
 * * `(Option<String>, Option<String>, Option<String>)` - 用户ID、用户名和域名
 */
fn get_user_info(extensions: &Extensions) -> (Option<String>, Option<String>, Option<String>) {
    extensions
        .get::<User>()
        .map(|user| {
            (
                Some(user.user_id()),
                Some(user.username()),
                Some(user.domain()),
            )
        })
        .unwrap_or((None, None, None))
}

/**
 * 解析URI中的查询参数
 *
 * 将URI中的查询参数解析为JSON对象。
 *
 * # 参数
 * * `uri` - HTTP URI
 *
 * # 返回值
 * * `Option<Value>` - 解析后的查询参数，如果没有查询参数则返回None
 */
#[inline(always)]
fn parse_query_params(uri: &Uri) -> Option<Value> {
    uri.query().map(|query| {
        let params: HashMap<_, _> = query
            .split('&')
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                let key = parts.next()?;
                let value = parts.next().unwrap_or("");
                Some((key.to_string(), value.to_string()))
            })
            .collect();
        serde_json::to_value(params).unwrap_or_default()
    })
}

impl fmt::Display for RequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Method;
    use crate::web::auth::User;
    use serde_json::json;

    /**
     * 创建测试用户
     * 
     * # 返回
     * * `User` - 测试用户实例
     */
    fn create_test_user() -> User {
        User::new(
            "test_user_id".to_string(),
            "test_username".to_string(),
            "test_domain".to_string(),
        )
    }

    /**
     * 创建测试请求
     * 
     * # 参数
     * * `method` - HTTP方法
     * * `uri` - 请求URI
     * * `body` - 请求体
     * 
     * # 返回
     * * `Request<Body>` - 测试请求实例
     */
    fn create_request(method: Method, uri: &str, body: Option<Value>) -> Request<Body> {
        let mut req = Request::builder()
            .method(method.clone())
            .uri(uri)
            .body(Body::empty())
            .unwrap();

        if let Some(body) = body {
            req = Request::builder()
                .method(method)
                .uri(uri)
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap();
        }

        req.extensions_mut().insert(create_test_user());
        req.extensions_mut().insert(RequestId("test_request_id".to_string()));
        req
    }

    /**
     * 断言操作日志上下文
     * 
     * # 参数
     * * `method` - HTTP方法
     * * `uri` - 请求URI
     * * `params` - 查询参数
     * * `body` - 请求体
     */
    async fn assert_context(method: &str, uri: &str, params: Option<Value>, body: Option<Value>) {
        let req = create_request(
            Method::from_bytes(method.as_bytes()).unwrap(),
            uri,
            body.clone(),
        );

        let mut service = OperationLogMiddleware {
            inner: tower::service_fn(|_req: Request<Body>| async move {
                Ok::<_, Infallible>(Response::new(Body::empty()))
            }),
            enabled: true,
        };

        let _ = service.call(req).await;

        let context = global::OperationLogContext::get().await.unwrap();
        assert_eq!(context.method, method);
        assert_eq!(context.url, uri);
        assert_eq!(context.params, params);
        assert_eq!(context.body, body);
    }

    /**
     * 测试操作日志中间件
     */
    #[tokio::test]
    async fn test_operation_log_middleware() {
        // 测试GET请求
        assert_context(
            "GET",
            "/api/test?param1=value1&param2=value2",
            Some(json!({
                "param1": "value1",
                "param2": "value2"
            })),
            None,
        )
        .await;

        // 测试POST请求
        assert_context(
            "POST",
            "/api/test",
            None,
            Some(json!({
                "key": "value"
            })),
        )
        .await;

        // 测试PUT请求
        assert_context(
            "PUT",
            "/api/test/1",
            None,
            Some(json!({
                "name": "test"
            })),
        )
        .await;

        // 测试DELETE请求
        assert_context("DELETE", "/api/test/1", None, None).await;
    }
}
