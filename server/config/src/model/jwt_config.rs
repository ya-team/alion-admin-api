/// JWT配置模块
/// 
/// 定义了JWT（JSON Web Token）相关的配置参数，用于用户认证和授权

use serde::Deserialize;

/// JWT配置结构体
/// 
/// 包含JWT令牌生成和验证所需的所有参数，包括：
/// - 密钥配置
/// - 令牌过期时间
/// - 刷新令牌配置
#[derive(Deserialize, Debug, Clone)]
pub struct JwtConfig {
    /// JWT密钥
    /// 
    /// 用于签名和验证JWT令牌的密钥
    /// 建议使用足够长的随机字符串
    pub secret: String,

    /// 访问令牌过期时间（秒）
    /// 
    /// 访问令牌的有效期，超过此时间后需要重新登录或使用刷新令牌
    /// 建议设置为较短时间，如15分钟到1小时
    pub access_token_expire: u64,

    /// 刷新令牌过期时间（秒）
    /// 
    /// 刷新令牌的有效期，用于获取新的访问令牌
    /// 建议设置为较长时间，如7天到30天
    pub refresh_token_expire: u64,

    /// 令牌签发者
    /// 
    /// JWT令牌的签发者标识，通常为应用程序名称或域名
    pub issuer: String,

    /// 令牌接收者
    /// 
    /// JWT令牌的目标接收者，通常为应用程序名称或域名
    pub audience: String,
}
