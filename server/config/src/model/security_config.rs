/// 安全配置模块
/// 
/// 定义了应用程序安全相关的参数
/// 包括密码策略、会话管理、安全头部等配置

use serde::Deserialize;

/// 安全配置结构体
/// 
/// 包含应用程序安全所需的所有参数，包括：
/// - 密码策略
/// - 会话管理
/// - 安全头部
/// - 其他安全设置
#[derive(Deserialize, Debug, Clone)]
pub struct SecurityConfig {
    /// 密码策略配置
    /// 
    /// 定义用户密码的复杂度要求和有效期
    pub password: PasswordPolicy,

    /// 会话配置
    /// 
    /// 定义用户会话的管理策略
    pub session: SessionConfig,

    /// 安全头部配置
    /// 
    /// 定义HTTP响应头中的安全相关头部
    pub headers: SecurityHeaders,

    /// 是否启用CSRF保护
    /// 
    /// 控制是否启用跨站请求伪造保护
    /// 建议在生产环境中启用
    pub enable_csrf: bool,

    /// 是否启用XSS保护
    /// 
    /// 控制是否启用跨站脚本攻击保护
    /// 建议在生产环境中启用
    pub enable_xss: bool,

    /// 是否启用SQL注入保护
    /// 
    /// 控制是否启用SQL注入攻击保护
    /// 建议在生产环境中启用
    pub enable_sql_injection: bool,

    /// 是否启用请求限流
    /// 
    /// 控制是否启用请求速率限制
    /// 建议在生产环境中启用
    pub enable_rate_limit: bool,

    /// 是否启用IP黑名单
    /// 
    /// 控制是否启用IP地址黑名单功能
    /// 用于阻止恶意IP访问
    pub enable_ip_blacklist: bool,

    /// 是否启用审计日志
    /// 
    /// 控制是否记录安全相关的审计日志
    /// 建议在生产环境中启用
    pub enable_audit_log: bool,
}

/// 密码策略配置结构体
/// 
/// 定义了用户密码的复杂度要求和有效期
#[derive(Deserialize, Debug, Clone)]
pub struct PasswordPolicy {
    /// 最小密码长度
    /// 
    /// 密码必须包含的最小字符数
    /// 建议设置为8或更长
    pub min_length: u32,

    /// 最大密码长度
    /// 
    /// 密码允许的最大字符数
    /// 建议设置为64或更短
    pub max_length: u32,

    /// 是否要求包含大写字母
    /// 
    /// 密码是否必须包含至少一个大写字母
    pub require_uppercase: bool,

    /// 是否要求包含小写字母
    /// 
    /// 密码是否必须包含至少一个小写字母
    pub require_lowercase: bool,

    /// 是否要求包含数字
    /// 
    /// 密码是否必须包含至少一个数字
    pub require_digit: bool,

    /// 是否要求包含特殊字符
    /// 
    /// 密码是否必须包含至少一个特殊字符
    pub require_special: bool,

    /// 密码有效期（天）
    /// 
    /// 密码在多少天后需要更改
    /// 如果为None，则密码永不过期
    pub expiration_days: Option<u32>,

    /// 密码历史记录数
    /// 
    /// 保存的旧密码数量，用于防止重复使用
    pub history_count: u32,

    /// 最大登录尝试次数
    /// 
    /// 允许的最大连续登录失败次数
    /// 超过此限制后账户将被锁定
    pub max_login_attempts: u32,

    /// 账户锁定时间（分钟）
    /// 
    /// 账户被锁定后的解锁等待时间
    pub lockout_duration: u32,
}

/// 会话配置结构体
/// 
/// 定义了用户会话的管理策略
#[derive(Deserialize, Debug, Clone)]
pub struct SessionConfig {
    /// 会话超时时间（分钟）
    /// 
    /// 用户会话的最大空闲时间
    /// 超过此时间后需要重新登录
    pub timeout: u32,

    /// 是否允许多设备登录
    /// 
    /// 控制是否允许同一用户在多台设备上同时登录
    pub allow_multiple_devices: bool,

    /// 最大并发会话数
    /// 
    /// 同一用户允许的最大并发会话数
    /// 仅在允许多设备登录时有效
    pub max_concurrent_sessions: Option<u32>,

    /// 是否启用会话固定保护
    /// 
    /// 控制是否在用户登录时重新生成会话ID
    /// 用于防止会话固定攻击
    pub enable_session_fixation: bool,

    /// 是否启用会话劫持保护
    /// 
    /// 控制是否验证会话的IP地址和用户代理
    /// 用于防止会话劫持攻击
    pub enable_session_hijacking: bool,

    /// 是否启用安全Cookie
    /// 
    /// 控制是否设置Cookie的Secure标志
    /// 建议在生产环境中启用
    pub secure_cookie: bool,

    /// 是否启用HttpOnly Cookie
    /// 
    /// 控制是否设置Cookie的HttpOnly标志
    /// 建议在生产环境中启用
    pub http_only_cookie: bool,

    /// Cookie域名
    /// 
    /// 设置Cookie的Domain属性
    /// 如果为None，则使用当前域名
    pub cookie_domain: Option<String>,

    /// Cookie路径
    /// 
    /// 设置Cookie的Path属性
    /// 默认为"/"
    pub cookie_path: String,
}

/// 安全头部配置结构体
/// 
/// 定义了HTTP响应头中的安全相关头部
#[derive(Deserialize, Debug, Clone)]
pub struct SecurityHeaders {
    /// 是否启用HSTS
    /// 
    /// 控制是否启用HTTP严格传输安全
    /// 建议在生产环境中启用
    pub enable_hsts: bool,

    /// HSTS最大有效期（秒）
    /// 
    /// 浏览器应该强制使用HTTPS的时间
    /// 建议设置为31536000（1年）或更长
    pub hsts_max_age: Option<u32>,

    /// 是否包含子域名
    /// 
    /// 控制HSTS是否应用于子域名
    /// 建议在生产环境中启用
    pub include_subdomains: bool,

    /// 是否启用预加载
    /// 
    /// 控制是否将域名加入HSTS预加载列表
    /// 需要谨慎使用
    pub preload: bool,

    /// 是否启用X-Frame-Options
    /// 
    /// 控制是否设置X-Frame-Options头部
    /// 用于防止点击劫持攻击
    pub enable_x_frame_options: bool,

    /// X-Frame-Options值
    /// 
    /// 设置X-Frame-Options头部的值
    /// 可选值：
    /// - "DENY": 完全禁止在框架中显示
    /// - "SAMEORIGIN": 只允许同源框架
    pub x_frame_options: Option<String>,

    /// 是否启用X-Content-Type-Options
    /// 
    /// 控制是否设置X-Content-Type-Options头部
    /// 用于防止MIME类型嗅探
    pub enable_x_content_type_options: bool,

    /// 是否启用X-XSS-Protection
    /// 
    /// 控制是否设置X-XSS-Protection头部
    /// 用于启用浏览器的XSS过滤器
    pub enable_x_xss_protection: bool,

    /// 是否启用Content-Security-Policy
    /// 
    /// 控制是否设置Content-Security-Policy头部
    /// 用于防止XSS和其他注入攻击
    pub enable_csp: bool,

    /// Content-Security-Policy值
    /// 
    /// 设置Content-Security-Policy头部的值
    /// 定义了允许加载的资源来源
    pub csp_value: Option<String>,

    /// 是否启用Referrer-Policy
    /// 
    /// 控制是否设置Referrer-Policy头部
    /// 用于控制HTTP请求中Referrer头部的信息
    pub enable_referrer_policy: bool,

    /// Referrer-Policy值
    /// 
    /// 设置Referrer-Policy头部的值
    /// 可选值：
    /// - "no-referrer": 不发送Referrer
    /// - "strict-origin-when-cross-origin": 跨域时只发送源
    /// - "same-origin": 同源时发送完整URL
    pub referrer_policy: Option<String>,
} 