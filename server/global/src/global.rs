/// 全局状态管理模块
/// 
/// 该模块实现了应用程序的全局状态管理，包括配置、连接池、事件通道等。
/// 所有全局状态都使用线程安全的容器进行管理，支持异步操作。

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
pub static GLOBAL_CONFIG: Lazy<RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// 初始化全局配置
/// 
/// # 参数
/// * `config` - 要存储的配置对象，必须实现Send + Sync trait
pub async fn init_config<T: 'static + Any + Send + Sync>(config: T) {
    let mut context = GLOBAL_CONFIG.write().await;
    context.insert(TypeId::of::<T>(), Arc::new(config));
}

/// 获取全局配置
/// 
/// # 参数
/// * `T` - 配置类型
/// 
/// # 返回
/// * `Option<Arc<T>>` - 如果存在则返回配置对象的Arc包装，否则返回None
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
pub static GLOBAL_PRIMARY_DB: Lazy<RwLock<Option<Arc<DatabaseConnection>>>> =
    Lazy::new(|| RwLock::new(None));

/// 数据库连接池
/// 
/// 使用字符串作为键，存储多个数据库连接实例
pub static GLOBAL_DB_POOL: Lazy<RwLock<HashMap<String, Arc<DatabaseConnection>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Redis连接类型
/// 
/// 支持单实例和集群两种模式
#[derive(Clone)]
pub enum RedisConnection {
    /// 单实例Redis连接
    Single(Arc<Client>),
    /// Redis集群连接
    Cluster(Arc<ClusterClient>),
}

/// 主Redis连接
pub static GLOBAL_PRIMARY_REDIS: Lazy<RwLock<Option<RedisConnection>>> =
    Lazy::new(|| RwLock::new(None));

/// Redis连接池
/// 
/// 使用字符串作为键，存储多个Redis连接实例
pub static GLOBAL_REDIS_POOL: Lazy<RwLock<HashMap<String, RedisConnection>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// 主MongoDB连接
pub static GLOBAL_PRIMARY_MONGO: Lazy<RwLock<Option<Arc<MongoClient>>>> =
    Lazy::new(|| RwLock::new(None));

/// MongoDB连接池
/// 
/// 使用字符串作为键，存储多个MongoDB连接实例
pub static GLOBAL_MONGO_POOL: Lazy<RwLock<HashMap<String, Arc<MongoClient>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

//*****************************************************************************
// AWS S3客户端管理
//*****************************************************************************

/// 主S3客户端
pub static GLOBAL_PRIMARY_S3: Lazy<RwLock<Option<Arc<S3Client>>>> = Lazy::new(|| RwLock::new(None));

/// S3客户端池
/// 
/// 使用字符串作为键，存储多个S3客户端实例
pub static GLOBAL_S3_POOL: Lazy<RwLock<HashMap<String, Arc<S3Client>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

//*****************************************************************************
// JWT密钥和验证管理
//*****************************************************************************

/// JWT密钥对
/// 
/// 包含用于签名和验证的密钥
pub struct Keys {
    /// 用于签名的密钥
    pub encoding: EncodingKey,
    /// 用于验证的密钥
    pub decoding: DecodingKey,
}

impl Keys {
    /// 创建新的密钥对
    /// 
    /// # 参数
    /// * `secret` - 密钥字节数组
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

/// 全局JWT密钥对
pub static KEYS: OnceCell<Arc<Mutex<Keys>>> = OnceCell::const_new();

/// 全局JWT验证器
pub static VALIDATION: OnceCell<Arc<Mutex<Validation>>> = OnceCell::const_new();

//*****************************************************************************
// 事件通道管理
//*****************************************************************************

/// 动态类型事件通道条目
struct DynChannelEntry {
    /// 通道名称
    name: String,
    /// 事件发送器
    tx: mpsc::UnboundedSender<Box<dyn Any + Send>>,
}

/// 事件通道管理器
/// 
/// 管理所有类型的事件通道
struct EventChannels {
    /// 字符串事件发送器
    string_tx: mpsc::UnboundedSender<String>,
    /// 动态类型事件通道列表
    dyn_channels: Vec<DynChannelEntry>,
}

/// 全局事件通道管理器
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
#[inline]
pub async fn get_string_sender() -> mpsc::UnboundedSender<String> {
    EVENT_CHANNELS.lock().await.string_tx.clone()
}

/// 获取动态类型事件发送器
/// 
/// # 参数
/// * `name` - 通道名称
/// 
/// # 返回
/// * `Option<mpsc::UnboundedSender<Box<dyn Any + Send>>>` - 如果存在则返回发送器，否则返回None
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
/// # 参数
/// * `string_listener` - 字符串事件监听器
/// * `dyn_listeners` - 动态类型事件监听器列表
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

//*****************************************************************************
// 路由信息收集
//*****************************************************************************

/// 路由信息结构体
#[derive(Clone, Debug)]
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
    pub fn new(path: &str, method: Method, service_name: &str, summary: &str) -> Self {
        RouteInfo {
            path: path.to_string(),
            method,
            service_name: service_name.to_string(),
            summary: summary.to_string(),
        }
    }
}

/// 路由信息收集器
pub static ROUTE_COLLECTOR: Lazy<Mutex<Vec<RouteInfo>>> = Lazy::new(|| Mutex::new(Vec::new()));

/// 添加路由信息
pub async fn add_route(route: RouteInfo) {
    ROUTE_COLLECTOR.lock().await.push(route);
}

/// 获取所有收集的路由信息
pub async fn get_collected_routes() -> Vec<RouteInfo> {
    ROUTE_COLLECTOR.lock().await.clone()
}

/// 清空路由信息
pub async fn clear_routes() {
    ROUTE_COLLECTOR.lock().await.clear();
}

//*****************************************************************************
// 操作日志管理
//*****************************************************************************

/// 操作日志上下文
#[derive(Debug, Clone)]
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

/// 操作日志上下文存储
static OPERATION_LOG_CONTEXT: Lazy<Arc<RwLock<Option<OperationLogContext>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl OperationLogContext {
    /// 设置操作日志上下文
    pub async fn set(context: OperationLogContext) {
        let mut writer = OPERATION_LOG_CONTEXT.write().await;
        *writer = Some(context);
    }

    /// 获取操作日志上下文
    pub async fn get() -> Option<OperationLogContext> {
        OPERATION_LOG_CONTEXT.read().await.clone()
    }

    /// 清空操作日志上下文
    pub async fn clear() {
        let mut writer = OPERATION_LOG_CONTEXT.write().await;
        *writer = None;
    }
}

/// 异步发送字符串事件
#[inline]
pub fn send_string_event(msg: String) {
    tokio::spawn(async move {
        let sender = get_string_sender().await;
        let _ = sender.send(msg);
    });
}

/// 异步发送动态类型事件
#[inline]
pub fn send_dyn_event(event_name: &'static str, event: Box<dyn Any + Send>) {
    tokio::spawn(async move {
        if let Some(sender) = get_dyn_sender(event_name).await {
            let _ = sender.send(event);
        }
    });
}
