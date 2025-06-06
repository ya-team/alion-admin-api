/// 全局状态管理模块
/// 
/// 该模块实现了应用程序的全局状态管理，包括配置、连接池、事件通道等。
/// 所有全局状态都使用线程安全的容器进行管理，支持异步操作。
/// 
/// # 主要功能
/// 
/// ## 配置管理
/// 使用TypeId作为键，支持存储和访问任意类型的配置对象。
/// 所有配置对象都被包装在Arc中，支持跨线程共享。
/// 
/// ## 连接池管理
/// 支持多种数据库和服务的连接池管理：
/// - 主数据库连接
/// - 多数据库连接池
/// - Redis连接（单实例和集群）
/// - MongoDB连接
/// - S3客户端
/// 
/// ## JWT管理
/// 提供JWT令牌的签名和验证功能：
/// - 密钥对管理
/// - 验证器配置
/// 
/// ## 事件系统
/// 提供基于通道的事件通信机制：
/// - 字符串事件通道
/// - 动态类型事件通道
/// 
/// ## 路由管理
/// 记录和管理API路由信息，包括：
/// - 路由路径
/// - HTTP方法
/// - 服务名称
/// - 路由描述
/// 
/// ## 操作日志
/// 记录和管理操作日志信息，包括：
/// - 用户信息
/// - 请求信息
/// - 响应信息
/// - 时间信息
/// 
/// # 使用示例
/// 
/// 
/// // 初始化配置
/// let config = AppConfig::new();
/// global::init_config(config).await;
/// 
/// // 获取配置
/// let config = global::get_config::<AppConfig>().await.unwrap();
/// 
/// // 注册事件监听器
/// global::register_event_listeners(
///     Box::new(|rx| Box::pin(async move {
///         while let Some(msg) = rx.recv().await {
///             println!("Received: {}", msg);
///         }
///     })),
///     &[],
/// ).await;
/// 
/// // 发送事件
/// global::send_string_event("Hello, World!".to_string());
/// 
/// // 记录路由信息
/// global::add_route(RouteInfo::new(
///     "/api/users",
///     Method::GET,
///     "user_service",
///     "Get user list",
/// )).await;
/// 
/// // 记录操作日志
/// let context = OperationLogContext {
///     user_id: Some("user123".to_string()),
///     username: Some("john_doe".to_string()),
///     // ... 其他字段
/// };
/// OperationLogContext::set(context).await;
/// 

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::Arc,
};

use aws_sdk_s3::Client as S3Client;
use chrono::NaiveDateTime;
use http::Method;
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use mongodb::Client as MongoClient;
use once_cell::sync::Lazy;
use redis::{cluster::ClusterClient, Client};
use sea_orm::DatabaseConnection;
use serde_json::Value;
use tokio::sync::{mpsc, Mutex, OnceCell, RwLock};

use crate::project_info;

//*****************************************************************************
// 全局配置管理
//*****************************************************************************

/// 全局配置存储
/// 
/// 使用TypeId作为键，存储不同类型的配置对象。
/// 所有配置对象都被包装在Arc中，支持跨线程共享。
/// 
/// # 类型参数
/// * `T`: 配置类型，必须实现Send + Sync trait
/// 
/// # 使用示例
/// 
/// // 存储配置
/// let config = AppConfig::new();
/// global::init_config(config).await;
/// 
/// // 获取配置
/// let config = global::get_config::<AppConfig>().await.unwrap();
/// 
pub static GLOBAL_CONFIG: Lazy<RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// 全局路由存储
/// 
/// 存储应用程序的所有路由信息。
/// 使用RwLock包装，支持并发访问。
pub static GLOBAL_ROUTES: Lazy<RwLock<Vec<RouteInfo>>> = Lazy::new(|| RwLock::new(Vec::new()));

/// 全局操作日志上下文
/// 
/// 存储当前请求的操作日志上下文。
/// 使用RwLock包装，支持并发访问。
pub static GLOBAL_OPERATION_LOG_CTX: Lazy<RwLock<Option<OperationLogContext>>> = Lazy::new(|| RwLock::new(None));

/// 初始化全局配置
/// 
/// 将配置对象存储到全局配置存储中。
/// 
/// # 参数
/// * `config` - 要存储的配置对象，必须实现Send + Sync trait
/// 
/// # 使用示例
/// 
/// let config = AppConfig::new();
/// global::init_config(config).await;
/// 
pub async fn init_config<T: 'static + Any + Send + Sync>(config: T) {
    let mut context = GLOBAL_CONFIG.write().await;
    context.insert(TypeId::of::<T>(), Arc::new(config));
}

/// 获取全局配置
/// 
/// 从全局配置存储中获取指定类型的配置对象。
/// 
/// # 类型参数
/// * `T` - 配置类型，必须实现Send + Sync trait
/// 
/// # 返回
/// * `Option<Arc<T>>` - 如果存在则返回配置对象的Arc包装，否则返回None
/// 
/// # 使用示例
/// 
/// let config = global::get_config::<AppConfig>().await.unwrap();
/// 
pub async fn get_config<T: 'static + Any + Send + Sync>() -> Option<Arc<T>> {
    let context = GLOBAL_CONFIG.read().await;
    context
        .get(&TypeId::of::<T>())
        .and_then(|config| config.clone().downcast::<T>().ok())
}

//*****************************************************************************
// 数据库连接管理
//*****************************************************************************

/// 主数据库连接
/// 
/// 存储应用程序的主数据库连接实例。
/// 使用RwLock包装，支持并发访问。
pub static GLOBAL_PRIMARY_DB: Lazy<RwLock<Option<Arc<DatabaseConnection>>>> =
    Lazy::new(|| RwLock::new(None));

/// 数据库连接池
/// 
/// 使用字符串作为键，存储多个数据库连接实例。
/// 支持按名称访问不同的数据库连接。
/// 
/// # 使用示例
/// 
/// // 添加数据库连接
/// let conn = DatabaseConnection::connect("postgres://...").await.unwrap();
/// let mut pool = GLOBAL_DB_POOL.write().await;
/// pool.insert("secondary".to_string(), Arc::new(conn));
/// 
/// // 获取数据库连接
/// let conn = GLOBAL_DB_POOL.read().await
///     .get("secondary")
///     .cloned()
///     .unwrap();
/// 
pub static GLOBAL_DB_POOL: Lazy<RwLock<HashMap<String, Arc<DatabaseConnection>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Redis连接类型
/// 
/// 支持单实例和集群两种模式的Redis连接。
/// 
/// # 变体
/// * `Single` - 单实例Redis连接
/// * `Cluster` - Redis集群连接
#[derive(Clone)]
pub enum RedisConnection {
    /// 单实例Redis连接
    Single(Arc<Client>),
    /// Redis集群连接
    Cluster(Arc<ClusterClient>),
}

/// 主Redis连接
/// 
/// 存储应用程序的主Redis连接实例。
/// 使用RwLock包装，支持并发访问。
pub static GLOBAL_PRIMARY_REDIS: Lazy<RwLock<Option<RedisConnection>>> =
    Lazy::new(|| RwLock::new(None));

/// Redis连接池
/// 
/// 使用字符串作为键，存储多个Redis连接实例。
/// 支持按名称访问不同的Redis连接。
/// 
/// # 使用示例
/// 
/// // 添加Redis连接
/// let client = Client::open("redis://...").unwrap();
/// let mut pool = GLOBAL_REDIS_POOL.write().await;
/// pool.insert("cache".to_string(), RedisConnection::Single(Arc::new(client)));
/// 
/// // 获取Redis连接
/// let conn = GLOBAL_REDIS_POOL.read().await
///     .get("cache")
///     .cloned()
///     .unwrap();
/// 
pub static GLOBAL_REDIS_POOL: Lazy<RwLock<HashMap<String, RedisConnection>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// 主MongoDB连接
/// 
/// 存储应用程序的主MongoDB连接实例。
/// 使用RwLock包装，支持并发访问。
pub static GLOBAL_PRIMARY_MONGO: Lazy<RwLock<Option<Arc<MongoClient>>>> =
    Lazy::new(|| RwLock::new(None));

/// MongoDB连接池
/// 
/// 使用字符串作为键，存储多个MongoDB连接实例。
/// 支持按名称访问不同的MongoDB连接。
/// 
/// # 使用示例
/// 
/// // 添加MongoDB连接
/// let client = MongoClient::with_uri_str("mongodb://...").await.unwrap();
/// let mut pool = GLOBAL_MONGO_POOL.write().await;
/// pool.insert("logs".to_string(), Arc::new(client));
/// 
/// // 获取MongoDB连接
/// let client = GLOBAL_MONGO_POOL.read().await
///     .get("logs")
///     .cloned()
///     .unwrap();
/// 
pub static GLOBAL_MONGO_POOL: Lazy<RwLock<HashMap<String, Arc<MongoClient>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

//*****************************************************************************
// AWS S3客户端管理
//*****************************************************************************

/// 主S3客户端
/// 
/// 存储应用程序的主S3客户端实例。
/// 使用RwLock包装，支持并发访问。
pub static GLOBAL_PRIMARY_S3: Lazy<RwLock<Option<Arc<S3Client>>>> = Lazy::new(|| RwLock::new(None));

/// S3客户端池
/// 
/// 使用字符串作为键，存储多个S3客户端实例。
/// 支持按名称访问不同的S3客户端。
/// 
/// # 使用示例
/// 
/// // 添加S3客户端
/// let config = aws_config::load_from_env().await;
/// let client = S3Client::new(&config);
/// let mut pool = GLOBAL_S3_POOL.write().await;
/// pool.insert("backup".to_string(), Arc::new(client));
/// 
/// // 获取S3客户端
/// let client = GLOBAL_S3_POOL.read().await
///     .get("backup")
///     .cloned()
///     .unwrap();
/// 
pub static GLOBAL_S3_POOL: Lazy<RwLock<HashMap<String, Arc<S3Client>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

//*****************************************************************************
// JWT密钥和验证管理
//*****************************************************************************

/// JWT密钥对
/// 
/// 包含用于签名和验证的密钥。
/// 
/// # 字段
/// * `encoding` - 用于签名的密钥
/// * `decoding` - 用于验证的密钥
pub struct Keys {
    /// 用于签名的密钥
    pub encoding: EncodingKey,
    /// 用于验证的密钥
    pub decoding: DecodingKey,
}

impl Keys {
    /// 创建新的密钥对
    /// 
    /// 从密钥字节数组创建JWT密钥对。
    /// 
    /// # 参数
    /// * `secret` - 密钥字节数组
    /// 
    /// # 使用示例
    /// 
    /// let keys = Keys::new(b"your-secret-key");
    /// 
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

/// 全局JWT密钥对
/// 
/// 存储应用程序的JWT密钥对。
/// 使用OnceCell和Mutex包装，支持并发访问。
pub static KEYS: OnceCell<Arc<Mutex<Keys>>> = OnceCell::const_new();

/// 全局JWT验证器
/// 
/// 存储应用程序的JWT验证器配置。
/// 使用OnceCell和Mutex包装，支持并发访问。
pub static VALIDATION: OnceCell<Arc<Mutex<Validation>>> = OnceCell::const_new();

//*****************************************************************************
// 事件通道管理
//*****************************************************************************

/// 动态类型事件通道条目
/// 
/// 存储动态类型事件通道的信息。
/// 
/// # 字段
/// * `name` - 通道名称
/// * `tx` - 事件发送器
struct DynChannelEntry {
    /// 通道名称
    name: String,
    /// 事件发送器
    tx: mpsc::UnboundedSender<Box<dyn Any + Send>>,
}

/// 事件通道管理器
/// 
/// 管理所有类型的事件通道。
/// 
/// # 字段
/// * `string_tx` - 字符串事件发送器
/// * `dyn_channels` - 动态类型事件通道列表
struct EventChannels {
    /// 字符串事件发送器
    string_tx: mpsc::UnboundedSender<String>,
    /// 动态类型事件通道列表
    dyn_channels: Vec<DynChannelEntry>,
}

/// 全局事件通道管理器
/// 
/// 存储应用程序的事件通道管理器实例。
/// 使用Lazy和Mutex包装，支持并发访问。
static EVENT_CHANNELS: Lazy<Arc<Mutex<EventChannels>>> = Lazy::new(|| {
    let (string_tx, _) = mpsc::unbounded_channel();
    Arc::new(Mutex::new(EventChannels {
        string_tx,
        dyn_channels: Vec::new(),
    }))
});

/// 动态Future类型别名
type DynFuture = dyn Future<Output = ()> + Send + 'static;

/// 字符串事件监听器类型别名
type StringListener = Box<dyn FnOnce(mpsc::UnboundedReceiver<String>) -> Pin<Box<DynFuture>>>;

/// 动态类型事件监听器类型别名
type DynListener = (
    String,
    Box<dyn Fn(mpsc::UnboundedReceiver<Box<dyn Any + Send>>) -> Pin<Box<DynFuture>>>,
);

/// 获取字符串事件发送器
/// 
/// 返回全局字符串事件通道的发送器。
/// 
/// # 返回
/// * `mpsc::UnboundedSender<String>` - 字符串事件发送器
/// 
/// # 使用示例
/// 
/// let tx = global::get_string_sender().await;
/// tx.send("Hello, World!".to_string()).unwrap();
/// 
#[inline]
pub async fn get_string_sender() -> mpsc::UnboundedSender<String> {
    EVENT_CHANNELS.lock().await.string_tx.clone()
}

/// 获取动态类型事件发送器
/// 
/// 根据通道名称获取动态类型事件通道的发送器。
/// 
/// # 参数
/// * `name` - 通道名称
/// 
/// # 返回
/// * `Option<mpsc::UnboundedSender<Box<dyn Any + Send>>>` - 如果存在则返回发送器，否则返回None
/// 
/// # 使用示例
/// 
/// let tx = global::get_dyn_sender("user_events").await.unwrap();
/// tx.send(Box::new(UserEvent::Login)).unwrap();
/// 
#[inline]
pub async fn get_dyn_sender(name: &str) -> Option<mpsc::UnboundedSender<Box<dyn Any + Send>>> {
    let channels = EVENT_CHANNELS.lock().await;
    channels
        .dyn_channels
        .iter()
        .find(|entry| entry.name == name)
        .map(|entry| entry.tx.clone())
}

/// 注册事件监听器
/// 
/// 注册字符串事件监听器和动态类型事件监听器。
/// 
/// # 参数
/// * `string_listener` - 字符串事件监听器
/// * `dyn_listeners` - 动态类型事件监听器列表
/// 
/// # 使用示例
/// 
/// global::register_event_listeners(
///     Box::new(|rx| Box::pin(async move {
///         while let Some(msg) = rx.recv().await {
///             println!("Received: {}", msg);
///         }
///     })),
///     &[
///         ("user_events".to_string(), Box::new(|rx| Box::pin(async move {
///             while let Some(event) = rx.recv().await {
///             if let Some(user_event) = event.downcast_ref::<UserEvent>() {
///                 println!("User event: {:?}", user_event);
///             }
///         }
///     }))),
///     ],
/// ).await;
/// 
pub async fn register_event_listeners(
    string_listener: StringListener,
    dyn_listeners: &[DynListener],
) {
    let mut channels = EVENT_CHANNELS.lock().await;

    // 设置字符串事件通道
    let (string_tx, string_rx) = mpsc::unbounded_channel();
    channels.string_tx = string_tx;

    // 启动字符串事件监听器
    tokio::spawn(string_listener(string_rx));
    project_info!("String event listener spawned");

    // 清空旧的发送器
    channels.dyn_channels.clear();

    // 为每个监听器创建独立通道
    for (name, listener) in dyn_listeners {
        let (tx, rx) = mpsc::unbounded_channel();
        channels.dyn_channels.push(DynChannelEntry {
            name: name.clone(),
            tx,
        });
        tokio::spawn(listener(rx));
        project_info!("Dynamic event listener '{}' spawned", name);
    }
}

/// 路由信息
/// 
/// 记录API路由的详细信息，包括路径、方法、服务名称和描述。
/// 
/// # 字段
/// * `path` - 路由路径
/// * `method` - HTTP方法
/// * `service_name` - 服务名称
/// * `summary` - 路由描述
#[derive(Clone)]
pub struct RouteInfo {
    /// 路由路径
    pub path: String,
    /// HTTP方法
    pub method: Method,
    /// 服务名称
    pub service_name: String,
    /// 路由描述
    pub summary: String,
}

impl RouteInfo {
    /// 创建新的路由信息
    /// 
    /// # 参数
    /// * `path` - 路由路径
    /// * `method` - HTTP方法
    /// * `service_name` - 服务名称
    /// * `summary` - 路由描述
    /// 
    /// # 返回
    /// * `Self` - 新的路由信息实例
    /// 
    /// # 使用示例
    /// 
    /// let route = RouteInfo::new(
    ///     "/api/users",
    ///     Method::GET,
    ///     "user_service",
    ///     "Get user list",
    /// );
    /// 
    pub fn new(path: &str, method: Method, service_name: &str, summary: &str) -> Self {
        Self {
            path: path.to_string(),
            method,
            service_name: service_name.to_string(),
            summary: summary.to_string(),
        }
    }
}

/// 添加路由信息
/// 
/// 将路由信息添加到全局路由集合中。
/// 
/// # 参数
/// * `route` - 路由信息
/// 
/// # 使用示例
/// 
/// global::add_route(RouteInfo::new(
///     "/api/users",
///     Method::GET,
///     "user_service",
///     "Get user list",
/// )).await;
/// 
pub async fn add_route(route: RouteInfo) {
    let mut routes = GLOBAL_ROUTES.write().await;
    routes.push(route);
}

/// 获取收集的路由信息
/// 
/// 返回所有已收集的路由信息。
/// 
/// # 返回
/// * `Vec<RouteInfo>` - 路由信息列表
/// 
/// # 使用示例
/// 
/// let routes = global::get_collected_routes().await;
/// for route in routes {
///     println!("{} {} - {}", route.method, route.path, route.summary);
/// }
/// 
pub async fn get_collected_routes() -> Vec<RouteInfo> {
    GLOBAL_ROUTES.read().await.clone()
}

/// 清空路由信息
/// 
/// 清空所有已收集的路由信息。
/// 
/// # 使用示例
/// 
/// global::clear_routes().await;
/// 
pub async fn clear_routes() {
    let mut routes = GLOBAL_ROUTES.write().await;
    routes.clear();
}

/// 操作日志上下文
/// 
/// 记录请求处理过程中的操作日志信息，包括用户信息、请求信息、响应信息等。
/// 
/// # 字段
/// * `user_id` - 用户ID
/// * `username` - 用户名
/// * `domain` - 域名
/// * `module_name` - 模块名称
/// * `description` - 操作描述
/// * `request_id` - 请求ID
/// * `method` - HTTP方法
/// * `url` - 请求URL
/// * `ip` - 客户端IP
/// * `user_agent` - 用户代理
/// * `params` - 请求参数
/// * `body` - 请求体
/// * `response` - 响应数据
/// * `start_time` - 开始时间
/// * `end_time` - 结束时间
/// * `duration` - 持续时间（毫秒）
/// * `created_at` - 创建时间
#[derive(Clone)]
pub struct OperationLogContext {
    /// 用户ID
    pub user_id: Option<String>,
    /// 用户名
    pub username: Option<String>,
    /// 域名
    pub domain: Option<String>,
    /// 模块名称
    pub module_name: String,
    /// 操作描述
    pub description: String,
    /// 请求ID
    pub request_id: String,
    /// HTTP方法
    pub method: String,
    /// 请求URL
    pub url: String,
    /// 客户端IP
    pub ip: String,
    /// 用户代理
    pub user_agent: Option<String>,
    /// 请求参数
    pub params: Option<Value>,
    /// 请求体
    pub body: Option<Value>,
    /// 响应数据
    pub response: Option<Value>,
    /// 开始时间
    pub start_time: NaiveDateTime,
    /// 结束时间
    pub end_time: NaiveDateTime,
    /// 持续时间（毫秒）
    pub duration: i32,
    /// 创建时间
    pub created_at: NaiveDateTime,
}

impl OperationLogContext {
    /// 设置操作日志上下文
    /// 
    /// 将操作日志上下文存储到全局上下文中。
    /// 
    /// # 参数
    /// * `context` - 操作日志上下文
    /// 
    /// # 使用示例
    /// 
    /// let context = OperationLogContext {
    ///     user_id: Some("user123".to_string()),
    ///     username: Some("john_doe".to_string()),
    ///     // ... 其他字段
    /// };
    /// OperationLogContext::set(context).await;
    /// 
    pub async fn set(context: OperationLogContext) {
        let mut ctx = GLOBAL_OPERATION_LOG_CTX.write().await;
        *ctx = Some(context);
    }

    /// 获取操作日志上下文
    /// 
    /// 从全局上下文中获取操作日志上下文。
    /// 
    /// # 返回
    /// * `Option<OperationLogContext>` - 如果存在则返回操作日志上下文，否则返回None
    /// 
    /// # 使用示例
    /// 
    /// let context = OperationLogContext::get().await;
    /// if let Some(ctx) = context {
    ///     println!("User: {}", ctx.username.unwrap_or_default());
    /// }
    /// 
    pub async fn get() -> Option<OperationLogContext> {
        GLOBAL_OPERATION_LOG_CTX.read().await.clone()
    }

    /// 清空操作日志上下文
    /// 
    /// 清空全局上下文中的操作日志上下文。
    /// 
    /// # 使用示例
    /// 
    /// OperationLogContext::clear().await;
    /// 
    pub async fn clear() {
        let mut ctx = GLOBAL_OPERATION_LOG_CTX.write().await;
        *ctx = None;
    }
}

/// 发送字符串事件
/// 
/// 向全局字符串事件通道发送事件。
/// 
/// # 参数
/// * `msg` - 事件消息
/// 
/// # 使用示例
/// 
/// global::send_string_event("Hello, World!".to_string());
/// 
pub fn send_string_event(msg: String) {
    let tx = EVENT_CHANNELS.blocking_lock().string_tx.clone();
    let _ = tx.send(msg);
}

/// 发送动态类型事件
/// 
/// 向指定的动态类型事件通道发送事件。
/// 
/// # 参数
/// * `event_name` - 事件通道名称
/// * `event` - 事件对象
/// 
/// # 使用示例
/// 
/// global::send_dyn_event("user_events", Box::new(UserEvent::Login));
/// 
pub fn send_dyn_event(event_name: &'static str, event: Box<dyn Any + Send>) {
    let channels = EVENT_CHANNELS.blocking_lock();
    if let Some(entry) = channels.dyn_channels.iter().find(|entry| entry.name == event_name) {
        let _ = entry.tx.send(event);
    }
}
