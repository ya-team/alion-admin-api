/// 配置模块
/// 
/// 该模块负责管理应用程序的配置，包括：
/// - 配置初始化：从文件加载和解析配置
/// - 配置模型：定义各种配置结构体
/// 
/// 主要导出的配置类型：
/// - Config: 应用程序主配置
/// - DatabaseConfig: 数据库配置
/// - JwtConfig: JWT配置
/// - MongoConfig: MongoDB配置
/// - RedisConfig: Redis配置
/// - S3Config: AWS S3配置
/// - ServerConfig: 服务器配置
/// 
/// 其他导出的类型：
/// - OptionalConfigs: 可选配置包装器
/// - RedisMode: Redis运行模式
/// - DatabasesInstancesConfig: 数据库实例配置
/// - MongoInstancesConfig: MongoDB实例配置
/// - RedisInstancesConfig: Redis实例配置
/// - S3InstancesConfig: S3实例配置

/// 重新导出配置初始化函数
pub use config_init::init_from_file;

/// 重新导出配置模型
pub use model::{
    Config, DatabaseConfig, DatabasesInstancesConfig, JwtConfig, MongoConfig, MongoInstancesConfig,
    OptionalConfigs, RedisConfig, RedisInstancesConfig, RedisMode, S3Config, S3InstancesConfig,
    ServerConfig,
};

/// 重新导出日志宏
pub use server_global::{project_error, project_info};

/// 配置初始化模块
mod config_init;

/// 配置模型模块
mod model;
