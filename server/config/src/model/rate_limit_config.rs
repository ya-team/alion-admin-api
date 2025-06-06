/**
 * 速率限制配置模块
 * 
 * 定义了API请求速率限制的相关参数
 * 用于防止API滥用和DoS攻击
 */

use serde::Deserialize;

/**
 * 速率限制配置结构体
 * 
 * 包含API请求速率限制所需的所有参数，包括：
 * - 全局限制
 * - IP限制
 * - 用户限制
 * - 自定义限制
 */
#[derive(Deserialize, Debug, Clone)]
pub struct RateLimitConfig {
    /**
     * 是否启用速率限制
     * 
     * 控制是否启用API请求速率限制功能
     * 建议在生产环境中启用
     */
    pub enabled: bool,

    /**
     * 全局请求限制
     * 
     * 所有请求共享的限制配置
     * 用于限制整个API的总体请求速率
     */
    pub global: Option<LimitConfig>,

    /**
     * IP地址限制
     * 
     * 基于客户端IP地址的限制配置
     * 用于防止单个IP地址的滥用
     */
    pub ip: Option<LimitConfig>,

    /**
     * 用户限制
     * 
     * 基于用户身份的限制配置
     * 用于限制已认证用户的请求速率
     */
    pub user: Option<LimitConfig>,

    /**
     * 自定义限制规则
     * 
     * 针对特定路径或方法的自定义限制配置
     * 可以设置不同的限制规则
     */
    pub custom: Vec<CustomLimitConfig>,

    /**
     * 限制存储类型
     * 
     * 指定用于存储限制计数器的存储类型
     * 支持内存存储和Redis存储
     */
    pub storage: LimitStorage,
}

/**
 * 限制配置结构体
 * 
 * 定义了基本的速率限制参数
 */
#[derive(Deserialize, Debug, Clone)]
pub struct LimitConfig {
    /**
     * 时间窗口（秒）
     * 
     * 限制计数的时间窗口大小
     * 例如：60表示每分钟的限制
     */
    pub window: u32,

    /**
     * 最大请求数
     * 
     * 在时间窗口内允许的最大请求数
     * 超过此限制的请求将被拒绝
     */
    pub max_requests: u32,

    /**
     * 是否启用突发限制
     * 
     * 控制是否允许请求突发
     * 启用后可以更好地处理流量峰值
     */
    pub burst: bool,

    /**
     * 突发请求数
     * 
     * 允许的突发请求数量
     * 仅在启用突发限制时有效
     */
    pub burst_size: Option<u32>,
}

/**
 * 自定义限制配置结构体
 * 
 * 定义了针对特定路径或方法的自定义限制规则
 */
#[derive(Deserialize, Debug, Clone)]
pub struct CustomLimitConfig {
    /**
     * 限制规则名称
     * 
     * 用于标识此限制规则的唯一名称
     */
    pub name: String,

    /**
     * 匹配的路径模式
     * 
     * 使用通配符匹配的URL路径
     * 例如：
     * - "/api/v1/*"
     * - "/api/v1/users/*"
     */
    pub path_pattern: String,

    /**
     * 匹配的HTTP方法
     * 
     * 指定此规则适用的HTTP方法
     * 例如：["GET", "POST"]
     * 如果为None，则适用于所有方法
     */
    pub methods: Option<Vec<String>>,

    /**
     * 限制配置
     * 
     * 此规则的具体限制参数
     */
    pub limit: LimitConfig,
}

/**
 * 限制存储类型枚举
 * 
 * 定义了用于存储限制计数器的存储类型
 */
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum LimitStorage {
    /**
     * 内存存储
     * 
     * 使用内存存储限制计数器
     * 优点：快速、简单
     * 缺点：不适用于多实例部署
     */
    Memory,

    /**
     * Redis存储
     * 
     * 使用Redis存储限制计数器
     * 优点：支持分布式部署
     * 缺点：需要额外的Redis服务
     */
    Redis,
} 