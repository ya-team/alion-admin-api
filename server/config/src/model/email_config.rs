/**
 * 邮件配置模块
 * 
 * 定义了邮件服务的相关参数
 * 用于配置邮件发送服务器和模板
 */

use serde::Deserialize;

/**
 * 邮件配置结构体
 * 
 * 包含邮件服务所需的所有参数，包括：
 * - SMTP服务器配置
 * - 邮件模板配置
 * - 发送者信息
 */
#[derive(Deserialize, Debug, Clone)]
pub struct EmailConfig {
    /**
     * 是否启用邮件服务
     * 
     * 控制是否启用邮件发送功能
     * 建议在生产环境中启用
     */
    pub enabled: bool,

    /**
     * SMTP服务器配置
     * 
     * 邮件服务器的连接参数
     */
    pub smtp: SmtpConfig,

    /**
     * 默认发件人信息
     * 
     * 邮件的默认发件人地址和名称
     */
    pub sender: SenderConfig,

    /**
     * 邮件模板配置
     * 
     * 预定义的邮件模板
     */
    pub templates: Vec<EmailTemplate>,

    /**
     * 是否启用队列
     * 
     * 控制是否使用队列发送邮件
     * 可以提高性能和可靠性
     */
    pub enable_queue: bool,

    /**
     * 队列配置
     * 
     * 邮件队列的具体参数
     * 仅在启用队列时有效
     */
    pub queue: Option<EmailQueueConfig>,

    /**
     * 重试配置
     * 
     * 邮件发送失败时的重试策略
     */
    pub retry: RetryConfig,

    /**
     * 是否启用日志
     * 
     * 控制是否记录邮件发送日志
     * 建议在生产环境中启用
     */
    pub enable_logging: bool,
}

/**
 * SMTP服务器配置结构体
 * 
 * 定义了邮件服务器的连接参数
 */
#[derive(Deserialize, Debug, Clone)]
pub struct SmtpConfig {
    /**
     * 服务器地址
     * 
     * SMTP服务器的地址
     * 例如：smtp.gmail.com
     */
    pub host: String,

    /**
     * 服务器端口
     * 
     * SMTP服务器的端口号
     * 常用端口：
     * - 25: 非加密
     * - 465: SSL/TLS
     * - 587: STARTTLS
     */
    pub port: u16,

    /**
     * 用户名
     * 
     * SMTP服务器的认证用户名
     */
    pub username: String,

    /**
     * 密码
     * 
     * SMTP服务器的认证密码
     */
    pub password: String,

    /**
     * 是否启用TLS
     * 
     * 控制是否使用TLS加密连接
     * 建议在生产环境中启用
     */
    pub enable_tls: bool,

    /**
     * 连接超时（秒）
     * 
     * 建立SMTP连接的超时时间
     */
    pub connect_timeout: u64,

    /**
     * 发送超时（秒）
     * 
     * 发送邮件的最大等待时间
     */
    pub send_timeout: u64,

    /**
     * 是否启用调试
     * 
     * 控制是否输出SMTP调试信息
     * 建议仅在开发环境启用
     */
    pub enable_debug: bool,
}

/**
 * 发件人配置结构体
 * 
 * 定义了邮件的默认发件人信息
 */
#[derive(Deserialize, Debug, Clone)]
pub struct SenderConfig {
    /**
     * 发件人邮箱地址
     * 
     * 邮件的发件人地址
     * 例如：noreply@example.com
     */
    pub email: String,

    /**
     * 发件人名称
     * 
     * 邮件的发件人显示名称
     * 例如：System Notification
     */
    pub name: String,

    /**
     * 回复地址
     * 
     * 邮件的回复地址
     * 如果为None，则使用发件人地址
     */
    pub reply_to: Option<String>,
}

/**
 * 邮件模板配置结构体
 * 
 * 定义了预定义的邮件模板
 */
#[derive(Deserialize, Debug, Clone)]
pub struct EmailTemplate {
    /**
     * 模板ID
     * 
     * 用于标识模板的唯一ID
     * 例如：welcome_email, password_reset
     */
    pub id: String,

    /**
     * 模板名称
     * 
     * 模板的显示名称
     * 例如：Welcome Email, Password Reset
     */
    pub name: String,

    /**
     * 主题模板
     * 
     * 邮件主题的模板字符串
     * 可以包含变量，如：Welcome to {app_name}!
     */
    pub subject: String,

    /**
     * 内容模板
     * 
     * 邮件正文的模板字符串
     * 可以包含HTML格式和变量
     */
    pub content: String,

    /**
     * 是否使用HTML
     * 
     * 控制邮件内容是否使用HTML格式
     */
    pub is_html: bool,

    /**
     * 附件配置
     * 
     * 模板的默认附件列表
     */
    pub attachments: Vec<EmailAttachment>,

    /**
     * 变量列表
     * 
     * 模板支持的变量列表
     * 用于验证和文档
     */
    pub variables: Vec<String>,
}

/**
 * 邮件附件配置结构体
 * 
 * 定义了邮件附件的参数
 */
#[derive(Deserialize, Debug, Clone)]
pub struct EmailAttachment {
    /**
     * 附件ID
     * 
     * 用于标识附件的唯一ID
     */
    pub id: String,

    /**
     * 文件名
     * 
     * 附件的显示文件名
     */
    pub filename: String,

    /**
     * 文件路径
     * 
     * 附件文件的存储路径
     * 可以是相对路径或绝对路径
     */
    pub path: String,

    /**
     * 内容类型
     * 
     * 附件的MIME类型
     * 例如：application/pdf
     */
    pub content_type: String,

    /**
     * 是否内联
     * 
     * 控制附件是否作为内联内容显示
     * 通常用于图片等需要直接显示的内容
     */
    pub is_inline: bool,
}

/**
 * 邮件队列配置结构体
 * 
 * 定义了邮件队列的具体参数
 */
#[derive(Deserialize, Debug, Clone)]
pub struct EmailQueueConfig {
    /**
     * 队列名称
     * 
     * 邮件队列的名称
     * 用于区分不同的队列
     */
    pub name: String,

    /**
     * 最大并发数
     * 
     * 同时处理的最大邮件数
     */
    pub max_concurrent: usize,

    /**
     * 批处理大小
     * 
     * 每次处理的最大邮件数
     */
    pub batch_size: usize,

    /**
     * 批处理间隔（秒）
     * 
     * 两次批处理之间的等待时间
     */
    pub batch_interval: u64,

    /**
     * 是否启用持久化
     * 
     * 控制是否将队列持久化到存储
     * 可以提高可靠性
     */
    pub enable_persistence: bool,

    /**
     * 存储类型
     * 
     * 队列的存储类型
     * 支持内存存储和Redis存储
     */
    pub storage: QueueStorage,
}

/**
 * 重试配置结构体
 * 
 * 定义了邮件发送失败时的重试策略
 */
#[derive(Deserialize, Debug, Clone)]
pub struct RetryConfig {
    /**
     * 最大重试次数
     * 
     * 发送失败时的最大重试次数
     */
    pub max_attempts: u32,

    /**
     * 初始延迟（秒）
     * 
     * 第一次重试的等待时间
     */
    pub initial_delay: u64,

    /**
     * 最大延迟（秒）
     * 
     * 重试等待时间的上限
     */
    pub max_delay: u64,

    /**
     * 延迟倍数
     * 
     * 每次重试后延迟时间的增长倍数
     * 例如：2表示每次重试等待时间翻倍
     */
    pub delay_multiplier: f64,

    /**
     * 是否使用随机抖动
     * 
     * 控制是否在重试延迟中添加随机值
     * 可以避免多个失败请求同时重试
     */
    pub use_jitter: bool,

    /**
     * 抖动范围（百分比）
     * 
     * 随机抖动的范围
     * 例如：0.1表示±10%的随机值
     */
    pub jitter_range: f64,
}

/**
 * 队列存储类型枚举
 * 
 * 定义了邮件队列的存储类型
 */
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum QueueStorage {
    /**
     * 内存存储
     * 
     * 使用内存存储队列数据
     * 优点：快速、简单
     * 缺点：不适用于多实例部署
     */
    Memory,

    /**
     * Redis存储
     * 
     * 使用Redis存储队列数据
     * 优点：支持分布式部署
     * 缺点：需要额外的Redis服务
     */
    Redis,
} 