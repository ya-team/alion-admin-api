/// 数据库配置模块
/// 
/// 定义了数据库连接的基本配置参数，包括MySQL、Redis和MongoDB的配置

use serde::Deserialize;

/// 数据库配置结构体
/// 
/// 包含所有数据库相关的配置参数，包括：
/// - MySQL数据库配置
/// - Redis缓存配置
/// - MongoDB文档数据库配置
#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    /// MySQL数据库配置
    /// 
    /// 包含数据库连接所需的所有参数，如主机地址、端口、用户名等
    pub mysql: MysqlConfig,

    /// Redis缓存配置
    /// 
    /// 包含Redis服务器连接参数，支持单机和集群模式
    pub redis: RedisConfig,

    /// MongoDB文档数据库配置
    /// 
    /// 包含MongoDB连接参数，用于存储非结构化数据
    pub mongodb: MongoConfig,
}

/// MySQL数据库配置结构体
/// 
/// 定义了MySQL数据库连接的详细参数
#[derive(Deserialize, Debug, Clone)]
pub struct MysqlConfig {
    /// 数据库主机地址
    pub host: String,

    /// 数据库端口号
    /// 
    /// 默认MySQL端口为3306
    pub port: u32,

    /// 数据库用户名
    pub username: String,

    /// 数据库密码
    pub password: String,

    /// 数据库名称
    pub database: String,

    /// 连接池最大连接数
    /// 
    /// 控制同时可以打开的数据库连接数量
    pub max_connections: u32,

    /// 连接池最小空闲连接数
    /// 
    /// 保持的最小空闲连接数，用于提高性能
    pub min_idle: Option<u32>,

    /// 连接超时时间（秒）
    /// 
    /// 建立连接时的最大等待时间
    pub connect_timeout: Option<u64>,

    /// 空闲连接超时时间（秒）
    /// 
    /// 连接在连接池中保持空闲的最大时间
    pub idle_timeout: Option<u64>,

    /// 连接最大生命周期（秒）
    /// 
    /// 连接在连接池中的最大存活时间
    pub max_lifetime: Option<u64>,
}

/// Redis配置结构体
/// 
/// 定义了Redis缓存的连接参数，支持单机和集群模式
#[derive(Deserialize, Debug, Clone)]
pub struct RedisConfig {
    /// Redis服务器模式
    /// 
    /// 支持单机模式和集群模式
    pub mode: RedisMode,

    /// 数据库索引
    /// 
    /// Redis数据库编号，默认为0
    pub database: i32,

    /// 密码
    /// 
    /// Redis服务器密码，如果不需要认证则为空
    pub password: Option<String>,

    /// 连接池大小
    /// 
    /// 控制Redis连接池的最大连接数
    pub pool_size: u32,

    /// 连接超时时间（秒）
    /// 
    /// 建立连接时的最大等待时间
    pub timeout: Option<u64>,

    /// 单机模式配置
    /// 
    /// 当mode为Single时使用
    pub single: Option<SingleRedisConfig>,

    /// 集群模式配置
    /// 
    /// 当mode为Cluster时使用
    pub cluster: Option<ClusterRedisConfig>,
}

/// Redis服务器模式枚举
/// 
/// 定义了Redis服务器的部署模式
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum RedisMode {
    /// 单机模式
    /// 
    /// 连接到单个Redis服务器
    Single,

    /// 集群模式
    /// 
    /// 连接到Redis集群
    Cluster,
}

/// 单机Redis配置结构体
/// 
/// 定义了单机模式下的Redis连接参数
#[derive(Deserialize, Debug, Clone)]
pub struct SingleRedisConfig {
    /// Redis服务器地址
    /// 
    /// 格式：host:port
    pub addr: String,
}

/// 集群Redis配置结构体
/// 
/// 定义了集群模式下的Redis连接参数
#[derive(Deserialize, Debug, Clone)]
pub struct ClusterRedisConfig {
    /// Redis集群节点地址列表
    /// 
    /// 格式：["host1:port1", "host2:port2", ...]
    pub nodes: Vec<String>,
}

/// MongoDB配置结构体
/// 
/// 定义了MongoDB文档数据库的连接参数
#[derive(Deserialize, Debug, Clone)]
pub struct MongoConfig {
    /// MongoDB连接URI
    /// 
    /// 格式：mongodb://username:password@host:port/database
    pub uri: String,

    /// 数据库名称
    pub database: String,

    /// 连接池最大连接数
    /// 
    /// 控制同时可以打开的MongoDB连接数量
    pub max_pool_size: Option<u32>,

    /// 连接超时时间（毫秒）
    /// 
    /// 建立连接时的最大等待时间
    pub connect_timeout_ms: Option<u64>,

    /// 服务器选择超时时间（毫秒）
    /// 
    /// 选择服务器时的最大等待时间
    pub server_selection_timeout_ms: Option<u64>,

    /// 心跳频率（毫秒）
    /// 
    /// 服务器心跳检测的时间间隔
    pub heartbeat_frequency_ms: Option<u64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DatabasesInstancesConfig {
    pub name: String,
    pub database: DatabaseConfig,
}
