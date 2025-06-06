/// API签名验证模块
/// 
/// 该模块提供了API密钥验证和签名验证的功能，包括：
/// - API密钥管理
/// - 简单和复杂的签名验证
/// - Nonce存储管理
/// - API密钥中间件
/// - 事件处理

mod api_key;
mod api_key_middleware;
mod memory_nonce_store;
mod nonce_store;
mod redis_nonce_store;

pub use api_key::{
    ApiKeyConfig, ComplexApiKeyValidator, SignatureAlgorithm, SimpleApiKeyValidator,
};
pub use api_key_middleware::{
    api_key_middleware, protect_route, ApiKeySource, ApiKeyValidation, ComplexApiKeyConfig,
    SimpleApiKeyConfig,
};
pub use memory_nonce_store::{create_memory_nonce_store_factory, MemoryNonceStore};
pub use nonce_store::{NonceStore, NonceStoreFactory};
pub use redis_nonce_store::{create_redis_nonce_store_factory, RedisNonceStore};

use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 验证器类型枚举
/// 
/// 定义了支持的API密钥验证器类型：
/// - Simple: 简单验证器，仅验证API密钥
/// - Complex: 复杂验证器，支持签名验证
pub enum ValidatorType {
    /// 简单验证器
    Simple,
    /// 复杂验证器
    Complex,
}

/// 全局API密钥验证器实例
/// 
/// 使用Lazy初始化，包含简单和复杂两种验证器
static API_KEY_VALIDATORS: Lazy<(
    Arc<RwLock<SimpleApiKeyValidator>>,
    Arc<RwLock<ComplexApiKeyValidator>>,
)> = Lazy::new(|| {
    (
        Arc::new(RwLock::new(SimpleApiKeyValidator::new())),
        Arc::new(RwLock::new(ComplexApiKeyValidator::new(None))),
    )
});

/// 获取简单验证器实例
/// 
/// # 返回
/// * `SimpleApiKeyValidator` - 简单API密钥验证器
pub async fn get_simple_validator() -> SimpleApiKeyValidator {
    API_KEY_VALIDATORS.0.read().await.clone()
}

/// 获取复杂验证器实例
/// 
/// # 返回
/// * `ComplexApiKeyValidator` - 复杂API密钥验证器
pub async fn get_complex_validator() -> ComplexApiKeyValidator {
    API_KEY_VALIDATORS.1.read().await.clone()
}

/// 添加API密钥
/// 
/// # 参数
/// * `validator_type` - 验证器类型
/// * `key` - API密钥
/// * `secret` - 可选的密钥（仅用于复杂验证器）
pub async fn add_key(validator_type: ValidatorType, key: &str, secret: Option<&str>) {
    match validator_type {
        ValidatorType::Simple => {
            API_KEY_VALIDATORS.0.write().await.add_key(key.to_string());
        },
        ValidatorType::Complex => {
            if let Some(secret) = secret {
                API_KEY_VALIDATORS
                    .1
                    .write()
                    .await
                    .add_key_secret(key.to_string(), secret.to_string());
            }
        },
    }
}

/// 移除API密钥
/// 
/// # 参数
/// * `validator_type` - 验证器类型
/// * `key` - 要移除的API密钥
pub async fn remove_key(validator_type: ValidatorType, key: &str) {
    match validator_type {
        ValidatorType::Simple => {
            API_KEY_VALIDATORS.0.write().await.remove_key(key);
        },
        ValidatorType::Complex => {
            API_KEY_VALIDATORS.1.write().await.remove_key(key);
        },
    }
}

/// 初始化验证器
/// 
/// 使用默认的内存nonce存储初始化验证器
/// 
/// # 参数
/// * `config` - 可选的API密钥配置
pub async fn init_validators(config: Option<ApiKeyConfig>) {
    // 使用默认的内存 nonce 存储
    init_validators_with_nonce_store(config, create_memory_nonce_store_factory()).await;
}

/// 使用指定的nonce存储工厂函数初始化验证器
///
/// # 参数
/// * `config` - 可选的API密钥验证配置
/// * `nonce_store_factory` - 用于创建nonce存储的工厂函数
pub async fn init_validators_with_nonce_store(
    config: Option<ApiKeyConfig>,
    nonce_store_factory: NonceStoreFactory,
) {
    let complex_validator = ComplexApiKeyValidator::with_nonce_store(config, nonce_store_factory);
    *API_KEY_VALIDATORS.1.write().await = complex_validator;
}

/// API密钥事件结构体
/// 
/// 用于表示与API密钥相关的事件
#[derive(Debug, Clone)]
pub struct ApiKeyEvent {
    /// API密钥
    pub api_key: String,
}
