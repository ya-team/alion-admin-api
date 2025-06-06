/// 配置模型模块
/// 
/// 该模块定义了应用程序的所有配置结构体
/// 包括数据库、服务器、认证等各个子系统的配置

/// 重新导出主配置结构体
pub use config::Config;

/// 重新导出数据库相关配置
pub use database_config::{DatabaseConfig, DatabasesInstancesConfig};

/// 重新导出JWT认证配置
pub use jwt_config::JwtConfig;

/// 重新导出Redis相关配置
pub use redis_config::{RedisConfig, RedisInstancesConfig, RedisMode};

/// 重新导出S3存储相关配置
pub use s3_config::{S3Config, S3InstancesConfig};

/// 重新导出服务器配置
pub use server_config::ServerConfig;

/// 可选配置集合的包装类
/// 
/// 用于包装可选的多实例配置列表
/// 例如：多个数据库实例、多个Redis实例等
/// 
/// 示例用法：
/// ```rust
/// let configs = OptionalConfigs {
///     configs: Some(vec![
///         DatabasesInstancesConfig {
///             name: "other_db".to_string(),
///             database: DatabaseConfig::default(),
///         }
///     ])
/// };
/// ```
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OptionalConfigs<T> {
    /// 配置实例列表
    /// 
    /// 包含多个配置实例的列表
    /// 如果为None，表示没有额外的配置实例
    pub configs: Option<Vec<T>>,
}

impl<T> From<Option<Vec<T>>> for OptionalConfigs<T> {
    /// 从Option<Vec<T>>创建OptionalConfigs
    /// 
    /// 允许从Option<Vec<T>>直接转换为OptionalConfigs
    /// 简化配置的创建过程
    fn from(configs: Option<Vec<T>>) -> Self {
        Self { configs }
    }
}

/// 配置模块
mod config;

/// 数据库配置模块
mod database_config;

/// JWT认证配置模块
mod jwt_config;

/// Redis配置模块
mod redis_config;

/// S3存储配置模块
mod s3_config;

/// 服务器配置模块
mod server_config;
