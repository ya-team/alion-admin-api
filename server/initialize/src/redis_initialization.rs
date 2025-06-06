#![allow(dead_code)]
use redis::{cluster::ClusterClient, Client};
use server_config::{OptionalConfigs, RedisConfig, RedisInstancesConfig, RedisMode};
use server_global::global::{get_config, RedisConnection, GLOBAL_PRIMARY_REDIS, GLOBAL_REDIS_POOL};
use std::{process, sync::Arc};

use crate::{project_error, project_info};

/// 初始化主Redis
pub async fn init_primary_redis() {
    if let Some(config) = get_config::<RedisConfig>().await {
        match create_redis_connection(&config).await {
            Ok(connection) => {
                *GLOBAL_PRIMARY_REDIS.write().await = Some(connection);
                project_info!(
                    "Primary Redis connection initialized ({})",
                    if config.mode == RedisMode::Cluster {
                        "Cluster mode"
                    } else {
                        "Single mode"
                    }
                );
            },
            Err(e) => {
                project_error!("Failed to initialize primary Redis: {}", e);
                process::exit(1);
            },
        }
    }
}

async fn create_redis_connection(config: &RedisConfig) -> Result<RedisConnection, String> {
    if config.mode == RedisMode::Cluster {
        create_cluster_connection(config).await
    } else {
        create_single_connection(config).await
    }
}

async fn create_single_connection(config: &RedisConfig) -> Result<RedisConnection, String> {
    let url = config
        .get_url()
        .ok_or_else(|| "URL is required for single mode Redis".to_string())?;

    let client = redis::Client::open(url.as_str())
        .map_err(|e| format!("Failed to create Redis client: {}", e))?;

    test_single_connection(&client).await?;

    Ok(RedisConnection::Single(Arc::new(client)))
}

async fn test_single_connection(client: &Client) -> Result<(), String> {
    let mut con = client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| format!("Failed to create connection manager: {}", e))?;

    let _: String = redis::cmd("PING")
        .query_async(&mut con)
        .await
        .map_err(|e| format!("Failed to connect to Redis: {}", e))?;

    Ok(())
}

async fn create_cluster_connection(config: &RedisConfig) -> Result<RedisConnection, String> {
    let urls = config
        .get_urls()
        .ok_or_else(|| "URLs are required for cluster mode".to_string())?;

    if urls.is_empty() {
        return Err("Cluster mode requires at least one URL".to_string());
    }

    let client =
        redis::cluster::ClusterClient::new(urls.iter().map(|s| s.as_str()).collect::<Vec<_>>())
            .map_err(|e| format!("Failed to create Redis cluster client: {}", e))?;

    test_cluster_connection(&client).await?;

    Ok(RedisConnection::Cluster(Arc::new(client)))
}

async fn test_cluster_connection(client: &ClusterClient) -> Result<(), String> {
    let mut con = client
        .get_async_connection()
        .await
        .map_err(|e| format!("Failed to connect to Redis cluster: {}", e))?;

    let _: String = redis::cmd("PING")
        .query_async(&mut con)
        .await
        .map_err(|e| format!("Failed to connect to Redis: {}", e))?;

    Ok(())
}

pub async fn init_redis_pool(
    redis_instances_config: Option<Vec<RedisInstancesConfig>>,
) -> Result<(), String> {
    if let Some(redis_instances) = redis_instances_config {
        for redis_instance in redis_instances {
            init_redis_connection(&redis_instance.name, &redis_instance.redis).await?;
        }
    }
    Ok(())
}

async fn init_redis_connection(name: &str, config: &RedisConfig) -> Result<(), String> {
    match create_redis_connection(config).await {
        Ok(connection) => {
            GLOBAL_REDIS_POOL
                .write()
                .await
                .insert(name.to_string(), connection);
            project_info!("Redis '{}' initialized", name);
            Ok(())
        },
        Err(e) => {
            let error_msg = format!("Failed to initialize Redis '{}': {}", name, e);
            project_error!("{}", error_msg);
            Err(error_msg)
        },
    }
}

/// 初始化所有 Redis 连接
pub async fn init_redis_pools() {
    if let Some(redis_instances_config) =
        get_config::<OptionalConfigs<RedisInstancesConfig>>().await
    {
        if let Some(redis_instances) = &redis_instances_config.configs {
            let _ = init_redis_pool(Some(redis_instances.clone())).await;
        }
    }
}

pub async fn get_primary_redis() -> Option<RedisConnection> {
    GLOBAL_PRIMARY_REDIS.read().await.clone()
}

pub async fn get_redis_pool_connection(name: &str) -> Option<RedisConnection> {
    GLOBAL_REDIS_POOL.read().await.get(name).cloned()
}

pub async fn add_or_update_redis_pool(name: &str, config: &RedisConfig) -> Result<(), String> {
    init_redis_connection(name, config).await
}

pub async fn remove_redis_pool(name: &str) -> Result<(), String> {
    let mut redis_pool = GLOBAL_REDIS_POOL.write().await;
    redis_pool
        .remove(name)
        .ok_or_else(|| format!("Redis connection '{}' not found", name))?;
    project_info!("Redis connection '{}' removed", name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::initialize_config;
    use log::LevelFilter;
    use redis::AsyncCommands;
    use simple_logger::SimpleLogger;
    use tokio::sync::Mutex;

    static INITIALIZED: Mutex<Option<Arc<()>>> = Mutex::const_new(None);

    fn setup_logger() {
        let _ = SimpleLogger::new().with_level(LevelFilter::Info).init();
    }

    async fn init() {
        let mut initialized = INITIALIZED.lock().await;
        if initialized.is_none() {
            initialize_config("../resources/application.yaml").await;
            *initialized = Some(Arc::new(()));
        }
    }

    async fn test_redis_operations(connection: &RedisConnection) -> Result<(), String> {
        match connection {
            RedisConnection::Single(client) => {
                let mut con = client
                    .get_multiplexed_async_connection()
                    .await
                    .map_err(|e| format!("Failed to get connection: {}", e))?;
                test_redis_basic_operations(&mut con).await
            },
            RedisConnection::Cluster(client) => {
                let mut con = client
                    .get_async_connection()
                    .await
                    .map_err(|e| format!("Failed to get cluster connection: {}", e))?;
                test_redis_basic_operations(&mut con).await
            },
        }
    }

    #[allow(dependency_on_unit_never_type_fallback)]
    async fn test_redis_basic_operations<C: AsyncCommands>(con: &mut C) -> Result<(), String> {
        let test_key = "test_key";
        let test_value = "test_value";
        con.set(test_key, test_value)
            .await
            .map_err(|e| format!("Failed to set value: {}", e))?;
        project_info!("Successfully set value: {} = {}", test_key, test_value);

        let value: String = con
            .get(test_key)
            .await
            .map_err(|e| format!("Failed to get value: {}", e))?;
        assert_eq!(value, test_value, "Retrieved value does not match");
        project_info!("Successfully retrieved value: {} = {}", test_key, value);

        let deleted: bool = con
            .del(test_key)
            .await
            .map_err(|e| format!("Failed to delete key: {}", e))?;
        assert!(deleted, "Key was not deleted");
        project_info!("Successfully deleted key: {}", test_key);

        Ok(())
    }

    #[tokio::test]
    async fn test_primary_redis_connection() {
        setup_logger();
        init().await;

        init_primary_redis().await;

        let connection = get_primary_redis().await;
        assert!(
            connection.is_some(),
            "Primary Redis connection does not exist"
        );

        if let Some(conn) = connection {
            let result = test_redis_operations(&conn).await;
            assert!(
                result.is_ok(),
                "Redis operations test failed: {:?}",
                result.err()
            );
        }
    }

    #[tokio::test]
    async fn test_redis_pool_connection() {
        setup_logger();
        init().await;

        let single_config = server_config::RedisInstancesConfig {
            name: "test_single".to_string(),
            redis: RedisConfig {
                mode: RedisMode::Single,
                url: Some("redis://:123456@bytebytebrew.local:26379/11".to_string()),
                urls: None,
            },
        };

        let result = init_redis_pool(Some(vec![single_config.clone()])).await;
        assert!(
            result.is_ok(),
            "Failed to initialize Redis pool: {:?}",
            result.err()
        );

        // 测试单机连接
        let single_connection = get_redis_pool_connection("test_single").await;
        if let Some(conn) = single_connection {
            let result = test_redis_operations(&conn).await;
            assert!(
                result.is_ok(),
                "Single Redis operations test failed: {:?}",
                result.err()
            );
            project_info!("Single Redis connection test passed");
        }

        // 测试添加新连接
        let add_result = add_or_update_redis_pool("test_new", &single_config.redis).await;
        assert!(add_result.is_ok(), "Failed to add Redis connection");

        let new_connection = get_redis_pool_connection("test_new").await;
        assert!(
            new_connection.is_some(),
            "New Redis connection does not exist"
        );

        if let Some(conn) = new_connection {
            let result = test_redis_operations(&conn).await;
            assert!(
                result.is_ok(),
                "New Redis connection test failed: {:?}",
                result.err()
            );
            project_info!("New Redis connection test passed");
        }

        project_info!(
            "Current pool size: {}",
            GLOBAL_REDIS_POOL.read().await.len()
        );

        // 测试移除连接
        let remove_result = remove_redis_pool("test_new").await;
        assert!(remove_result.is_ok(), "Failed to remove Redis connection");

        let connection_after_removal = get_redis_pool_connection("test_new").await;
        assert!(
            connection_after_removal.is_none(),
            "Redis connection still exists after removal"
        );

        project_info!(
            "Current pool size after removal: {}",
            GLOBAL_REDIS_POOL.read().await.len()
        );
    }
}
