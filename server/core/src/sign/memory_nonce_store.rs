/// 内存Nonce存储模块
/// 
/// 该模块提供了基于内存的nonce存储实现，使用moka缓存来管理nonce的生命周期。
/// 主要用于防止重放攻击，同时保持内存效率。

use moka::sync::Cache;

use super::api_key::NONCE_TTL_SECS;

/// 内存Nonce存储结构体
///
/// 使用moka缓存来存储nonce，具有10分钟的TTL（生存时间）。
/// 一旦nonce过期，它可以被重用。这有助于防止重放攻击，同时保持内存效率。
#[derive(Clone)]
pub struct MemoryNonceStore {
    /// 使用moka缓存存储nonce
    /// 
    /// 键为nonce字符串，值为空元组（仅用于标记存在性）
    nonces: Cache<String, ()>,
}

impl MemoryNonceStore {
    /// 创建新的MemoryNonceStore实例
    /// 
    /// 使用10分钟的TTL初始化缓存
    /// 
    /// # 返回
    /// * `Self` - 新的MemoryNonceStore实例
    #[inline]
    pub fn new() -> Self {
        Self {
            nonces: Cache::builder()
                .time_to_live(std::time::Duration::from_secs(NONCE_TTL_SECS))
                .build(),
        }
    }

    /// 验证并存储nonce
    ///
    /// 检查nonce是否已存在，如果不存在则存储它
    ///
    /// # 参数
    /// * `nonce` - 要验证和存储的nonce字符串
    ///
    /// # 返回
    /// * `true` - 如果nonce有效且未被使用过
    /// * `false` - 如果nonce无效或已被使用过
    #[inline]
    pub async fn check_and_set(&self, nonce: &str) -> bool {
        if self.nonces.contains_key(nonce) {
            false
        } else {
            self.nonces.insert(nonce.to_string(), ());
            true
        }
    }
}

impl Default for MemoryNonceStore {
    /// 创建默认的MemoryNonceStore实例
    /// 
    /// # 返回
    /// * `Self` - 新的MemoryNonceStore实例
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// 创建内存NonceStore的工厂函数
/// 
/// 注意：此函数已被nonce_store.rs中的函数替代
/// 为了向后兼容而保留，现在只是调用nonce_store.rs中的函数
/// 
/// # 返回
/// * `NonceStoreFactory` - 创建内存NonceStore的工厂函数
pub fn create_memory_nonce_store_factory() -> super::nonce_store::NonceStoreFactory {
    super::nonce_store::create_memory_store_factory()
}
