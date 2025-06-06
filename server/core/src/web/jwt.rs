/**
 * JWT (JSON Web Token) 处理模块
 * 
 * 该模块提供了JWT令牌的生成和验证功能，包括：
 * - 生成访问令牌：创建包含用户信息的JWT令牌
 * - 验证令牌有效性：验证令牌的签名和声明
 * - 处理令牌相关的错误：提供统一的错误处理机制
 * 
 * # 主要组件
 * 
 * ## JwtError
 * JWT操作可能出现的错误类型：
 * - 密钥未初始化
 * - 验证配置未初始化
 * - 令牌创建错误
 * - 令牌验证错误
 * 
 * ## JwtUtils
 * JWT工具类，提供令牌操作的核心功能：
 * - 生成令牌
 * - 验证令牌
 * - 处理令牌声明
 * 
 * # 使用示例
 * 
 * 
 * // 创建Claims
 * let claims = Claims::new(
 *     "user123".to_string(),
 *     "api".to_string(),
 *     "john_doe".to_string(),
 *     vec!["admin".to_string()],
 *     "example.com".to_string(),
 *     Some("org1".to_string())
 * );
 * 
 * // 生成令牌
 * let token = JwtUtils::generate_token(&claims).await?;
 * 
 * // 验证令牌
 * let token_data = JwtUtils::validate_token(&token, "api").await?;
 */

use std::{error::Error, fmt};

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Header, TokenData};
use server_config::JwtConfig;
use server_global::global;
use ulid::Ulid;

use crate::web::auth::Claims;

// pub static KEYS: Lazy<Arc<Mutex<Keys>>> = Lazy::new(|| {
//     let config = global::get_config::<JwtConfig>()
//         .expect("[alion-admin] >>>>>> [server-core] Failed to load JWT
// config");     Arc::new(Mutex::new(Keys::new(config.jwt_secret.as_bytes())))
// });
//
// pub static VALIDATION: Lazy<Arc<Mutex<Validation>>> = Lazy::new(|| {
//     let config = global::get_config::<JwtConfig>()
//         .expect("[alion-admin] >>>>>> [server-core] Failed to load JWT
// config");     let mut validation = Validation::default();
//     validation.leeway = 60;
//     validation.set_issuer(&[config.issuer.clone()]);
//     Arc::new(Mutex::new(validation))
// });

/**
 * JWT错误类型枚举
 * 
 * 定义了JWT操作过程中可能出现的错误类型。
 * 实现了Display和Error trait，支持错误信息的格式化和错误处理。
 */
#[derive(Debug)]
pub enum JwtError {
    /**
     * 密钥未初始化错误
     * 
     * 当尝试使用未初始化的JWT密钥时抛出此错误。
     */
    KeysNotInitialized,
    /**
     * 验证配置未初始化错误
     * 
     * 当尝试使用未初始化的JWT验证配置时抛出此错误。
     */
    ValidationNotInitialized,
    /**
     * 令牌创建错误
     * 
     * 在创建JWT令牌过程中发生的错误，包含具体的错误信息。
     */
    TokenCreationError(String),
    /**
     * 令牌验证错误
     * 
     * 在验证JWT令牌过程中发生的错误，包含具体的错误信息。
     */
    TokenValidationError(String),
}

impl fmt::Display for JwtError {
    /**
     * 格式化错误信息
     * 
     * 将JwtError转换为人类可读的字符串表示。
     * 
     * # 参数
     * * `f` - 格式化器
     * 
     * # 返回
     * * `fmt::Result` - 格式化结果
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JwtError::KeysNotInitialized => write!(f, "Keys not initialized"),
            JwtError::ValidationNotInitialized => write!(f, "Validation not initialized"),
            JwtError::TokenCreationError(err) => write!(f, "Token creation error: {}", err),
            JwtError::TokenValidationError(err) => write!(f, "Token validation error: {}", err),
        }
    }
}

impl Error for JwtError {}

/**
 * JWT工具类，提供令牌生成和验证功能
 * 
 * 实现了JWT令牌的核心操作，包括令牌生成和验证。
 * 使用全局配置和密钥管理，确保令牌操作的安全性。
 */
pub struct JwtUtils;

impl JwtUtils {
    /**
     * 生成JWT访问令牌
     * 
     * 根据提供的Claims生成JWT访问令牌。
     * 自动设置令牌的过期时间、签发者、签发时间等字段。
     * 
     * # 参数
     * * `claims` - 包含用户信息的Claims对象
     * 
     * # 返回
     * * `Result<String, JwtError>` - 成功返回令牌字符串，失败返回错误
     * 
     * # 错误
     * * `JwtError::KeysNotInitialized` - 密钥未初始化
     * * `JwtError::TokenCreationError` - 令牌创建失败
     */
    pub async fn generate_token(claims: &Claims) -> Result<String, JwtError> {
        let keys_arc = global::KEYS.get().ok_or(JwtError::KeysNotInitialized)?;

        let keys = keys_arc.lock().await;

        let mut claims_clone = claims.clone();

        let now = Utc::now();
        let timestamp = now.timestamp() as usize;
        let jwt_config = global::get_config::<JwtConfig>().await.unwrap();
        claims_clone.set_exp((now + Duration::seconds(jwt_config.access_token_expire as i64)).timestamp() as usize);
        claims_clone.set_iss(jwt_config.issuer.to_string());
        claims_clone.set_iat(timestamp);
        claims_clone.set_nbf(timestamp);
        claims_clone.set_jti(Ulid::new().to_string());

        let token = encode(&Header::default(), &claims_clone, &keys.encoding)
            .map_err(|e| JwtError::TokenCreationError(e.to_string()))?;

        global::send_string_event(token.clone());

        Ok(token)
    }

    /**
     * 验证JWT令牌
     * 
     * 验证JWT令牌的有效性，包括签名验证和声明验证。
     * 验证令牌的受众（audience）是否匹配。
     * 
     * # 参数
     * * `token` - 要验证的令牌字符串
     * * `audience` - 令牌的目标受众
     * 
     * # 返回
     * * `Result<TokenData<Claims>, JwtError>` - 成功返回解析后的令牌数据，失败返回错误
     * 
     * # 错误
     * * `JwtError::KeysNotInitialized` - 密钥未初始化
     * * `JwtError::ValidationNotInitialized` - 验证配置未初始化
     * * `JwtError::TokenValidationError` - 令牌验证失败
     */
    pub async fn validate_token(
        token: &str,
        audience: &str,
    ) -> Result<TokenData<Claims>, JwtError> {
        let keys_arc = global::KEYS.get().ok_or(JwtError::KeysNotInitialized)?;

        let keys = keys_arc.lock().await;
        let validation_arc = global::VALIDATION
            .get()
            .ok_or(JwtError::ValidationNotInitialized)?;
        let validation = validation_arc.lock().await;

        let mut validation_clone = validation.clone();
        validation_clone.set_audience(&[audience.to_string()]);
        decode::<Claims>(token, &keys.decoding, &validation_clone)
            .map_err(|e| JwtError::TokenValidationError(e.to_string()))
    }
}
