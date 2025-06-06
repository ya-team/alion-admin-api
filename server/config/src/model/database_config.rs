/**
 * 数据库配置模块
 * 
 * 定义了数据库连接的基本配置参数
 */

use serde::Deserialize;

/**
 * 数据库配置结构体
 * 
 * 包含所有数据库相关的配置参数，包括：
 * - 数据库连接URL
 * - 连接池配置
 */
#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    /**
     * 数据库连接URL
     * 
     * 格式：postgres://username:password@host:port/database
     */
    pub url: String,

    /**
     * 连接池最大连接数
     * 
     * 控制同时可以打开的数据库连接数量
     */
    pub max_connections: u32,

    /**
     * 连接池最小空闲连接数
     * 
     * 保持的最小空闲连接数，用于提高性能
     */
    pub min_idle: Option<u32>,

    /**
     * 连接超时时间（秒）
     * 
     * 建立连接时的最大等待时间
     */
    pub connect_timeout: Option<u64>,

    /**
     * 空闲连接超时时间（秒）
     * 
     * 连接在连接池中保持空闲的最大时间
     */
    pub idle_timeout: Option<u64>,

    /**
     * 连接最大生命周期（秒）
     * 
     * 连接在连接池中的最大存活时间
     */
    pub max_lifetime: Option<u64>,
}

/**
 * 数据库实例配置结构体
 * 
 * 用于配置多个命名的数据库连接
 */
#[derive(Deserialize, Debug, Clone)]
pub struct DatabasesInstancesConfig {
    /**
     * 实例名称
     */
    pub name: String,
    /**
     * 数据库配置
     */
    pub database: DatabaseConfig,
}

/**
 * PostgreSQL数据库配置结构体
 * 
 * 定义了PostgreSQL数据库连接的详细参数
 */
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct PostgresConfig {
    /**
     * 数据库主机地址
     */
    pub host: String,

    /**
     * 数据库端口号
     * 
     * 默认PostgreSQL端口为5432
     */
    pub port: u32,

    /**
     * 数据库用户名
     */
    pub username: String,

    /**
     * 数据库密码
     */
    pub password: String,

    /**
     * 数据库名称
     */
    pub database: String,

    /**
     * 连接池最大连接数
     * 
     * 控制同时可以打开的数据库连接数量
     */
    pub max_connections: u32,

    /**
     * 连接池最小空闲连接数
     * 
     * 保持的最小空闲连接数，用于提高性能
     */
    pub min_idle: Option<u32>,

    /**
     * 连接超时时间（秒）
     * 
     * 建立连接时的最大等待时间
     */
    pub connect_timeout: Option<u64>,

    /**
     * 空闲连接超时时间（秒）
     * 
     * 连接在连接池中保持空闲的最大时间
     */
    pub idle_timeout: Option<u64>,

    /**
     * 连接最大生命周期（秒）
     * 
     * 连接在连接池中的最大存活时间
     */
    pub max_lifetime: Option<u64>,
}

/**
 * Redis配置结构体
 * 
 * 定义了Redis缓存的连接参数，支持单机和集群模式
 */
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct RedisConfig {
    /**
     * Redis服务器模式
     * 
     * 支持单机模式和集群模式
     */
    pub mode: RedisMode,

    /**
     * 数据库索引
     * 
     * Redis数据库编号，默认为0
     */
    pub database: i32,

    /**
     * 密码
     * 
     * Redis服务器密码，如果不需要认证则为空
     */
    pub password: Option<String>,

    /**
     * 连接池大小
     * 
     * 控制Redis连接池的最大连接数
     */
    pub pool_size: u32,

    /**
     * 连接超时时间（秒）
     * 
     * 建立连接时的最大等待时间
     */
    pub timeout: Option<u64>,

    /**
     * 单机模式配置
     * 
     * 当mode为Single时使用
     */
    pub single: Option<SingleRedisConfig>,

    /**
     * 集群模式配置
     * 
     * 当mode为Cluster时使用
     */
    pub cluster: Option<ClusterRedisConfig>,
}

/**
 * Redis服务器模式枚举
 * 
 * 定义了Redis服务器的部署模式
 */
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum RedisMode {
    /**
     * 单机模式
     * 
     * 连接到单个Redis服务器
     */
    Single,

    /**
     * 集群模式
     * 
     * 连接到Redis集群
     */
    Cluster,
}

/**
 * 单机Redis配置结构体
 * 
 * 定义了单机模式下的Redis连接参数
 */
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct SingleRedisConfig {
    /**
     * Redis服务器地址
     * 
     * 格式：host:port
     */
    pub addr: String,
}

/**
 * 集群Redis配置结构体
 * 
 * 定义了集群模式下的Redis连接参数
 */
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct ClusterRedisConfig {
    /**
     * Redis集群节点地址列表
     * 
     * 格式：["host1:port1", "host2:port2", ...]
     */
    pub nodes: Vec<String>,
}
