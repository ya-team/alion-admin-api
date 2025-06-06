/**
 * 配置模型模块
 * 
 * 该模块定义了应用程序的所有配置结构体
 * 包括数据库、服务器、认证等各个子系统的配置
 * 
 * 主要功能：
 * - 提供统一的配置管理接口
 * - 支持多实例配置
 * - 支持配置的序列化和反序列化
 * - 提供类型安全的配置访问
 */

/**
 * 重新导出主配置结构体
 * 
 * 用于统一管理所有子系统的配置
 */
pub use config::Config;

/**
 * 重新导出数据库相关配置
 * 
 * 包含数据库连接和实例配置
 */
pub use database_config::{DatabaseConfig, DatabasesInstancesConfig};

/**
 * 重新导出JWT认证配置
 * 
 * 用于配置JWT令牌的生成和验证
 */
pub use jwt_config::JwtConfig;

/**
 * 重新导出Redis相关配置
 * 
 * 包含Redis连接和实例配置
 * 支持单机和集群模式
 */
pub use redis_config::{RedisConfig, RedisInstancesConfig, RedisMode};

/**
 * 重新导出S3存储相关配置
 * 
 * 用于配置对象存储服务
 * 支持多个存储实例
 */
pub use s3_config::{S3Config, S3InstancesConfig};

/**
 * 重新导出服务器配置
 * 
 * 包含HTTP服务器的基本配置
 * 如主机地址和端口号
 */
pub use server_config::ServerConfig;

/**
 * 可选配置集合的包装类
 * 
 * 用于包装可选的多实例配置列表
 * 例如：多个数据库实例、多个Redis实例等
 * 
 * 示例用法：
 * ```rust
 * let configs = OptionalConfigs {
 *     configs: Some(vec![
 *         DatabasesInstancesConfig {
 *             name: "other_db".to_string(),
 *             database: DatabaseConfig::default(),
 *         }
 *     ])
 * };
 * ```
 */
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OptionalConfigs<T> {
    /**
     * 配置实例列表
     * 
     * 包含多个配置实例的列表
     * 如果为None，表示没有额外的配置实例
     * 
     * 类型参数T必须实现Clone和Debug trait
     */
    pub configs: Option<Vec<T>>,
}

impl<T> From<Option<Vec<T>>> for OptionalConfigs<T> {
    /**
     * 从Option<Vec<T>>创建OptionalConfigs
     * 
     * 允许从Option<Vec<T>>直接转换为OptionalConfigs
     * 简化配置的创建过程
     * 
     * # Arguments
     * 
     * * `configs` - 配置实例列表的Option包装
     * 
     * # Returns
     * 
     * 返回一个新的OptionalConfigs实例
     */
    fn from(configs: Option<Vec<T>>) -> Self {
        Self { configs }
    }
}

/**
 * 配置模块
 * 
 * 包含主配置结构体的定义
 * 用于管理所有子系统的配置
 */
pub mod config;

/**
 * 数据库配置模块
 * 
 * 定义数据库连接和实例的配置
 * 支持多个数据库实例
 */
pub mod database_config;

/**
 * JWT认证配置模块
 * 
 * 定义JWT令牌的配置参数
 * 包括密钥、过期时间等
 */
pub mod jwt_config;

/**
 * Redis配置模块
 * 
 * 定义Redis缓存的配置参数
 * 支持单机和集群模式
 */
pub mod redis_config;

/**
 * S3存储配置模块
 * 
 * 定义对象存储服务的配置参数
 * 支持多个存储实例
 */
pub mod s3_config;

/**
 * 服务器配置模块
 * 
 * 定义HTTP服务器的基本配置
 * 包括主机地址和端口号
 */
pub mod server_config;
