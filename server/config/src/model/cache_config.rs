/// 缓存配置模块
/// 
/// 定义了应用程序缓存的相关参数
/// 用于配置不同类型缓存的存储和过期策略

use serde::Deserialize;

/// 缓存配置结构体
/// 
/// 包含应用程序缓存所需的所有参数，包括：
/// - 内存缓存配置
/// - Redis缓存配置
/// - 缓存策略设置
#[derive(Deserialize, Debug, Clone)]
pub struct CacheConfig {
    /// 是否启用缓存
    /// 
    /// 控制是否启用应用程序缓存功能
    /// 建议在生产环境中启用
    pub enabled: bool,

    /// 默认缓存类型
    /// 
    /// 指定默认使用的缓存存储类型
    /// 可以是内存缓存或Redis缓存
    pub default_type: CacheType,

    /// 内存缓存配置
    /// 
    /// 本地内存缓存的配置参数
    /// 适用于单实例部署
    pub memory: Option<MemoryCacheConfig>,

    /// Redis缓存配置
    /// 
    /// Redis缓存的配置参数
    /// 适用于分布式部署
    pub redis: Option<RedisCacheConfig>,

    /// 默认过期时间（秒）
    /// 
    /// 缓存项的默认过期时间
    /// 如果未指定具体过期时间，则使用此值
    pub default_ttl: u64,

    /// 是否启用缓存预热
    /// 
    /// 控制是否在应用启动时预热缓存
    /// 可以减少首次访问的延迟
    pub enable_warmup: bool,

    /// 缓存预热配置
    /// 
    /// 定义需要预热的缓存项
    /// 仅在启用缓存预热时有效
    pub warmup: Option<CacheWarmupConfig>,
}

/// 缓存类型枚举
/// 
/// 定义了可用的缓存存储类型
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum CacheType {
    /// 内存缓存
    /// 
    /// 使用本地内存存储缓存数据
    /// 优点：快速、简单
    /// 缺点：不适用于多实例部署
    Memory,

    /// Redis缓存
    /// 
    /// 使用Redis存储缓存数据
    /// 优点：支持分布式部署
    /// 缺点：需要额外的Redis服务
    Redis,
}

/// 内存缓存配置结构体
/// 
/// 定义了本地内存缓存的详细参数
#[derive(Deserialize, Debug, Clone)]
pub struct MemoryCacheConfig {
    /// 最大缓存项数量
    /// 
    /// 内存缓存可以存储的最大项数
    /// 超过此限制时，将使用LRU策略移除旧项
    pub max_items: usize,

    /// 初始容量
    /// 
    /// 内存缓存的初始容量
    /// 用于优化内存分配
    pub initial_capacity: usize,

    /// 是否启用统计
    /// 
    /// 控制是否收集缓存使用统计信息
    /// 可以用于监控和调优
    pub enable_stats: bool,
}

/// Redis缓存配置结构体
/// 
/// 定义了Redis缓存的详细参数
#[derive(Deserialize, Debug, Clone)]
pub struct RedisCacheConfig {
    /// 键前缀
    /// 
    /// 用于区分不同应用的缓存键
    /// 例如："myapp:cache:"
    pub key_prefix: String,

    /// 是否启用压缩
    /// 
    /// 控制是否压缩存储在Redis中的值
    /// 可以节省内存使用
    pub enable_compression: bool,

    /// 压缩阈值（字节）
    /// 
    /// 触发压缩的最小数据大小
    /// 仅在启用压缩时有效
    pub compression_threshold: Option<usize>,

    /// 连接池大小
    /// 
    /// Redis连接池的最大连接数
    pub pool_size: u32,

    /// 连接超时（秒）
    /// 
    /// 建立Redis连接的超时时间
    pub connect_timeout: u64,

    /// 操作超时（秒）
    /// 
    /// Redis操作的最大等待时间
    pub operation_timeout: u64,
}

/// 缓存预热配置结构体
/// 
/// 定义了缓存预热的具体参数
#[derive(Deserialize, Debug, Clone)]
pub struct CacheWarmupConfig {
    /// 预热项列表
    /// 
    /// 需要预热的缓存项配置
    pub items: Vec<WarmupItem>,

    /// 预热超时（秒）
    /// 
    /// 整个预热过程的最大等待时间
    pub timeout: u64,

    /// 预热并发数
    /// 
    /// 同时预热的缓存项数量
    pub concurrency: usize,

    /// 预热重试次数
    /// 
    /// 预热失败时的重试次数
    pub retry_count: u32,

    /// 重试间隔（秒）
    /// 
    /// 预热重试之间的等待时间
    pub retry_interval: u64,
}

/// 预热项配置结构体
/// 
/// 定义了单个缓存预热项的详细参数
#[derive(Deserialize, Debug, Clone)]
pub struct WarmupItem {
    /// 缓存键
    /// 
    /// 要预热的缓存项的键
    pub key: String,

    /// 数据源类型
    /// 
    /// 预热数据的来源类型
    /// 例如：数据库、API等
    pub source_type: String,

    /// 数据源配置
    /// 
    /// 数据源的具体配置参数
    /// 格式取决于source_type
    pub source_config: serde_json::Value,

    /// 过期时间（秒）
    /// 
    /// 此缓存项的特定过期时间
    /// 如果未指定，则使用默认过期时间
    pub ttl: Option<u64>,
} 