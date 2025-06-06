/// MongoDB配置模块
/// 
/// 定义了MongoDB文档数据库的连接参数
/// 支持单实例和多实例配置

use serde::Deserialize;

/// MongoDB配置结构体
/// 
/// 包含MongoDB服务连接所需的所有参数
#[derive(Debug, Clone, Deserialize)]
pub struct MongoConfig {
    /// MongoDB连接URI
    /// 
    /// 支持以下格式：
    /// mongodb://[username:password@]host1[:port1][,...hostN[:portN]][/[defaultauthdb][?options]]
    ///
    /// 示例:
    /// - 基本连接：mongodb://localhost:27017/mydb
    /// - 带认证：mongodb://user:pass@localhost:27017/mydb
    /// - 带参数：mongodb://localhost:27017/mydb?maxPoolSize=20&w=majority
    ///
    /// 常用连接选项：
    /// - maxPoolSize: 连接池大小
    /// - minPoolSize: 最小连接数
    /// - maxIdleTimeMS: 连接最大空闲时间
    /// - connectTimeoutMS: 连接超时时间
    /// - socketTimeoutMS: Socket操作超时时间
    /// - serverSelectionTimeoutMS: 服务器选择超时时间
    /// - w: 写入关注级别
    /// - wtimeoutMS: 写入超时时间
    /// - journal: 是否启用日志
    /// - readPreference: 读取偏好
    /// - replicaSet: 副本集名称
    /// - ssl: 是否使用SSL
    pub uri: String,

    /// 数据库名称
    /// 
    /// 要连接的MongoDB数据库名称
    /// 如果URI中已指定数据库，此字段将被忽略
    pub database: Option<String>,

    /// 连接池最大连接数
    /// 
    /// 控制同时可以打开的MongoDB连接数量
    /// 如果URI中已指定maxPoolSize，此字段将被忽略
    pub max_pool_size: Option<u32>,

    /// 连接超时时间（毫秒）
    /// 
    /// 建立连接时的最大等待时间
    /// 如果URI中已指定connectTimeoutMS，此字段将被忽略
    pub connect_timeout_ms: Option<u64>,

    /// 服务器选择超时时间（毫秒）
    /// 
    /// 选择服务器时的最大等待时间
    /// 如果URI中已指定serverSelectionTimeoutMS，此字段将被忽略
    pub server_selection_timeout_ms: Option<u64>,

    /// 心跳频率（毫秒）
    /// 
    /// 服务器心跳检测的时间间隔
    /// 如果URI中已指定heartbeatFrequencyMS，此字段将被忽略
    pub heartbeat_frequency_ms: Option<u64>,

    /// 是否启用SSL
    /// 
    /// 控制是否使用SSL/TLS加密连接
    /// 如果URI中已指定ssl，此字段将被忽略
    pub ssl: Option<bool>,

    /// SSL证书文件路径
    /// 
    /// SSL证书文件的路径
    /// 仅在启用SSL时有效
    pub ssl_cert_file: Option<String>,

    /// SSL密钥文件路径
    /// 
    /// SSL密钥文件的路径
    /// 仅在启用SSL时有效
    pub ssl_key_file: Option<String>,

    /// SSLCA文件路径
    /// 
    /// SSL CA证书文件的路径
    /// 仅在启用SSL时有效
    pub ssl_ca_file: Option<String>,

    /// 是否允许自签名证书
    /// 
    /// 控制是否接受自签名的SSL证书
    /// 仅在启用SSL时有效
    /// 生产环境建议设置为false
    pub ssl_allow_invalid_certificates: Option<bool>,
}

/// MongoDB实例配置结构体
/// 
/// 用于配置多个命名的MongoDB连接
/// 每个实例可以有不同的配置参数
#[derive(Debug, Clone, Deserialize)]
pub struct MongoInstancesConfig {
    /// 实例名称
    /// 
    /// 用于标识此MongoDB实例的唯一名称
    /// 例如：main, analytics, logs等
    pub name: String,

    /// MongoDB配置
    /// 
    /// 此实例的具体MongoDB配置参数
    pub mongo: MongoConfig,
}
