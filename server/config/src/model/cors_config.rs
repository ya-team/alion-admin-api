/// CORS配置模块
/// 
/// 定义了跨域资源共享（Cross-Origin Resource Sharing）的相关参数
/// 用于控制不同源之间的资源访问权限

use serde::Deserialize;

/// CORS配置结构体
/// 
/// 包含跨域资源共享所需的所有参数，包括：
/// - 允许的源
/// - 允许的方法
/// - 允许的请求头
/// - 凭证设置
#[derive(Deserialize, Debug, Clone)]
pub struct CorsConfig {
    /// 允许的源列表
    /// 
    /// 指定允许访问资源的源（域名）
    /// 例如：
    /// - ["https://example.com"]
    /// - ["http://localhost:3000"]
    /// - ["*"] 表示允许所有源（不推荐在生产环境使用）
    pub allowed_origins: Vec<String>,

    /// 允许的HTTP方法列表
    /// 
    /// 指定允许的HTTP请求方法
    /// 例如：
    /// - ["GET", "POST"]
    /// - ["GET", "POST", "PUT", "DELETE"]
    pub allowed_methods: Vec<String>,

    /// 允许的请求头列表
    /// 
    /// 指定允许客户端在请求中使用的HTTP头
    /// 例如：
    /// - ["Content-Type", "Authorization"]
    /// - ["*"] 表示允许所有请求头
    pub allowed_headers: Vec<String>,

    /// 允许暴露的响应头列表
    /// 
    /// 指定允许客户端访问的响应头
    /// 例如：
    /// - ["Content-Length", "X-Custom-Header"]
    pub exposed_headers: Vec<String>,

    /// 是否允许发送凭证
    /// 
    /// 控制是否允许跨域请求携带凭证信息（如cookies）
    /// 当设置为true时，allowed_origins不能包含"*"
    pub allow_credentials: bool,

    /// 预检请求的缓存时间（秒）
    /// 
    /// 指定预检请求（OPTIONS）的结果可以被缓存的时间
    /// 可以减少预检请求的次数
    pub max_age: u32,

    /// 是否允许私有网络访问
    /// 
    /// 控制是否允许来自私有网络的请求
    /// 例如：局域网、VPN等
    pub allow_private_network: bool,
} 