/**
 * Nonce存储模块
 * 
 * 该模块提供了用于存储和验证nonce（一次性数字）的功能，包括：
 * - 内存存储实现
 * - Redis存储实现
 * - 存储工厂函数
 */

use std::sync::Arc;

/**
 * Nonce存储trait
 * 
 * 定义了nonce存储的基本接口
 */
#[async_trait::async_trait]
pub trait NonceStore: Send + Sync {
    /**
     * 检查并设置nonce
     *
     * 验证nonce是否有效且未被使用过，如果有效则存储它
     *
     * # 参数
     * * `nonce` - 要验证和存储的nonce字符串
     *
     * # 返回
     * * `true` - 如果nonce有效且未被使用过
     * * `false` - 如果nonce无效或已被使用过
     */
    async fn check_and_set(&self, nonce: &str) -> bool;
}

/**
 * Nonce存储枚举
 * 
 * 支持不同的存储实现，包括内存存储和Redis存储
 */
#[derive(Clone)]
pub enum NonceStoreImpl {
    /**
     * 内存存储实现
     * 
     * 使用内存存储nonce，适用于单机部署
     */
    Memory(Arc<crate::sign::memory_nonce_store::MemoryNonceStore>),
    /**
     * Redis存储实现
     * 
     * 使用Redis存储nonce，适用于分布式部署
     */
    Redis(Arc<crate::sign::redis_nonce_store::RedisNonceStore>),
}

#[async_trait::async_trait]
impl NonceStore for NonceStoreImpl {
    async fn check_and_set(&self, nonce: &str) -> bool {
        match self {
            NonceStoreImpl::Memory(store) => store.check_and_set(nonce).await,
            NonceStoreImpl::Redis(store) => store.check_and_set(nonce).await,
        }
    }
}

/**
 * Nonce存储工厂函数类型
 * 
 * 用于创建NonceStore实例的工厂函数类型
 */
pub type NonceStoreFactory = Arc<dyn Fn() -> NonceStoreImpl + Send + Sync>;

/**
 * 创建内存版本的NonceStore
 * 
 * # 返回
 * * `NonceStoreImpl` - 使用内存存储的NonceStore实例
 */
pub fn create_memory_store() -> NonceStoreImpl {
    NonceStoreImpl::Memory(Arc::new(
        crate::sign::memory_nonce_store::MemoryNonceStore::new(),
    ))
}

/**
 * 创建Redis版本的NonceStore
 * 
 * # 参数
 * * `client` - Redis客户端实例
 * * `ttl` - Nonce的过期时间
 * 
 * # 返回
 * * `NonceStoreImpl` - 使用Redis存储的NonceStore实例
 */
pub fn create_redis_store(client: redis::Client, ttl: std::time::Duration) -> NonceStoreImpl {
    NonceStoreImpl::Redis(Arc::new(
        crate::sign::redis_nonce_store::RedisNonceStore::new(client, ttl),
    ))
}

/**
 * 创建内存NonceStore的工厂函数
 * 
 * # 返回
 * * `NonceStoreFactory` - 创建内存NonceStore的工厂函数
 */
pub fn create_memory_store_factory() -> NonceStoreFactory {
    Arc::new(|| create_memory_store())
}

/**
 * 创建Redis NonceStore的工厂函数
 * 
 * # 参数
 * * `client` - Redis客户端实例
 * * `ttl` - Nonce的过期时间
 * 
 * # 返回
 * * `NonceStoreFactory` - 创建Redis NonceStore的工厂函数
 */
pub fn create_redis_store_factory(
    client: redis::Client,
    ttl: std::time::Duration,
) -> NonceStoreFactory {
    Arc::new(move || create_redis_store(client.clone(), ttl))
}
