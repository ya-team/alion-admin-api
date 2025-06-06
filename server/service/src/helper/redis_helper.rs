#![allow(dead_code)]
/**
 * Redis connection helper module
 * 
 * This module provides utilities for managing Redis connections in both single-instance and cluster modes.
 * It supports two types of Redis connections:
 * - Primary Redis instance: The main Redis connection used by the application
 * - Named Redis instances: Additional Redis connections that can be accessed by name
 * 
 * The module provides functions to get both single-instance and cluster-mode connections,
 * with proper error handling and type safety.
 */

use redis::{aio::MultiplexedConnection, cluster_async::ClusterConnection, ErrorKind, RedisError};
use server_core::web::error::AppError;
use server_global::global::{RedisConnection, GLOBAL_PRIMARY_REDIS, GLOBAL_REDIS_POOL};

/**
 * Redis connection source type
 * 
 * This enum determines which Redis instance to connect to:
 * - `Primary`: The main Redis instance used by the application
 * - `Named`: A named Redis instance from the connection pool
 */
#[derive(Debug, Clone)]
pub enum RedisSource {
    /** The primary Redis instance */
    Primary,
    /** A named Redis instance from the connection pool */
    Named(String),
}

/**
 * Gets a multiplexed Redis connection for single-instance mode
 * 
 * # Arguments
 * * `source` - The Redis source to connect to (Primary or Named)
 * 
 * # Returns
 * * `Result<MultiplexedConnection, AppError>` - A multiplexed Redis connection or an error
 * 
 * # Errors
 * * Returns an error if the Redis instance is not initialized
 * * Returns an error if trying to get a single-instance connection from a cluster-mode Redis
 */
pub async fn get_redis_connection(source: RedisSource) -> Result<MultiplexedConnection, AppError> {
    match source {
        RedisSource::Primary => {
            let redis = GLOBAL_PRIMARY_REDIS.read().await.clone().ok_or_else(|| {
                AppError::from(RedisError::from((
                    ErrorKind::IoError,
                    "Primary Redis not initialized",
                )))
            })?;
            match redis {
                RedisConnection::Single(client) => {
                    Ok(client.get_multiplexed_async_connection().await?)
                },
                RedisConnection::Cluster(_) => Err(AppError::from(RedisError::from((
                    ErrorKind::IoError,
                    "Primary Redis is not Single mode",
                )))),
            }
        },
        RedisSource::Named(name) => {
            let pools = GLOBAL_REDIS_POOL.read().await;
            let redis = pools.get(&name).ok_or_else(|| {
                AppError::from(RedisError::from((
                    ErrorKind::IoError,
                    "Redis pool not found",
                )))
            })?;
            match redis {
                RedisConnection::Single(client) => {
                    Ok(client.get_multiplexed_async_connection().await?)
                },
                RedisConnection::Cluster(_) => Err(AppError::from(RedisError::from((
                    ErrorKind::IoError,
                    "Named Redis is not Single mode",
                )))),
            }
        },
    }
}

/**
 * Gets a Redis cluster connection
 * 
 * # Arguments
 * * `source` - The Redis source to connect to (Primary or Named)
 * 
 * # Returns
 * * `Result<ClusterConnection, AppError>` - A Redis cluster connection or an error
 * 
 * # Errors
 * * Returns an error if the Redis instance is not initialized
 * * Returns an error if trying to get a cluster connection from a single-instance Redis
 */
pub async fn get_redis_cluster_connection(
    source: RedisSource,
) -> Result<ClusterConnection, AppError> {
    match source {
        RedisSource::Primary => {
            let redis = GLOBAL_PRIMARY_REDIS.read().await.clone().ok_or_else(|| {
                AppError::from(RedisError::from((
                    ErrorKind::IoError,
                    "Primary Redis not initialized",
                )))
            })?;
            match redis {
                RedisConnection::Single(_) => Err(AppError::from(RedisError::from((
                    ErrorKind::IoError,
                    "Primary Redis is not Cluster mode",
                )))),
                RedisConnection::Cluster(client) => Ok(client.get_async_connection().await?),
            }
        },
        RedisSource::Named(name) => {
            let pools = GLOBAL_REDIS_POOL.read().await;
            let redis = pools.get(&name).ok_or_else(|| {
                AppError::from(RedisError::from((
                    ErrorKind::IoError,
                    "Redis pool not found",
                )))
            })?;
            match redis {
                RedisConnection::Single(_) => Err(AppError::from(RedisError::from((
                    ErrorKind::IoError,
                    "Named Redis is not Cluster mode",
                )))),
                RedisConnection::Cluster(client) => Ok(client.get_async_connection().await?),
            }
        },
    }
}
