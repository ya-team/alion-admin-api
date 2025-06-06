/**
 * API密钥验证模块
 * 
 * 该模块提供了API密钥验证和签名验证的功能，包括：
 * - 简单API密钥验证
 * - 复杂签名验证
 * - 多种签名算法支持
 * - 时间戳验证
 * - Nonce验证
 * - URL参数签名
 */

use md5::{Digest, Md5};
use parking_lot::RwLock;
use ring::{digest, hmac};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::sign::nonce_store::{create_memory_store_factory, NonceStore, NonceStoreFactory, NonceStoreImpl};

/**
 * 支持的签名算法
 *
 * 这些算法用于生成和验证API请求的签名。
 * 算法按性能排序（从最快到最慢）。
 */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SignatureAlgorithm {
    /**
     * MD5签名算法（默认，最快）
     */
    Md5,
    /**
     * SHA1签名算法
     */
    Sha1,
    /**
     * SHA256签名算法
     */
    Sha256,
    /**
     * HMAC-SHA256签名算法（最安全）
     */
    HmacSha256,
}

impl Default for SignatureAlgorithm {
    /**
     * 返回默认的签名算法（MD5）
     */
    #[inline]
    fn default() -> Self {
        Self::Md5
    }
}

/**
 * API密钥验证配置
 *
 * 该结构体包含API密钥验证系统的配置选项。
 * 设计为轻量级且高效可克隆。
 */
#[derive(Debug, Clone, Copy)]
pub struct ApiKeyConfig {
    /**
     * 用于请求验证的签名算法
     */
    pub algorithm: SignatureAlgorithm,
}

impl Default for ApiKeyConfig {
    /**
     * 返回默认配置（使用MD5算法）
     */
    #[inline]
    fn default() -> Self {
        Self {
            algorithm: SignatureAlgorithm::default(),
        }
    }
}

/**
 * 验证超时和过期常量
 */
pub const NONCE_TTL_SECS: u64 = 600; // 10分钟
pub const TIMESTAMP_DISPARITY_MS: i64 = 300_000; // 5分钟

/**
 * 集合的容量提示
 */
const DEFAULT_CAPACITY: usize = 32;

/**
 * 简单API密钥验证器
 *
 * 该验证器通过比较预定义的有效密钥集合来提供基本的API密钥验证。
 * 密钥永久存储，只能通过显式API调用修改。
 */
#[derive(Clone)]
pub struct SimpleApiKeyValidator {
    /**
     * 存储有效API密钥的映射
     */
    keys: Arc<RwLock<HashMap<String, ()>>>,
}

impl SimpleApiKeyValidator {
    /**
     * 创建一个新的SimpleApiKeyValidator实例，初始为空
     */
    #[inline]
    pub fn new() -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::with_capacity(DEFAULT_CAPACITY))),
        }
    }

    /**
     * 验证API密钥是否有效
     *
     * # 参数
     * * `key` - 要验证的API密钥
     *
     * # 返回
     * * `true` - 如果密钥有效
     * * `false` - 如果密钥无效
     */
    #[inline]
    pub fn validate_key(&self, key: &str) -> bool {
        self.keys.read().contains_key(key)
    }

    /**
     * 添加新的有效API密钥
     *
     * # 参数
     * * `key` - 要添加的API密钥
     */
    #[inline]
    pub fn add_key(&self, key: String) {
        self.keys.write().insert(key, ());
    }

    /**
     * 从有效密钥集合中移除API密钥
     *
     * # 参数
     * * `key` - 要移除的API密钥
     */
    #[inline]
    pub fn remove_key(&self, key: &str) {
        self.keys.write().remove(key);
    }
}

impl Default for SimpleApiKeyValidator {
    /**
     * 返回默认的SimpleApiKeyValidator实例
     */
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/**
 * 复杂API密钥验证器
 *
 * 该验证器提供高级API密钥验证功能，包括：
 * - 多种签名算法（MD5、SHA1、SHA256、HMAC-SHA256）
 * - 时间戳验证以防止重放攻击
 * - 带自动过期的Nonce验证
 * - URL参数签名
 *
 * API密钥及其对应的密钥永久存储，只能通过显式API调用修改。
 */
#[derive(Clone)]
pub struct ComplexApiKeyValidator {
    /**
     * 存储API密钥及其对应密钥的映射
     */
    secrets: Arc<RwLock<HashMap<String, String>>>,
    /**
     * Nonce存储实例
     */
    nonce_store: NonceStoreImpl,
    /**
     * Nonce存储工厂函数
     */
    nonce_store_factory: NonceStoreFactory,
    /**
     * API密钥验证配置
     */
    config: ApiKeyConfig,
}

impl ComplexApiKeyValidator {
    /**
     * 创建一个新的ComplexApiKeyValidator实例，可选配置
     *
     * # 参数
     * * `config` - 可选的API密钥验证配置。如果为None，使用默认配置
     */
    #[inline]
    pub fn new(config: Option<ApiKeyConfig>) -> Self {
        Self::with_nonce_store(config, create_memory_store_factory())
    }

    /**
     * 创建一个新的ComplexApiKeyValidator实例，使用指定的nonce存储工厂函数
     *
     * # 参数
     * * `config` - 可选的API密钥验证配置。如果为None，使用默认配置
     * * `nonce_store_factory` - 用于创建nonce存储的工厂函数
     */
    #[inline]
    pub fn with_nonce_store(
        config: Option<ApiKeyConfig>,
        nonce_store_factory: NonceStoreFactory,
    ) -> Self {
        Self {
            secrets: Arc::new(RwLock::new(HashMap::with_capacity(DEFAULT_CAPACITY))),
            nonce_store: (nonce_store_factory)(),
            nonce_store_factory,
            config: config.unwrap_or_default(),
        }
    }

    /**
     * 获取一个新的nonce存储实例
     */
    #[inline]
    pub fn get_new_nonce_store(&self) -> NonceStoreImpl {
        (self.nonce_store_factory)()
    }

    /**
     * 验证时间戳是否在允许的5分钟窗口内
     */
    #[inline]
    fn validate_timestamp(&self, timestamp: i64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        (now - timestamp).abs() < TIMESTAMP_DISPARITY_MS
    }

    /**
     * 使用配置的算法计算签名字符串的签名
     *
     * # 参数
     * * `signing_string` - 要签名的字符串
     * * `secret` - 用于签名的密钥
     *
     * # 返回
     * 计算得到的签名的十六进制字符串
     */
    #[inline]
    pub fn calculate_signature(&self, signing_string: &str, secret: &str) -> String {
        let signing_string = format!("{}&key={}", signing_string, secret);
        match self.config.algorithm {
            SignatureAlgorithm::Md5 => {
                let mut hasher = Md5::new();
                hasher.update(signing_string.as_bytes());
                hex::encode(hasher.finalize())
            },
            SignatureAlgorithm::Sha1 => {
                let mut context = digest::Context::new(&digest::SHA1_FOR_LEGACY_USE_ONLY);
                context.update(signing_string.as_bytes());
                hex::encode(context.finish())
            },
            SignatureAlgorithm::Sha256 => {
                let mut context = digest::Context::new(&digest::SHA256);
                context.update(signing_string.as_bytes());
                hex::encode(context.finish())
            },
            SignatureAlgorithm::HmacSha256 => {
                let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
                let tag = hmac::sign(&key, signing_string.as_bytes());
                hex::encode(tag.as_ref())
            },
        }
    }

    /**
     * 验证签名的API请求
     *
     * # 参数
     * * `api_key` - 要验证的API密钥
     * * `params` - 表示请求参数的键值对向量
     * * `signature` - 要验证的签名
     * * `timestamp` - 请求时间戳（UNIX纪元以来的毫秒数）
     * * `nonce` - 用于防止重放攻击的唯一请求标识符
     *
     * # 返回
     * * `true` - 如果请求有效
     * * `false` - 如果任何验证检查失败
     */
    pub fn validate_signature(
        &self,
        api_key: &str,
        params: &[(String, String)],
        signature: &str,
        timestamp: i64,
        nonce: &str,
    ) -> bool {
        if !self.validate_timestamp(timestamp) {
            return false;
        }

        let check_result = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async { self.nonce_store.check_and_set(nonce).await })
        });

        if !check_result {
            return false;
        }

        let secrets_guard = self.secrets.read();
        let secret = match secrets_guard.get(api_key) {
            Some(secret) => secret,
            None => return false,
        };

        let mut sorted_params = params.to_vec();
        sorted_params.sort_by(|a, b| a.0.cmp(&b.0));

        let signing_string = sorted_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        let calculated_signature = self.calculate_signature(&signing_string, secret);
        calculated_signature == signature
    }

    /**
     * 添加新的API密钥和密钥对
     *
     * # 参数
     * * `key` - API密钥
     * * `secret` - 对应的密钥
     */
    pub fn add_key_secret(&self, key: String, secret: String) {
        self.secrets.write().insert(key, secret);
    }

    /**
     * 移除API密钥
     *
     * # 参数
     * * `key` - 要移除的API密钥
     */
    pub fn remove_key(&self, key: &str) {
        self.secrets.write().remove(key);
    }

    /**
     * 更新验证器配置
     *
     * # 参数
     * * `config` - 新的API密钥验证配置
     */
    pub fn update_config(&mut self, config: ApiKeyConfig) {
        self.config = config;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * 测试简单API密钥验证器
     */
    #[test]
    fn test_simple_api_key_validator() {
        let validator = SimpleApiKeyValidator::new();
        validator.add_key("test_key".to_string());
        assert!(validator.validate_key("test_key"));
        assert!(!validator.validate_key("invalid_key"));
        validator.remove_key("test_key");
        assert!(!validator.validate_key("test_key"));
    }

    /**
     * 测试Nonce存储
     */
    #[tokio::test]
    async fn test_nonce_store() {
        let validator = ComplexApiKeyValidator::new(None);
        let nonce = "test_nonce";
        assert!(validator.nonce_store.check_and_set(nonce).await);
        assert!(!validator.nonce_store.check_and_set(nonce).await);
    }

    /**
     * 测试复杂验证器
     */
    #[test]
    fn test_complex_validator() {
        let validator = ComplexApiKeyValidator::new(None);
        validator.add_key_secret("test_key".to_string(), "test_secret".to_string());

        let params = vec![
            ("param1".to_string(), "value1".to_string()),
            ("param2".to_string(), "value2".to_string()),
        ];

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let nonce = "test_nonce";
        let signing_string = "param1=value1&param2=value2";
        let signature = validator.calculate_signature(signing_string, "test_secret");

        assert!(validator.validate_signature(
            "test_key",
            &params,
            &signature,
            timestamp,
            nonce
        ));
    }

    /**
     * 测试并发访问
     */
    #[test]
    fn test_concurrent_access() {
        use std::thread;

        let validator = SimpleApiKeyValidator::new();
        let mut handles = vec![];

        for i in 0..10 {
            let validator = validator.clone();
            handles.push(thread::spawn(move || {
                let key = format!("key_{}", i);
                validator.add_key(key.clone());
                assert!(validator.validate_key(&key));
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
