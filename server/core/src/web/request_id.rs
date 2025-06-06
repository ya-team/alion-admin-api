/// 请求ID处理模块
/// 
/// 该模块提供了请求ID的生成和管理功能，包括：
/// - 为每个请求生成唯一的ID：使用ULID算法生成全局唯一的标识符
/// - 请求ID的中间件服务：为每个请求添加唯一ID
/// - 请求ID的层处理：提供请求ID服务的创建和管理
/// 
/// # 主要组件
/// 
/// ## RequestId
/// 请求ID结构，使用ULID作为唯一标识符：
/// - 全局唯一性
/// - 时间排序
/// - 可读性
/// 
/// ## RequestIdService
/// 请求ID服务，实现tower::Service trait：
/// - 为请求添加唯一ID
/// - 管理请求生命周期
/// 
/// ## RequestIdLayer
/// 请求ID层，实现tower::Layer trait：
/// - 创建请求ID服务
/// - 管理服务配置
/// 
/// # 使用示例
/// 
/// 
/// // 创建请求ID层
/// let layer = RequestIdLayer;
/// 
/// // 创建服务
/// let service = layer.layer(inner_service);
/// 
/// // 处理请求
/// let response = service.call(request).await?;
/// 

use std::{
    fmt,
    task::{Context, Poll},
};

use http::Request;
use tower_layer::Layer;
use tower_service::Service;
use ulid::Ulid;

/// 请求ID结构体，使用ULID作为唯一标识符
/// 
/// 使用ULID（Universally Unique Lexicographically Sortable Identifier）算法
/// 生成全局唯一的标识符。ULID具有以下特性：
/// - 全局唯一性：碰撞概率极低
/// - 时间排序：按时间顺序可排序
/// - 可读性：使用Base32编码，便于阅读和传输
/// 
/// # 字段
/// 
/// * `0`: ULID实例
#[derive(Debug, Clone)]
pub struct RequestId(pub Ulid);

impl RequestId {
    /// 创建新的请求ID
    /// 
    /// 生成一个新的ULID作为请求ID。
    /// ULID包含时间戳和随机数，确保全局唯一性。
    /// 
    /// # 返回
    /// * `Self` - 包含新生成的ULID的请求ID
    fn new() -> Self {
        Self(Ulid::new())
    }
}

impl fmt::Display for RequestId {
    /// 将请求ID格式化为字符串
    /// 
    /// 将ULID转换为Base32编码的字符串表示。
    /// 
    /// # 参数
    /// * `f` - 格式化器
    /// 
    /// # 返回
    /// * `fmt::Result` - 格式化结果
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0.to_string())
    }
}

/// 请求ID服务，用于为请求添加唯一ID
/// 
/// 实现tower::Service trait，为每个请求添加唯一的请求ID。
/// 请求ID被添加到请求的扩展中，可以在后续处理中使用。
/// 
/// # 类型参数
/// 
/// * `S`: 内部服务的类型，必须实现Service trait
/// 
/// # 字段
/// 
/// * `inner`: 内部服务实例
#[derive(Clone, Debug)]
pub struct RequestIdService<S> {
    /// 内部服务
    inner: S,
}

impl<S> RequestIdService<S> {
    /// 创建新的请求ID服务
    /// 
    /// 包装内部服务，提供请求ID功能。
    /// 
    /// # 参数
    /// * `inner` - 内部服务
    /// 
    /// # 返回
    /// * `Self` - 新的请求ID服务实例
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<B, S> Service<Request<B>> for RequestIdService<S>
where
    S: Service<Request<B>>,
{
    type Error = S::Error;
    type Future = S::Future;
    type Response = S::Response;

    /// 检查服务是否准备好处理请求
    /// 
    /// 检查内部服务是否准备好处理请求。
    /// 
    /// # 参数
    /// * `cx` - 任务上下文
    /// 
    /// # 返回
    /// * `Poll<Result<(), Self::Error>>` - 服务就绪状态
    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    /// 处理请求，为请求添加唯一ID
    /// 
    /// 为请求生成唯一的请求ID，并将其添加到请求的扩展中。
    /// 然后调用内部服务处理请求。
    /// 
    /// # 参数
    /// * `req` - HTTP请求
    /// 
    /// # 返回
    /// * `Self::Future` - 异步处理结果
    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        let id = RequestId::new();
        req.extensions_mut().insert(id);
        self.inner.call(req)
    }
}

/// 请求ID层，用于创建请求ID服务
/// 
/// 实现tower::Layer trait，用于创建RequestIdService实例。
/// 提供了一种方便的方式来为服务添加请求ID功能。
/// 
/// # 类型参数
/// 
/// * `S`: 内部服务的类型，必须实现Service trait
#[derive(Clone, Debug)]
pub struct RequestIdLayer;

impl<S> Layer<S> for RequestIdLayer {
    type Service = RequestIdService<S>;

    /// 创建请求ID服务层
    /// 
    /// 创建RequestIdService实例，包装内部服务。
    /// 
    /// # 参数
    /// * `inner` - 内部服务
    /// 
    /// # 返回
    /// * `Self::Service` - 新的请求ID服务
    fn layer(&self, inner: S) -> Self::Service {
        RequestIdService { inner }
    }
}
