#![allow(dead_code)]
use mongodb::Client;
use server_config::{MongoConfig, MongoInstancesConfig, OptionalConfigs};
use server_global::global::{get_config, GLOBAL_MONGO_POOL, GLOBAL_PRIMARY_MONGO};
use std::{process, sync::Arc};

use crate::{project_error, project_info};

/// 初始化主MongoDB
pub async fn init_primary_mongo() {
    if let Some(config) = get_config::<MongoConfig>().await {
        match Client::with_uri_str(&config.uri).await {
            Ok(client) => {
                if let Err(e) = client.list_database_names().await {
                    project_error!("Failed to connect to MongoDB: {}", e);
                    process::exit(1);
                }
                *GLOBAL_PRIMARY_MONGO.write().await = Some(Arc::new(client));
                project_info!("Primary MongoDB connection initialized");
            },
            Err(e) => {
                project_error!("Failed to create initialize primary MongoDB: {}", e);
                process::exit(1);
            },
        }
    }
}

/// 初始化所有 MongoDB 连接
pub async fn init_mongo_pools() {
    if let Some(mongo_instances_config) =
        get_config::<OptionalConfigs<MongoInstancesConfig>>().await
    {
        if let Some(mongo_instances) = &mongo_instances_config.configs {
            let _ = init_mongo_pool(Some(mongo_instances.clone())).await;
        }
    }
}

pub async fn init_mongo_pool(
    mongo_instances_config: Option<Vec<MongoInstancesConfig>>,
) -> Result<(), String> {
    if let Some(mongo_instances) = mongo_instances_config {
        for mongo_instance in mongo_instances {
            init_mongo_connection(&mongo_instance.name, &mongo_instance.mongo).await?;
        }
    }
    Ok(())
}

async fn init_mongo_connection(name: &str, config: &MongoConfig) -> Result<(), String> {
    match Client::with_uri_str(&config.uri).await {
        Ok(client) => {
            if let Err(e) = client.list_database_names().await {
                let error_msg = format!("Failed to connect to MongoDB '{}': {}", name, e);
                project_error!("{}", error_msg);
                return Err(error_msg);
            }
            GLOBAL_MONGO_POOL
                .write()
                .await
                .insert(name.to_string(), Arc::new(client));
            project_info!("MongoDB '{}' initialized", name);
            Ok(())
        },
        Err(e) => {
            let error_msg = format!("Failed to initialize MongoDB '{}': {}", name, e);
            project_error!("{}", error_msg);
            Err(error_msg)
        },
    }
}

pub async fn get_primary_mongo() -> Option<Arc<Client>> {
    GLOBAL_PRIMARY_MONGO.read().await.clone()
}

pub async fn get_mongo_pool_connection(name: &str) -> Option<Arc<Client>> {
    GLOBAL_MONGO_POOL.read().await.get(name).cloned()
}

pub async fn add_or_update_mongo_pool(name: &str, config: &MongoConfig) -> Result<(), String> {
    init_mongo_connection(name, config).await
}

pub async fn remove_mongo_pool(name: &str) -> Result<(), String> {
    let mut mongo_pool = GLOBAL_MONGO_POOL.write().await;
    mongo_pool
        .remove(name)
        .ok_or_else(|| format!("MongoDB connection '{}' not found", name))?;
    project_info!("MongoDB connection '{}' removed", name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::initialize_config;
    use log::LevelFilter;
    use mongodb::bson::{doc, Document};
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

    async fn test_mongo_operations(client: &Client) -> Result<(), String> {
        let db = client.database("test");
        let collection = db.collection::<Document>("test_collection");

        // 插入测试文档
        let doc = doc! {
            "test_key": "test_value",
            "number": 42
        };

        collection
            .insert_one(doc.clone())
            .await
            .map_err(|e| format!("Failed to insert document: {}", e))?;

        // 查询文档
        let result = collection
            .find_one(doc! { "test_key": "test_value" })
            .await
            .map_err(|e| format!("Failed to find document: {}", e))?;

        assert!(result.is_some(), "Document not found");

        // 删除文档
        collection
            .delete_one(doc! { "test_key": "test_value" })
            .await
            .map_err(|e| format!("Failed to delete document: {}", e))?;

        Ok(())
    }

    #[tokio::test]
    async fn test_primary_mongo_connection() {
        setup_logger();
        init().await;

        init_primary_mongo().await;

        let client = get_primary_mongo().await;
        assert!(
            client.is_some(),
            "Primary MongoDB connection does not exist"
        );

        if let Some(client) = client {
            let result = test_mongo_operations(&client).await;
            assert!(
                result.is_ok(),
                "MongoDB operations test failed: {:?}",
                result.err()
            );
        }
    }

    #[tokio::test]
    async fn test_mongo_pool_operations() {
        setup_logger();
        init().await;

        let test_config = MongoInstancesConfig {
            name: "test_mongo".to_string(),
            mongo: MongoConfig {
                uri: "mongodb://localhost:27017".to_string(),
            },
        };

        let result = init_mongo_pool(Some(vec![test_config.clone()])).await;
        assert!(
            result.is_ok(),
            "Failed to initialize MongoDB pool: {:?}",
            result.err()
        );

        // 测试连接池连接
        let pool_connection = get_mongo_pool_connection("test_mongo").await;
        assert!(pool_connection.is_some(), "Pool connection not found");

        if let Some(client) = pool_connection {
            let result = test_mongo_operations(&client).await;
            assert!(
                result.is_ok(),
                "MongoDB pool operations test failed: {:?}",
                result.err()
            );
        }

        // 测试添加新连接
        let add_result = add_or_update_mongo_pool("test_new", &test_config.mongo).await;
        assert!(add_result.is_ok(), "Failed to add MongoDB connection");

        // 测试移除连接
        let remove_result = remove_mongo_pool("test_new").await;
        assert!(remove_result.is_ok(), "Failed to remove MongoDB connection");

        let connection_after_removal = get_mongo_pool_connection("test_new").await;
        assert!(
            connection_after_removal.is_none(),
            "MongoDB connection still exists after removal"
        );
    }
}
