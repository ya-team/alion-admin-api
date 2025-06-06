/// Redis Nonce存储模块
/// 
/// 该模块提供了基于Redis的nonce存储实现，支持单机和集群模式。
/// 使用Redis的SET NX命令来确保nonce的唯一性，并设置TTL来防止重放攻击。

use server_global::global::RedisConnection;

use super::api_key::NONCE_TTL_SECS;

/// Redis Nonce存储结构体
/// 
/// 使用Redis来存储nonce，支持键前缀以区分不同的应用或环境
#[derive(Clone)]
pub struct RedisNonceStore {
    /// Redis键前缀
    /// 
    /// 用于区分不同应用或环境的nonce存储
    prefix: String,
}

impl RedisNonceStore {
    /// 创建新的RedisNonceStore实例
    /// 
    /// # 参数
    /// * `prefix` - Redis键前缀
    /// 
    /// # 返回
    /// * `Self` - 新的RedisNonceStore实例
    #[inline]
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
        }
    }

    /// 获取完整的Redis键名
    /// 
    /// # 参数
    /// * `nonce` - nonce字符串
    /// 
    /// # 返回
    /// * `String` - 格式化的Redis键名
    fn get_key(&self, nonce: &str) -> String {
        format!("{}_nonce:{}", self.prefix, nonce)
    }
}

impl Default for RedisNonceStore {
    /// 创建默认的RedisNonceStore实例
    /// 
    /// 使用默认前缀"api_key"
    /// 
    /// # 返回
    /// * `Self` - 新的RedisNonceStore实例
    #[inline]
    fn default() -> Self {
        Self::new("api_key")
    }
}

impl RedisNonceStore {
    /// 验证并存储nonce
    ///
    /// 使用Redis的SET NX命令来确保nonce的唯一性，并设置TTL
    ///
    /// # 参数
    /// * `nonce` - 要验证和存储的nonce字符串
    ///
    /// # 返回
    /// * `true` - 如果nonce有效且未被使用过
    /// * `false` - 如果nonce无效或已被使用过，或Redis连接失败
    pub async fn check_and_set(&self, nonce: &str) -> bool {
        let redis_connection = match server_global::global::GLOBAL_PRIMARY_REDIS
            .read()
            .await
            .clone()
        {
            Some(conn) => conn,
            None => return false,
        };

        let key = self.get_key(nonce);

        match redis_connection {
            RedisConnection::Single(client) => {
                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                    let result: Option<bool> = redis::cmd("SET")
                        .arg(&key)
                        .arg("1")
                        .arg("NX")
                        .arg("EX")
                        .arg(NONCE_TTL_SECS)
                        .query_async(&mut conn)
                        .await
                        .ok();

                    result.unwrap_or(false)
                } else {
                    false
                }
            },
            RedisConnection::Cluster(client) => {
                if let Ok(mut conn) = client.get_async_connection().await {
                    let result: Option<bool> = redis::cmd("SET")
                        .arg(&key)
                        .arg("1")
                        .arg("NX")
                        .arg("EX")
                        .arg(NONCE_TTL_SECS)
                        .query_async(&mut conn)
                        .await
                        .ok();

                    result.unwrap_or(false)
                } else {
                    false
                }
            },
        }
    }
}

/// 创建Redis NonceStore的工厂函数
/// 
/// 注意：此函数已被nonce_store.rs中的函数替代
/// 为了向后兼容而保留，现在只是调用nonce_store.rs中的函数
/// 
/// # 参数
/// * `prefix` - Redis键前缀
/// 
/// # 返回
/// * `NonceStoreFactory` - 创建Redis NonceStore的工厂函数
pub fn create_redis_nonce_store_factory(
    prefix: impl Into<String> + Clone + Send + Sync + 'static,
) -> super::nonce_store::NonceStoreFactory {
    super::nonce_store::create_redis_store_factory(prefix)
}
