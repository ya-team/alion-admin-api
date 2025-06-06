/**
 * Redis Nonce存储实现
 * 
 * 该模块提供了基于Redis的Nonce存储实现，用于防止重放攻击。
 * 使用Redis的TTL特性自动处理Nonce的过期。
 */

use crate::sign::nonce_store::NonceStore;
use redis::{Client, RedisError};
use std::time::Duration;
use async_trait::async_trait;

/**
 * Redis Nonce存储结构体
 * 
 * 使用Redis作为后端存储，实现Nonce的存储和验证
 */
pub struct RedisNonceStore {
    /**
     * Redis客户端实例
     */
    client: Client,
    /**
     * Nonce的过期时间
     */
    ttl: Duration,
}

impl RedisNonceStore {
    /**
     * 创建新的Redis Nonce存储实例
     * 
     * # 参数
     * * `client` - Redis客户端实例
     * * `ttl` - Nonce的过期时间
     * 
     * # 返回
     * * `Self` - Redis Nonce存储实例
     */
    pub fn new(client: Client, ttl: Duration) -> Self {
        Self { client, ttl }
    }

    /**
     * 获取Redis连接
     * 
     * # 返回
     * * `Result<redis::Connection, RedisError>` - Redis连接结果
     */
    fn get_connection(&self) -> Result<redis::Connection, RedisError> {
        self.client.get_connection()
    }
}

#[async_trait]
impl NonceStore for RedisNonceStore {
    /**
     * 检查并设置Nonce
     * 
     * 如果Nonce不存在，则设置它并返回true；
     * 如果Nonce已存在，则返回false。
     * 
     * # 参数
     * * `nonce` - 要检查的Nonce值
     * 
     * # 返回
     * * `bool` - 如果Nonce有效且未被使用过返回true，否则返回false
     */
    async fn check_and_set(&self, nonce: &str) -> bool {
        let mut conn = match self.get_connection() {
            Ok(conn) => conn,
            Err(_) => return false,
        };
        
        let key = format!("nonce:{}", nonce);
        
        // 使用SETNX命令，如果key不存在则设置
        let result: bool = match redis::cmd("SETNX")
            .arg(&key)
            .arg("1")
            .query(&mut conn) {
                Ok(result) => result,
                Err(_) => return false,
            };
            
        if result {
            // 设置过期时间
            if let Err(_) = redis::cmd("EXPIRE")
                .arg(&key)
                .arg(self.ttl.as_secs() as usize)
                .query::<()>(&mut conn) {
                    return false;
                }
        }
        
        result
    }
}

/**
 * 创建Redis Nonce存储工厂函数
 * 
 * # 参数
 * * `client` - Redis客户端实例
 * * `ttl` - Nonce的过期时间
 * 
 * # 返回
 * * `NonceStoreFactory` - 创建Redis Nonce存储的工厂函数
 */
pub fn create_redis_nonce_store_factory(
    client: Client,
    ttl: Duration,
) -> super::nonce_store::NonceStoreFactory {
    super::nonce_store::create_redis_store_factory(client, ttl)
}
