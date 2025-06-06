/**
 * 配置模块
 * 
 * 该模块负责管理应用程序的配置，包括：
 * - 配置初始化：从文件加载和解析配置
 * - 配置模型：定义各种配置结构体
 * 
 * 主要导出的配置类型：
 * - Config: 应用程序主配置
 * - DatabaseConfig: 数据库配置
 * - JwtConfig: JWT配置
 * - RedisConfig: Redis配置
 * - S3Config: AWS S3配置
 * - ServerConfig: 服务器配置
 * 
 * 其他导出的类型：
 * - OptionalConfigs: 可选配置包装器
 * - RedisMode: Redis运行模式
 * - DatabasesInstancesConfig: 数据库实例配置
 * - RedisInstancesConfig: Redis实例配置
 * - S3InstancesConfig: S3实例配置
 * 
 * 使用示例：
 * ```rust
 * use server_config::init_from_file;
 * 
 * #[tokio::main]
 * async fn main() {
 *     // 从文件初始化配置
 *     init_from_file("config.yaml").await.unwrap();
 * }
 * ```
 */

/**
 * 重新导出配置初始化函数
 * 
 * 提供从文件加载和解析配置的功能
 * 支持YAML、TOML和JSON格式的配置文件
 */
pub use config_init::init_from_file;

/**
 * 重新导出配置模型
 * 
 * 包含所有配置相关的结构体和枚举
 * 用于定义和访问应用程序的配置
 */
pub use model::{
    Config, DatabaseConfig, DatabasesInstancesConfig, JwtConfig, OptionalConfigs,
    RedisConfig, RedisInstancesConfig, RedisMode, S3Config, S3InstancesConfig,
    ServerConfig,
};

/**
 * 重新导出日志宏
 * 
 * 用于记录配置相关的日志信息
 * 包括错误和普通信息日志
 */
pub use server_global::{project_error, project_info};

/**
 * 配置初始化模块
 * 
 * 负责从文件加载和解析配置
 * 提供配置初始化的核心功能
 */
mod config_init;

/**
 * 配置模型模块
 * 
 * 定义所有配置相关的结构体和枚举
 * 包含配置的序列化和反序列化实现
 */
mod model;
