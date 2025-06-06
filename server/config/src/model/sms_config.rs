/// 短信配置模块
/// 
/// 定义了短信服务的相关参数
/// 用于配置短信发送服务和模板

use serde::Deserialize;

/// 短信配置结构体
/// 
/// 包含短信服务所需的所有参数，包括：
/// - 短信服务商配置
/// - 短信模板配置
/// - 发送策略
#[derive(Deserialize, Debug, Clone)]
pub struct SmsConfig {
    /// 是否启用短信服务
    /// 
    /// 控制是否启用短信发送功能
    /// 建议在生产环境中启用
    pub enabled: bool,

    /// 短信服务商类型
    /// 
    /// 指定使用的短信服务商
    /// 支持多个主流短信服务商
    pub provider: SmsProvider,

    /// 服务商配置
    /// 
    /// 短信服务商的具体配置参数
    pub provider_config: ProviderConfig,

    /// 短信模板配置
    /// 
    /// 预定义的短信模板
    pub templates: Vec<SmsTemplate>,

    /// 是否启用队列
    /// 
    /// 控制是否使用队列发送短信
    /// 可以提高性能和可靠性
    pub enable_queue: bool,

    /// 队列配置
    /// 
    /// 短信队列的具体参数
    /// 仅在启用队列时有效
    pub queue: Option<SmsQueueConfig>,

    /// 重试配置
    /// 
    /// 短信发送失败时的重试策略
    pub retry: RetryConfig,

    /// 是否启用日志
    /// 
    /// 控制是否记录短信发送日志
    /// 建议在生产环境中启用
    pub enable_logging: bool,

    /// 是否启用黑名单
    /// 
    /// 控制是否启用手机号黑名单功能
    /// 用于防止短信轰炸
    pub enable_blacklist: bool,

    /// 黑名单配置
    /// 
    /// 手机号黑名单的具体参数
    /// 仅在启用黑名单时有效
    pub blacklist: Option<BlacklistConfig>,
}

/// 短信服务商枚举
/// 
/// 定义了支持的短信服务商类型
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum SmsProvider {
    /// 阿里云短信服务
    /// 
    /// 使用阿里云提供的短信服务
    /// 支持国内和国际短信
    Aliyun,

    /// 腾讯云短信服务
    /// 
    /// 使用腾讯云提供的短信服务
    /// 支持国内和国际短信
    Tencent,

    /// 华为云短信服务
    /// 
    /// 使用华为云提供的短信服务
    /// 支持国内和国际短信
    Huawei,

    /// 自定义短信服务
    /// 
    /// 使用自定义的短信服务
    /// 需要实现相应的接口
    Custom,
}

/// 服务商配置结构体
/// 
/// 定义了短信服务商的具体配置参数
#[derive(Deserialize, Debug, Clone)]
pub struct ProviderConfig {
    /// 访问密钥ID
    /// 
    /// 服务商的访问密钥ID
    /// 用于API认证
    pub access_key_id: String,

    /// 访问密钥密码
    /// 
    /// 服务商的访问密钥密码
    /// 用于API认证
    pub access_key_secret: String,

    /// 签名名称
    /// 
    /// 短信签名的名称
    /// 需要预先在服务商平台申请
    pub sign_name: String,

    /// 区域ID
    /// 
    /// 服务商的区域ID
    /// 例如：cn-hangzhou
    pub region_id: String,

    /// 自定义端点
    /// 
    /// 自定义的API端点
    /// 仅在provider为Custom时有效
    pub custom_endpoint: Option<String>,

    /// 连接超时（秒）
    /// 
    /// 建立API连接的超时时间
    pub connect_timeout: u64,

    /// 请求超时（秒）
    /// 
    /// API请求的最大等待时间
    pub request_timeout: u64,

    /// 是否启用调试
    /// 
    /// 控制是否输出API调试信息
    /// 建议仅在开发环境启用
    pub enable_debug: bool,
}

/// 短信模板配置结构体
/// 
/// 定义了预定义的短信模板
#[derive(Deserialize, Debug, Clone)]
pub struct SmsTemplate {
    /// 模板ID
    /// 
    /// 用于标识模板的唯一ID
    /// 例如：verification_code, notification
    pub id: String,

    /// 模板名称
    /// 
    /// 模板的显示名称
    /// 例如：Verification Code, System Notification
    pub name: String,

    /// 模板代码
    /// 
    /// 服务商平台上的模板代码
    /// 需要预先在服务商平台申请
    pub code: String,

    /// 模板内容
    /// 
    /// 短信模板的内容
    /// 可以包含变量，如：您的验证码是：{code}
    pub content: String,

    /// 变量列表
    /// 
    /// 模板支持的变量列表
    /// 用于验证和文档
    pub variables: Vec<String>,

    /// 模板类型
    /// 
    /// 短信模板的类型
    /// 例如：验证码、通知、营销等
    pub template_type: TemplateType,

    /// 是否启用
    /// 
    /// 控制此模板是否可用
    pub enabled: bool,
}

/// 模板类型枚举
/// 
/// 定义了短信模板的类型
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum TemplateType {
    /// 验证码
    /// 
    /// 用于发送验证码的模板
    /// 例如：登录验证码、注册验证码
    VerificationCode,

    /// 通知
    /// 
    /// 用于发送通知的模板
    /// 例如：系统通知、订单通知
    Notification,

    /// 营销
    /// 
    /// 用于发送营销信息的模板
    /// 例如：活动通知、优惠信息
    Marketing,

    /// 其他
    /// 
    /// 其他类型的模板
    Other,
}

/// 短信队列配置结构体
/// 
/// 定义了短信队列的具体参数
#[derive(Deserialize, Debug, Clone)]
pub struct SmsQueueConfig {
    /// 队列名称
    /// 
    /// 短信队列的名称
    /// 用于区分不同的队列
    pub name: String,

    /// 最大并发数
    /// 
    /// 同时处理的最大短信数
    pub max_concurrent: usize,

    /// 批处理大小
    /// 
    /// 每次处理的最大短信数
    pub batch_size: usize,

    /// 批处理间隔（秒）
    /// 
    /// 两次批处理之间的等待时间
    pub batch_interval: u64,

    /// 是否启用持久化
    /// 
    /// 控制是否将队列持久化到存储
    /// 可以提高可靠性
    pub enable_persistence: bool,

    /// 存储类型
    /// 
    /// 队列的存储类型
    /// 支持内存存储和Redis存储
    pub storage: QueueStorage,
}

/// 重试配置结构体
/// 
/// 定义了短信发送失败时的重试策略
#[derive(Deserialize, Debug, Clone)]
pub struct RetryConfig {
    /// 最大重试次数
    /// 
    /// 发送失败时的最大重试次数
    pub max_attempts: u32,

    /// 初始延迟（秒）
    /// 
    /// 第一次重试的等待时间
    pub initial_delay: u64,

    /// 最大延迟（秒）
    /// 
    /// 重试等待时间的上限
    pub max_delay: u64,

    /// 延迟倍数
    /// 
    /// 每次重试后延迟时间的增长倍数
    /// 例如：2表示每次重试等待时间翻倍
    pub delay_multiplier: f64,

    /// 是否使用随机抖动
    /// 
    /// 控制是否在重试延迟中添加随机值
    /// 可以避免多个失败请求同时重试
    pub use_jitter: bool,

    /// 抖动范围（百分比）
    /// 
    /// 随机抖动的范围
    /// 例如：0.1表示±10%的随机值
    pub jitter_range: f64,
}

/// 黑名单配置结构体
/// 
/// 定义了手机号黑名单的具体参数
#[derive(Deserialize, Debug, Clone)]
pub struct BlacklistConfig {
    /// 最大发送次数
    /// 
    /// 同一手机号在时间窗口内的最大发送次数
    pub max_sends: u32,

    /// 时间窗口（秒）
    /// 
    /// 限制计数的时间窗口大小
    /// 例如：3600表示每小时的限制
    pub window: u64,

    /// 黑名单有效期（天）
    /// 
    /// 手机号被加入黑名单后的有效期
    pub blacklist_duration: u32,

    /// 是否启用自动解除
    /// 
    /// 控制是否自动解除黑名单
    /// 启用后会在有效期后自动解除
    pub enable_auto_remove: bool,

    /// 是否启用白名单
    /// 
    /// 控制是否启用白名单功能
    /// 白名单中的手机号不受限制
    pub enable_whitelist: bool,

    /// 白名单列表
    /// 
    /// 不受限制的手机号列表
    /// 仅在启用白名单时有效
    pub whitelist: Vec<String>,
}

/// 队列存储类型枚举
/// 
/// 定义了短信队列的存储类型
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum QueueStorage {
    /// 内存存储
    /// 
    /// 使用内存存储队列数据
    /// 优点：快速、简单
    /// 缺点：不适用于多实例部署
    Memory,

    /// Redis存储
    /// 
    /// 使用Redis存储队列数据
    /// 优点：支持分布式部署
    /// 缺点：需要额外的Redis服务
    Redis,
} 