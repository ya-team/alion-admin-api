/// Redis配置模块
/// 
/// 定义了Redis缓存服务的连接参数
/// 支持单机模式和集群模式的配置

use serde::Deserialize;

/// Redis配置结构体
/// 
/// 包含Redis服务连接所需的所有参数，包括：
/// - 运行模式（单机/集群）
/// - 连接URL
/// - 集群节点列表
#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    /// Redis运行模式
    /// 
    /// 指定Redis服务的部署模式
    /// - Single: 单机模式，连接到单个Redis服务器
    /// - Cluster: 集群模式，连接到Redis集群
    pub mode: RedisMode,

    /// Redis连接URL
    /// 
    /// 支持以下格式：
    /// 1. 标准TCP连接:
    ///    redis://[<username>][:<password>@]<hostname>[:port][/[<db>][?protocol=<protocol>]]
    ///    示例：
    ///    - 基本连接：redis://127.0.0.1:6379/0
    ///    - 带密码：redis://:password@127.0.0.1:6379/0
    ///    - 带用户名和密码：redis://username:password@127.0.0.1:6379/0
    ///
    /// 2. Unix Socket连接（如果系统支持）:
    ///    redis+unix:///<path>[?db=<db>[&pass=<password>][&user=<username>][&protocol=<protocol>]]
    ///    或
    ///    unix:///<path>[?db=<db>][&pass=<password>][&user=<username>][&protocol=<protocol>]]
    /// 
    /// 注意：仅在单机模式下使用
    pub url: Option<String>,

    /// Redis集群节点地址列表
    /// 
    /// 每个地址都支持与url相同的格式
    /// 仅在集群模式下使用
    ///
    /// 注意事项：
    /// - 集群模式下，db参数将被忽略，因为Redis集群不支持多数据库
    /// - 所有节点应使用相同的认证信息（用户名/密码）
    /// - 建议配置多个节点以提高可用性
    pub urls: Option<Vec<String>>,
}

/// Redis运行模式枚举
/// 
/// 定义了Redis服务的部署模式
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum RedisMode {
    /// 单机模式
    /// 
    /// 连接到单个Redis服务器
    /// 适用于开发环境或小型应用
    #[serde(rename = "single")]
    Single,

    /// 集群模式
    /// 
    /// 连接到Redis集群
    /// 适用于生产环境，提供高可用性和可扩展性
    #[serde(rename = "cluster")]
    Cluster,
}

/// Redis实例配置结构体
/// 
/// 用于配置多个命名的Redis连接
/// 每个实例可以有不同的配置参数
#[derive(Debug, Clone, Deserialize)]
pub struct RedisInstancesConfig {
    /// 实例名称
    /// 
    /// 用于标识此Redis实例的唯一名称
    /// 例如：cache, session, queue等
    pub name: String,

    /// Redis配置
    /// 
    /// 此实例的具体Redis配置参数
    pub redis: RedisConfig,
}

impl RedisConfig {
    /// 检查是否为集群模式
    /// 
    /// 返回true表示当前配置为集群模式
    pub fn is_cluster(&self) -> bool {
        self.mode == RedisMode::Cluster
    }

    /// 获取单机模式的连接URL
    /// 
    /// 仅在单机模式下返回Some(url)
    /// 集群模式下返回None
    pub fn get_url(&self) -> Option<String> {
        match self.mode {
            RedisMode::Single => self.url.clone(),
            RedisMode::Cluster => None,
        }
    }

    /// 获取集群模式的节点URL列表
    /// 
    /// 仅在集群模式下返回Some(urls)
    /// 单机模式下返回None
    pub fn get_urls(&self) -> Option<Vec<String>> {
        match self.mode {
            RedisMode::Single => None,
            RedisMode::Cluster => self.urls.clone(),
        }
    }
}
