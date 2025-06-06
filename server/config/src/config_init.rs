/// 配置初始化模块，负责从文件加载和解析配置信息
use server_global::global;
use std::path::Path;
use thiserror::Error;
use tokio::fs;

use crate::{
    model::{Config, OptionalConfigs},
    project_error, project_info, DatabaseConfig, DatabasesInstancesConfig, JwtConfig, MongoConfig,
    MongoInstancesConfig, RedisConfig, RedisInstancesConfig, S3Config, S3InstancesConfig,
    ServerConfig,
};

/// 配置错误类型，用于处理配置加载和解析过程中可能出现的错误
#[derive(Debug, Error)]
pub enum ConfigError {
    /// 读取配置文件失败
    #[error("Failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),
    /// YAML格式解析失败
    #[error("Failed to parse YAML config: {0}")]
    YamlError(#[from] serde_yaml::Error),
    /// TOML格式解析失败
    #[error("Failed to parse TOML config: {0}")]
    TomlError(#[from] toml::de::Error),
    /// JSON格式解析失败
    #[error("Failed to parse JSON config: {0}")]
    JsonError(#[from] serde_json::Error),
    /// 不支持的配置文件格式
    #[error("Unsupported config file format: {0}")]
    UnsupportedFormat(String),
}

/// 根据文件扩展名解析配置文件内容
/// 
/// # 参数
/// * `file_path` - 配置文件路径
/// * `content` - 配置文件内容
/// 
/// # 返回
/// * `Result<Config, ConfigError>` - 解析后的配置对象或错误
async fn parse_config(file_path: &str, content: String) -> Result<Config, ConfigError> {
    let extension = Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "yaml" | "yml" => Ok(serde_yaml::from_str(&content)?),
        "toml" => Ok(toml::from_str(&content)?),
        "json" => Ok(serde_json::from_str(&content)?),
        _ => Err(ConfigError::UnsupportedFormat(extension)),
    }
}

/// 从文件初始化配置
/// 
/// 该函数会：
/// 1. 读取配置文件
/// 2. 解析配置内容
/// 3. 初始化全局配置
/// 4. 设置数据库、Redis、MongoDB等服务的配置
/// 
/// # 参数
/// * `file_path` - 配置文件路径
/// 
/// # 返回
/// * `Result<(), ConfigError>` - 初始化结果
pub async fn init_from_file(file_path: &str) -> Result<(), ConfigError> {
    // 读取配置文件内容
    let config_data = fs::read_to_string(file_path).await.map_err(|e| {
        project_error!("Failed to read config file: {}", e);
        ConfigError::ReadError(e)
    })?;

    // 解析配置文件
    let config = parse_config(file_path, config_data).await.map_err(|e| {
        project_error!("Failed to parse config file: {}", e);
        e
    })?;

    // 初始化全局配置
    global::init_config::<Config>(config.clone()).await;
    global::init_config::<DatabaseConfig>(config.database).await;

    // 初始化数据库实例配置
    global::init_config::<OptionalConfigs<DatabasesInstancesConfig>>(
        config.database_instances.into(),
    )
    .await;

    // 初始化服务器和JWT配置
    global::init_config::<ServerConfig>(config.server).await;
    global::init_config::<JwtConfig>(config.jwt).await;

    // 初始化Redis配置
    if let Some(redis_config) = config.redis {
        global::init_config::<RedisConfig>(redis_config).await;
    }
    global::init_config::<OptionalConfigs<RedisInstancesConfig>>(config.redis_instances.into())
        .await;

    // 初始化MongoDB配置
    if let Some(mongo_config) = config.mongo {
        global::init_config::<MongoConfig>(mongo_config).await;
    }
    global::init_config::<OptionalConfigs<MongoInstancesConfig>>(config.mongo_instances.into())
        .await;

    // 初始化S3配置
    if let Some(s3_config) = config.s3 {
        global::init_config::<S3Config>(s3_config).await;
    }
    global::init_config::<OptionalConfigs<S3InstancesConfig>>(config.s3_instances.into()).await;

    project_info!("Configuration initialized successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use log::{info, LevelFilter};
    use simplelog::{Config as LogConfig, SimpleLogger};

    use super::*;
    use crate::model::DatabaseConfig;

    static INIT: std::sync::Once = std::sync::Once::new();

    fn init_logger() {
        INIT.call_once(|| {
            SimpleLogger::init(LevelFilter::Info, LogConfig::default()).unwrap();
        });
    }

    #[cfg_attr(test, tokio::test)]
    async fn test_yaml_config() {
        init_logger();
        let result = init_from_file("examples/application.yaml").await;
        assert!(result.is_ok());
        let db_config = global::get_config::<DatabaseConfig>().await.unwrap();
        info!("db_config is {:?}", db_config);
        assert_eq!(db_config.url, "postgres://user:password@localhost/db");
    }

    #[cfg_attr(test, tokio::test)]
    async fn test_toml_config() {
        init_logger();
        let result = init_from_file("examples/application.toml").await;
        assert!(result.is_ok());
    }

    #[cfg_attr(test, tokio::test)]
    async fn test_json_config() {
        init_logger();
        let result = init_from_file("examples/application.json").await;
        assert!(result.is_ok());
    }
}
