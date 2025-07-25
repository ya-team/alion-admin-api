#![allow(dead_code)]

/**
 * 数据库初始化模块
 * 
 * 本模块负责初始化和管理数据库连接，包括：
 * - 初始化主数据库连接
 * - 初始化数据库连接池
 * - 管理多数据库实例
 * - 提供数据库连接操作功能
 */

use std::{sync::Arc, time::Duration};
use std::error::Error;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use server_config::{DatabaseConfig, DatabasesInstancesConfig, OptionalConfigs};
use server_global::global::{get_config, GLOBAL_DB_POOL, GLOBAL_PRIMARY_DB};

use crate::{project_error, project_info};

/**
 * 初始化主数据库连接
 * 
 * 创建并初始化主数据库连接，同时将其添加到连接池中。
 * 
 * # 返回
 * - 成功：返回数据库连接实例
 * - 失败：返回错误信息
 * 
 * # 处理流程
 * 1. 读取数据库配置
 * 2. 构建连接选项
 * 3. 建立数据库连接
 * 4. 将连接添加到全局连接池
 */
pub async fn init_primary_connection() -> Result<DatabaseConnection, Box<dyn Error>> {
    let db_config = get_config::<DatabaseConfig>().await
        .ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Database config not found")))?;
    
    let opt = build_connect_options(&db_config);
    match Database::connect(opt).await {
        Ok(db) => {
            let db = Arc::new(db);
            *GLOBAL_PRIMARY_DB.write().await = Some(db.clone());
            GLOBAL_DB_POOL.write().await.insert("default".to_string(), db.clone());
            project_info!("Primary database connection initialized");
            Ok((*db).clone())
        },
        Err(e) => {
            project_error!("Failed to connect to primary database: {}", e);
            Err(Box::new(e))
        },
    }
}

/**
 * 初始化多数据库连接
 * 
 * 从配置中读取所有数据库实例配置，
 * 并为每个实例创建数据库连接。
 */
pub async fn init_db_pools() {
    if let Some(databases_instances_config) =
        get_config::<OptionalConfigs<DatabasesInstancesConfig>>().await
    {
        if let Some(databases_instances) = &databases_instances_config.configs {
            let _ = init_db_pool_connections(Some(databases_instances.clone())).await;
        }
    }
}

/**
 * 初始化数据库连接池
 * 
 * # 参数
 * - databases_config: 数据库实例配置列表
 * 
 * # 返回
 * - 成功：返回Ok(())
 * - 失败：返回错误信息
 */
pub async fn init_db_pool_connections(
    databases_config: Option<Vec<DatabasesInstancesConfig>>,
) -> Result<(), String> {
    if let Some(dbs) = databases_config {
        for db_config in dbs {
            init_db_connection(&db_config.name, &db_config.database).await?;
        }
    }
    Ok(())
}

/**
 * 初始化单个数据库连接
 * 
 * # 参数
 * - name: 数据库实例名称
 * - db_config: 数据库配置信息
 * 
 * # 返回
 * - 成功：返回Ok(())
 * - 失败：返回错误信息
 */
async fn init_db_connection(name: &str, db_config: &DatabaseConfig) -> Result<(), String> {
    let opt = build_connect_options(db_config);
    match Database::connect(opt).await {
        Ok(db) => {
            GLOBAL_DB_POOL
                .write()
                .await
                .insert(name.to_string(), Arc::new(db));
            project_info!("Database '{}' initialized", name);
            Ok(())
        },
        Err(e) => {
            let error_msg = format!("Failed to connect to database '{}': {}", name, e);
            project_error!("{}", error_msg);
            Err(error_msg)
        },
    }
}

/**
 * 构建数据库连接选项
 * 
 * # 参数
 * - db_config: 数据库配置信息
 * 
 * # 返回
 * 返回配置好的数据库连接选项，包括：
 * - 最大连接数
 * - 最小空闲连接数
 * - 连接超时时间
 * - 空闲超时时间
 * - 最大生命周期
 */
pub fn build_connect_options(db_config: &DatabaseConfig) -> ConnectOptions {
    let mut opt = ConnectOptions::new(db_config.url.clone());
    opt.max_connections(db_config.max_connections)
        .min_connections(db_config.min_idle.unwrap_or(5))
        .connect_timeout(Duration::from_secs(db_config.connect_timeout.unwrap_or(15)))
        .idle_timeout(Duration::from_secs(db_config.idle_timeout.unwrap_or(600)))
        .max_lifetime(Duration::from_secs(db_config.max_lifetime.unwrap_or(3600)));

    opt
}

/**
 * 获取主数据库连接
 * 
 * # 返回
 * - 成功：返回主数据库连接实例
 * - 失败：返回None
 */
pub async fn get_primary_db_connection() -> Option<Arc<DatabaseConnection>> {
    GLOBAL_PRIMARY_DB.read().await.clone()
}

/**
 * 获取命名的数据库连接
 * 
 * # 参数
 * - name: 数据库实例名称
 * 
 * # 返回
 * - 成功：返回对应的数据库连接实例
 * - 失败：返回None
 */
pub async fn get_db_pool_connection(name: &str) -> Option<Arc<DatabaseConnection>> {
    GLOBAL_DB_POOL.read().await.get(name).cloned()
}

/**
 * 添加或更新数据库连接
 * 
 * # 参数
 * - name: 数据库实例名称
 * - db_config: 数据库配置信息
 * 
 * # 返回
 * - 成功：返回Ok(())
 * - 失败：返回错误信息
 */
pub async fn add_or_update_db_pool_connection(
    name: &str,
    db_config: &DatabaseConfig,
) -> Result<(), String> {
    init_db_connection(name, db_config).await
}

/**
 * 移除数据库连接
 * 
 * # 参数
 * - name: 数据库实例名称
 * 
 * # 返回
 * - 成功：返回Ok(())
 * - 失败：返回错误信息
 */
pub async fn remove_db_pool_connection(name: &str) -> Result<(), String> {
    let mut db_pool = GLOBAL_DB_POOL.write().await;
    db_pool
        .remove(name)
        .ok_or_else(|| "Connection not found".to_string())?;
    project_info!("Database connection '{}' removed", name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::LevelFilter;
    use simple_logger::SimpleLogger;
    use tokio::sync::Mutex;
    use crate::initialize_config;
    use server_config::Config;

    fn setup_logger() {
        let _ = SimpleLogger::new().with_level(LevelFilter::Info).init();
    }

    static INITIALIZED: Mutex<Option<Arc<()>>> = Mutex::const_new(None);

    async fn init() {
        let mut initialized = INITIALIZED.lock().await;
        if initialized.is_none() {
            initialize_config("../resources/application.yaml").await;
            *initialized = Some(Arc::new(()));
        }
    }

    #[tokio::test]
    async fn test_primary_connection_persistence() {
        setup_logger();
        init().await;

        let result = init_primary_connection().await;
        assert!(result.is_ok(), "Failed to initialize primary connection: {:?}", result.err());

        let connection = get_primary_db_connection().await;
        assert!(
            connection.is_some(),
            "Primary database connection does not exist"
        );
    }

    #[tokio::test]
    async fn test_db_pool_connection() {
        setup_logger();
        init().await;

        let config = get_config::<Config>().await.unwrap().as_ref().clone();
        let result = init_db_pool_connections(config.database_instances).await;
        assert!(
            result.is_ok(),
            "Failed to initialize db_pool connections: {:?}",
            result.err()
        );

        let db_config = DatabaseConfig {
            url: "postgres://postgres:postgres@localhost:5432/test_db".to_string(),
            max_connections: 10,
            min_idle: Some(1),
            connect_timeout: Some(30),
            idle_timeout: Some(600),
            max_lifetime: Some(3600),
        };

        let add_result = add_or_update_db_pool_connection("test_connection", &db_config).await;
        assert!(add_result.is_ok(), "Failed to add database connection");

        let connection = get_db_pool_connection("test_connection").await;
        assert!(connection.is_some(), "Database connection does not exist");

        let remove_result = remove_db_pool_connection("test_connection").await;
        assert!(remove_result.is_ok(), "Failed to remove database connection");

        let connection = get_db_pool_connection("test_connection").await;
        assert!(connection.is_none(), "Database connection still exists");
    }
}
