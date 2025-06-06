/**
 * S3配置模块
 * 
 * 定义了对象存储服务（如AWS S3、MinIO等）的连接参数
 */

use serde::Deserialize;

/**
 * S3配置结构体
 * 
 * 包含对象存储服务连接所需的所有参数，包括：
 * - 端点配置
 * - 认证信息
 * - 存储桶配置
 * - 区域设置
 */
#[derive(Deserialize, Debug, Clone)]
pub struct S3Config {
    /**
     * 服务端点URL
     * 
     * 对象存储服务的访问地址
     * 例如：
     * - AWS S3: https://s3.amazonaws.com
     * - MinIO: http://localhost:9000
     */
    pub endpoint: String,

    /**
     * 访问密钥ID
     * 
     * 用于认证的访问密钥ID
     * 需要具有适当的权限来访问存储桶
     */
    pub access_key_id: String,

    /**
     * 访问密钥密码
     * 
     * 与访问密钥ID配对的密钥密码
     * 用于签名请求
     */
    pub access_key_secret: String,

    /**
     * 存储桶名称
     * 
     * 用于存储对象的容器名称
     * 需要预先创建并配置适当的访问权限
     */
    pub bucket: String,

    /**
     * 区域名称
     * 
     * 对象存储服务的数据中心区域
     * 例如：
     * - AWS: us-east-1
     * - MinIO: 可以留空
     */
    pub region: Option<String>,

    /**
     * 是否使用HTTPS
     * 
     * 控制是否使用HTTPS协议访问服务
     * 建议在生产环境中启用
     */
    pub use_ssl: bool,

    /**
     * 自定义域名
     * 
     * 用于访问存储桶的自定义域名
     * 可以用于CDN加速或自定义访问地址
     */
    pub custom_domain: Option<String>,
}

/**
 * S3实例配置结构体
 * 
 * 用于配置多个命名的S3连接
 * 每个实例可以有不同的配置参数
 */
#[derive(Debug, Clone, Deserialize)]
pub struct S3InstancesConfig {
    /**
     * 实例名称
     * 
     * 用于标识此S3实例的唯一名称
     * 例如：images, documents, backups等
     */
    pub name: String,

    /**
     * S3配置
     * 
     * 此实例的具体S3配置参数
     */
    pub s3: S3Config,
}
